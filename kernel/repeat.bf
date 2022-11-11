#define REP0(X)
#define REP1(X) X
#define REP2(X) REP1(X) X
#define REP3(X) REP2(X) X
#define REP4(X) REP3(X) X
#define REP5(X) REP4(X) X
#define REP6(X) REP5(X) X
#define REP7(X) REP6(X) X
#define REP8(X) REP7(X) X
#define REP9(X) REP8(X) X
#define REPA(X) REP9(X) X
#define REPB(X) REPA(X) X
#define REPC(X) REPB(X) X
#define REPD(X) REPC(X) X
#define REPE(X) REPD(X) X
#define REPF(X) REPE(X) X


#define REPEATHEX(ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE, TEN, ELEVEN, TWELVE, THIRTEEN, FOURTEEN, FIFTEEN, SIXTEEN, X) \
  REP##ONE(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(X)))))))))))))))) \
  REP##TWO(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(X))))))))))))))) \
  REP##THREE(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(X)))))))))))))) \
  REP##FOUR(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(X))))))))))))) \
  REP##FIVE(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(X)))))))))))) \
  REP##SIX(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(X))))))))))) \
  REP##SEVEN(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(X)))))))))) \
  REP##EIGHT(REPF(REPF(REPF(REPF(REPF(REPF(REPF(REPF(X))))))))) \
  REP##NINE(REPF(REPF(REPF(REPF(REPF(REPF(REPF(X)))))))) \
  REP##TEN(REPF(REPF(REPF(REPF(REPF(REPF(X))))))) \
  REP##ELEVEN(REPF(REPF(REPF(REPF(REPF(X)))))) \
  REP##TWELVE(REPF(REPF(REPF(REPF(X))))) \
  REP##THIRTEEN(REPF(REPF(REPF(X)))) \
  REP##FOURTEEN(REPF(REPF(X))) \
  REP##FIFTEEN(REPF(X)) \
  REP##SIXTEEN(X) 