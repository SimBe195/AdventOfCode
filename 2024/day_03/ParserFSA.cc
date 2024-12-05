#include "ParserFSA.hh"

#include <cctype>
#include <string>

ParserFSA::ParserFSA(bool enableDoDontLogic)
    : useDoDont_(enableDoDontLogic),
      multEnabled_(true),
      currentState_(State::INITIAL) {}

int ParserFSA::reset(char symbol) {
  currentNumber1_.clear();
  currentNumber2_.clear();
  currentState_ = State::INITIAL;
  return handleInitial(symbol);
}

int ParserFSA::handleInitial(char symbol) {
  if (multEnabled_ and symbol == 'm') {
    currentState_ = State::M;
  } else if (useDoDont_ and symbol == 'd') {
    currentState_ = State::D;
  }
  return 0;
}

int ParserFSA::handleM(char symbol) {
  if (symbol == 'u') {
    currentState_ = State::U;
  } else {
    reset(symbol);
  }
  return 0;
}

int ParserFSA::handleU(char symbol) {
  if (symbol == 'l') {
    currentState_ = State::L;
  } else {
    reset(symbol);
  }
  return 0;
}

int ParserFSA::handleL(char symbol) {
  if (symbol == '(') {
    currentState_ = State::BRACKET;
  } else {
    reset(symbol);
  }
  return 0;
}

int ParserFSA::handleBracket(char symbol) {
  if (isdigit(symbol)) {
    currentState_ = State::NUMBER_1;
    handleNumber1(symbol);
  } else {
    reset(symbol);
  }
  return 0;
}

int ParserFSA::handleNumber1(char symbol) {
  if (isdigit(symbol)) {
    currentNumber1_ += symbol;
  } else if (symbol == ',') {
    currentState_ = State::COMMA;
  } else {
    reset(symbol);
  }
  return 0;
}

int ParserFSA::handleComma(char symbol) {
  if (isdigit(symbol)) {
    currentState_ = State::NUMBER_2;
    handleNumber2(symbol);
  } else {
    reset(symbol);
  }
  return 0;
}

int ParserFSA::handleNumber2(char symbol) {
  int result = 0;
  if (isdigit(symbol)) {
    currentNumber2_ += symbol;
  } else if (symbol == ')') {
    result = std::stoi(currentNumber1_) * std::stoi(currentNumber2_);
    reset(symbol);
  } else {
    reset(symbol);
  }
  return result;
}

int ParserFSA::handleD(char symbol) {
  if (symbol == 'o') {
    currentState_ = State::O;
  } else {
    reset(symbol);
  }
  return 0;
}

int ParserFSA::handleO(char symbol) {
  if (symbol == '(') {
    currentState_ = State::DO_BRACKET;
  } else if (symbol == 'n') {
    currentState_ = State::N;
  } else {
    reset(symbol);
  }
  return 0;
}

int ParserFSA::handleDoBracket(char symbol) {
  if (symbol == ')') {
    multEnabled_ = true;
  }
  reset(symbol);
  return 0;
}

int ParserFSA::handleN(char symbol) {
  if (symbol == '\'') {
    currentState_ = State::APOSTROPHE;
  } else {
    reset(symbol);
  }
  return 0;
}

int ParserFSA::handleApostrophe(char symbol) {
  if (symbol == 't') {
    currentState_ = State::T;
  } else {
    reset(symbol);
  }
  return 0;
}

int ParserFSA::handleT(char symbol) {
  if (symbol == '(') {
    currentState_ = State::DONT_BRACKET;
  } else {
    reset(symbol);
  }
  return 0;
}

int ParserFSA::handleDontBracket(char symbol) {
  if (symbol == ')') {
    multEnabled_ = false;
  }
  reset(symbol);
  return 0;
}

int ParserFSA::feedSymbol(char symbol) {
  switch (currentState_) {
    case State::INITIAL:
      return handleInitial(symbol);
    case State::M:
      return handleM(symbol);
    case State::U:
      return handleU(symbol);
    case State::L:
      return handleL(symbol);
    case State::BRACKET:
      return handleBracket(symbol);
    case State::NUMBER_1:
      return handleNumber1(symbol);
    case State::COMMA:
      return handleComma(symbol);
    case State::NUMBER_2:
      return handleNumber2(symbol);
    case State::D:
      return handleD(symbol);
    case State::O:
      return handleO(symbol);
    case State::DO_BRACKET:
      return handleDoBracket(symbol);
    case State::N:
      return handleN(symbol);
    case State::APOSTROPHE:
      return handleApostrophe(symbol);
    case State::T:
      return handleT(symbol);
    case State::DONT_BRACKET:
      return handleDontBracket(symbol);
  }
}
