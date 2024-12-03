#include <string>

class ParserFSA {
 public:
  ParserFSA(bool useDoDontLogic);

  int feedSymbol(char symbol);

 private:
  enum class State {
    INITIAL,
    M,
    U,
    L,
    BRACKET,
    NUMBER_1,
    COMMA,
    NUMBER_2,
    D,
    O,
    DO_BRACKET,
    N,
    APOSTROPHE,
    T,
    DONT_BRACKET,
  };

  int reset(char symbol);
  int handleInitial(char symbol);
  int handleM(char symbol);
  int handleU(char symbol);
  int handleL(char symbol);
  int handleBracket(char symbol);
  int handleNumber1(char symbol);
  int handleComma(char symbol);
  int handleNumber2(char symbol);
  int handleD(char symbol);
  int handleO(char symbol);
  int handleN(char symbol);
  int handleApostrophe(char symbol);
  int handleT(char symbol);
  int handleDoBracket(char symbol);
  int handleDontBracket(char symbol);

  State currentState_;
  std::string currentNumber1_;
  std::string currentNumber2_;

  bool useDoDont_;
  bool multEnabled_;
};
