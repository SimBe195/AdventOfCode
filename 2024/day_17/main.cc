#include <climits>
#include <cstddef>
#include <cstdint>
#include <iostream>

#include "challenge2.hh"
#include "program.hh"

int main() {
  State testState;
  State testState2;
  State state;
  Program testProgram;
  Program testProgram2;
  Program program;

  parseFile("testinput.txt", testState, testProgram);
  parseFile("testinput2.txt", testState2, testProgram2);
  parseFile("input.txt", state, program);

  testProgram.run(testState);
  std::cout << "Challenge 1 test: ";
  testProgram.printOutput();

  program.run(state);
  std::cout << "Challenge 1: ";
  program.printOutput();

  for (size_t aValue = 0ul; aValue < SIZE_MAX; ++aValue) {
    if (testProgram2.isSelfReferential({aValue, 0ul, 0ul})) {
      std::cout << "Challenge 2 test: " << aValue << std::endl;
      break;
    }
  }

  size_t aValue = constructA();
  if (program.isSelfReferential({aValue, 0ul, 0ul})) {
    std::cout << "Challenge 2: " << aValue << std::endl;
  }

  return 0;
}
