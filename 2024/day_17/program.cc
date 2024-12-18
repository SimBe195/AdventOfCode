#include "program.hh"

#include <fstream>
#include <iostream>
#include <iterator>

bool Program::runStep(State& state) {
  if (currentInstructionIndex_ > size() - 2ul) {
    return false;
  }

  int opCode = at(currentInstructionIndex_);
  int operand = at(currentInstructionIndex_ + 1ul);

  switch (opCode) {
    case 0:
      state.registerA >>= inferComboOperandValue(operand, state);
      currentInstructionIndex_ += 2ul;
      break;
    case 1:
      state.registerB ^= operand;
      currentInstructionIndex_ += 2ul;
      break;
    case 2:
      state.registerB = inferComboOperandValue(operand, state) & 0b111;
      currentInstructionIndex_ += 2ul;
      break;
    case 3:
      if (state.registerA != 0ul) {
        currentInstructionIndex_ = operand;
      } else {
        currentInstructionIndex_ += 2ul;
      }
      break;
    case 4:
      state.registerB ^= state.registerC;
      currentInstructionIndex_ += 2ul;
      break;
    case 5:
      output_.push_back(inferComboOperandValue(operand, state) & 0b111);
      currentInstructionIndex_ += 2ul;
      break;
    case 6:
      state.registerB =
          state.registerA >> inferComboOperandValue(operand, state);
      currentInstructionIndex_ += 2ul;
      break;
    case 7:
      state.registerC =
          state.registerA >> inferComboOperandValue(operand, state);
      currentInstructionIndex_ += 2ul;
      break;
  }

  return true;
}

void Program::run(State state) { while (runStep(state)); }

void Program::printOutput() const {
  std::copy(output_.begin(), output_.end(),
            std::ostream_iterator<int>(std::cout, ","));
  std::cout << std::endl;
}

void Program::reset() {
  currentInstructionIndex_ = 0ul;
  output_.clear();
}

bool Program::isSelfReferential(State state) {
  reset();
  run(state);

  if (size() == output_.size()) {
    for (size_t idx = 0ul; idx < size(); ++idx) {
      if (at(idx) != output_.at(idx)) {
        return false;
      }
    }
    return true;
  }
  return false;
}

int Program::inferComboOperandValue(int operand, const State& state) const {
  switch (operand) {
    case 4:
      return state.registerA;
    case 5:
      return state.registerB;
    case 6:
      return state.registerC;
    default:
      return operand;
  }
}

void parseFile(const std::string& filename, State& state, Program& program) {
  std::ifstream file(filename);

  std::string data;
  file >> data;  // "Register"
  file >> data;  // "A:"
  file >> state.registerA;

  file >> data;  // "Register"
  file >> data;  // "B:"
  file >> state.registerB;

  file >> data;  // "Register"
  file >> data;  // "C:"
  file >> state.registerC;

  file >> data;  // "Program:"
  file >> data;  // Single-digit instructions separated by commas
  for (size_t idx = 0ul; idx < data.length(); idx += 2ul) {
    program.push_back(data[idx] - '0');  // Convert numerical char to integer
  }
}
