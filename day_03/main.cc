#include <fstream>
#include <iostream>

#include "ParserFSA.hh"

// --- Challenge 1 ---

void runChallenge1(const std::string& filename) {
  ParserFSA parser(false);
  std::ifstream file(filename);
  int sum = 0;
  char nextChar;
  while (file >> nextChar) {
    sum += parser.feedSymbol(nextChar);
  }
  std::cout << "Sum for challenge 1 is " << sum << std::endl;
  file.close();
}

// --- Challenge 2 ---

void runChallenge2(const std::string& filename) {
  ParserFSA parser(true);
  std::ifstream file(filename);
  int sum = 0;
  char nextChar;
  while (file >> nextChar) {
    sum += parser.feedSymbol(nextChar);
  }
  std::cout << "Sum for challenge 2 is " << sum << std::endl;
  file.close();
}

int main(int argc, char* argv[]) {
  runChallenge1("testinput1.txt");
  runChallenge1("input.txt");
  runChallenge2("testinput2.txt");
  runChallenge2("input.txt");

  return 0;
}
