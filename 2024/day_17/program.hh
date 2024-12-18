#ifndef PROGRAM_HH
#define PROGRAM_HH

#include <string>
#include <vector>

struct State {
  size_t registerA;
  size_t registerB;
  size_t registerC;
};

class Program : public std::vector<int> {
 public:
  bool runStep(State& state);
  void run(State state);

  void printOutput() const;

  bool isSelfReferential(State state);

  void reset();

 private:
  int inferComboOperandValue(int operand, const State& state) const;
  size_t currentInstructionIndex_ = 0ul;

  std::vector<int> output_;
};

void parseFile(const std::string& filename, State& state, Program& program);

#endif  // !PROGRAM_HH
