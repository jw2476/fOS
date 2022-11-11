use inkwell::context::Context;
use inkwell::module::Linkage;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple,
};
use inkwell::{AddressSpace, IntPredicate, OptimizationLevel};
use std::collections::VecDeque;

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::values::{FunctionValue, PointerValue};
use std::fs::File;
use std::io::prelude::*;

fn create_target() -> TargetMachine {
    Target::initialize_all(&InitializationConfig::default());

    let cpu = TargetMachine::get_host_cpu_name().to_string();
    let features = TargetMachine::get_host_cpu_features().to_string();

    let triple = TargetTriple::create("aarch64-unknown-none");
    let target = Target::from_triple(&triple).unwrap();
    let target_machine = target
        .create_target_machine(
            &triple,
            &cpu,
            &features,
            OptimizationLevel::Default,
            RelocMode::Default,
            CodeModel::Default,
        )
        .unwrap();

    return target_machine;
}

fn build_add_ptr(context: &Context, builder: &Builder, amount: i32, ptr: &PointerValue) {
    let i32_type = context.i32_type();
    let i32_amount = i32_type.const_int(amount as u64, false);
    let ptr_load = builder.build_load(*ptr, "load ptr").into_pointer_value();
    // unsafe because we are calling an unsafe function, since we could index out of bounds of the calloc
    let result = unsafe { builder.build_in_bounds_gep(ptr_load, &[i32_amount], "add to pointer") };
    builder.build_store(*ptr, result);
}

fn build_add(context: &Context, builder: &Builder, amount: i32, ptr: &PointerValue) {
    let i8_type = context.i8_type();
    let i8_amount = i8_type.const_int(amount as u64, false);
    let ptr_load = builder.build_load(*ptr, "load ptr").into_pointer_value();
    let ptr_val = builder.build_load(ptr_load, "load ptr value");
    let result = builder.build_int_add(ptr_val.into_int_value(), i8_amount, "add to data ptr");
    builder.build_store(ptr_load, result);
}

struct WhileBlock<'ctx> {
    while_start: BasicBlock<'ctx>,
    while_body: BasicBlock<'ctx>,
    while_end: BasicBlock<'ctx>,
}

fn build_while_start<'ctx>(
    context: &'ctx Context,
    builder: &Builder,
    main_fn: &'ctx FunctionValue,
    ptr: &PointerValue,
    while_blocks: &mut VecDeque<WhileBlock<'ctx>>,
) {
    // create the while block
    let num_while_blocks = while_blocks.len() + 1;
    let while_block = WhileBlock {
        while_start: context.append_basic_block(
            *main_fn,
            format!("while_start {}", num_while_blocks).as_str(),
        ),
        while_body: context.append_basic_block(
            *main_fn,
            format!("while_body {}", num_while_blocks).as_str(),
        ),
        while_end: context
            .append_basic_block(*main_fn, format!("while_end {}", num_while_blocks).as_str()),
    };
    while_blocks.push_front(while_block);
    let while_block = while_blocks.front().unwrap();

    builder.build_unconditional_branch(while_block.while_start);
    builder.position_at_end(while_block.while_start);

    // compare the value at ptr with zero
    let i8_type = context.i8_type();
    let i8_zero = i8_type.const_int(0, false);
    let ptr_load = builder.build_load(*ptr, "load ptr").into_pointer_value();
    let ptr_value = builder
        .build_load(ptr_load, "load ptr value")
        .into_int_value();
    let cmp = builder.build_int_compare(
        IntPredicate::NE,
        ptr_value,
        i8_zero,
        "compare value at pointer to zero",
    );

    // jump to the while_end if the data at ptr was zero
    builder.build_conditional_branch(cmp, while_block.while_body, while_block.while_end);
    builder.position_at_end(while_block.while_body);
}

fn build_while_end<'ctx>(builder: &Builder, while_blocks: &mut VecDeque<WhileBlock<'ctx>>) {
    if let Some(while_block) = while_blocks.pop_front() {
        // Jump to start
        builder.build_unconditional_branch(while_block.while_start);
        // Set builder to insert code after the while block
        builder.position_at_end(while_block.while_end);
        Ok(())
    } else {
        Err("error: unmatched `]`".to_string())
    }
    .map_err(|e| format!("{:?}", e))
    .unwrap();
}

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("INPUT")
                .help("source bf file to compile")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .help("output filename")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let context = Context::create();
    let module = context.create_module("fOS");
    let builder = context.create_builder();

    // Create main function
    let void_type = context.void_type();
    let main_fn_type = void_type.fn_type(&[], false);
    let main_fn = module.add_function("kernel_main", main_fn_type, Some(Linkage::External));

    // Entry point block
    let entry = context.append_basic_block(main_fn, "entry");
    builder.position_at_end(entry);

    // Allocate pointer
    let i8_type = context.i8_type();
    let i8_ptr_type = i8_type.ptr_type(AddressSpace::Generic);
    let ptr = builder.build_alloca(i8_ptr_type, "data");

    // Set pointer to 0x0
    let i64_type = context.i64_type();
    let vga_buffer_addr = i64_type.const_int(0x0, false);
    builder.build_store(ptr, vga_buffer_addr);

    // Read code into string program
    let source_filename = matches.value_of("INPUT").unwrap();
    let mut f = File::open(source_filename)
        .map_err(|e| format!("{:?}", e))
        .unwrap();
    let mut program = String::new();
    f.read_to_string(&mut program).unwrap();

    let mut while_blocks: VecDeque<WhileBlock> = VecDeque::new();

    let mut magnitude = 1;

    for command in program.chars() {
        match command {
            '>' => build_add_ptr(&context, &builder, magnitude, &ptr),
            '<' => build_add_ptr(&context, &builder, -magnitude, &ptr),
            '+' => build_add(&context, &builder, magnitude, &ptr),
            '-' => build_add(&context, &builder, -magnitude, &ptr),
            '[' => build_while_start(&context, &builder, &main_fn, &ptr, &mut while_blocks),
            ']' => build_while_end(&builder, &mut while_blocks),
            '/' => magnitude *= 8,
            '\\' => magnitude /= 8,
            _ => (),
        }
    }

    // Return from kernel_main
    builder.build_return(None);

    // Write to kernel.o
    let target_machine = create_target();
    let output_file = matches.value_of("output").unwrap();
    target_machine
        .write_to_file(&module, FileType::Object, output_file.as_ref())
        .map_err(|e| format!("{:?}", e))
        .unwrap();
}
