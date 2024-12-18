#ifndef CHALLENGE2_HH
#define CHALLENGE2_HH
/*
 * Program: 2,4,1,5,7,5,1,6,4,3,5,5,0,3,3,0
 * = [bst 4], [bxl 5], [cdv 5], [bxl 6], [bxc 3], [out 5], [adv 3], [jnz 0]
 *
 * 1. bst 4 => B = A & 111
 * 2. bxl 5 => B = B xor 101 = (A & 111) xor 101
 * 3. cdv 5 => C = A >> B = A >> ((A & 111) xor 101)
 * 4. bxl 6 => B = B xor 110 = ((A & 111) xor 101) xor 110 = (A & 111) xor 011
 * 5. bxc 3 => B = B xor C = ((A & 111) xor 011) xor (A >> ((A & 111) xor 101))
 * 6. out 5 => B & 111 = ((A & 111) xor 011) xor (A >> ((A & 111) xor 101)) &
 * 111
 * 7. adv 3 => A = A >> 3
 * 8. jnz 0 => if A != 0 go to step 1.
 *
 * Overall:
 *    Until A == 0:
 *    - Output ((A & 111) xor 011) xor (A >> ((A & 111) xor 101))
 *    - A = A >> 3
 *
 * Conclusions:
 *  - A must be 46 to 48 bits since loop must be executed 16 times
 *  - After 15 loops A becomes 3 digits long and must output 0, so then A = 011
 *  - After 14 loops A is 011xxx and must output 3, so A = 011000
 *  - This can be continued, each time inferring the next 3 binary digits (there
 * may be multiple options)
 *  - To infer the next digits abc to obtain instruction s we need
 *      ((abc xor 011) xor (A|abc >> (abc xor 101))) & 111 = s
 *  - All the matches can be used to formulate a backtracking solution
 *
 */

#include <optional>
#include <vector>

const std::vector<size_t> _fullProgram = {2, 4, 1, 5, 7, 5, 1, 6,
                                          4, 3, 5, 5, 0, 3, 3, 0};
inline std::optional<size_t> constructionHelper(size_t aValue, int symbolIdx) {
  if (symbolIdx < 0) {
    return aValue;
  }

  size_t currentSymbol = _fullProgram[symbolIdx];

  for (size_t next = 0ul; next < 8ul; ++next) {
    size_t testA = (aValue << 3) | next;
    size_t printSymbol =
        (((testA & 0b111) ^ 0b011) ^ (testA >> ((testA & 0b111) ^ 0b101))) &
        0b111;
    if (printSymbol == currentSymbol) {
      auto result = constructionHelper(testA, symbolIdx - 1);
      if (result.has_value()) {
        return result;
      }
    }
  }
  return std::nullopt;
}

inline size_t constructA() {
  auto result = constructionHelper(0ul, _fullProgram.size() - 1);
  return result.value();
}

#endif  // !CHALLENGE2_HH
