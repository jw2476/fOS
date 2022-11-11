import subprocess

def copy2clip(txt):
    cmd='echo '+txt.strip()+'|xclip -selection clipboard'
    return subprocess.check_call(cmd, shell=True)

num = int(input("Number: "))
copy2clip("[-]" + "+"*num)