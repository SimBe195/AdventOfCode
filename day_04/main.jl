# Input parsing
function create_char_matrix(filename::String)
  lines = readlines(filename)

  num_rows = length(lines)
  num_cols = length(first(lines))

  matrix = Matrix{Char}(undef, num_rows, num_cols)
  for (row, line) in enumerate(lines)
    for (col, char) in enumerate(line)
      matrix[row, col] = char
    end
  end

  return matrix
end

function search_pattern_in_matrix(matrix::Matrix{Char}, pattern::String, begin_row::Int, begin_col::Int, delta_row::Int, delta_col::Int)::Int
  # Bounds checks
  if !(begin_row in axes(matrix, 1))
    return 0
  end
  if !(begin_col in axes(matrix, 2))
    return 0
  end
  if !(begin_row + delta_row * (length(pattern) - 1) in axes(matrix, 1))
    return 0
  end

  if !(begin_col + delta_col * (length(pattern) - 1) in axes(matrix, 2))
    return 0
  end

  # Check pattern
  for (i, char) in enumerate(pattern)
    if matrix[begin_row+(i-1)*delta_row, begin_col+(i-1)*delta_col] != char
      return 0
    end
  end

  # No early exit -> pattern found
  return 1

end

# --- Challenge 1 ---

function count_xmas(matrix::Matrix{Char})
  # Search the string "MAS" in all 8 directions whenever encountering an 'X'
  sum = 0
  for row in axes(matrix, 1), col in axes(matrix, 2)
    if matrix[row, col] == 'X'
      for (delta_row, delta_col) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
        sum += search_pattern_in_matrix(matrix, "XMAS", row, col, delta_row, delta_col)
      end
    end
  end

  return sum
end

# --- Challenge 2 ---

function count_cross_mas(matrix::Matrix{Char})
  sum = 0
  for row = 2:size(matrix, 1)-1, col = 2:size(matrix, 2)-1
    if matrix[row, col] == 'A'
      # Summing the values of forward and backward direction of a diagonal
      # and multiplying the sums for both diagonals yields 1 iff "MAS" can be found
      # in any direction on both diagonals
      diag_1 = search_pattern_in_matrix(matrix, "MAS", row - 1, col - 1, 1, 1) + search_pattern_in_matrix(matrix, "MAS", row + 1, col + 1, -1, -1)
      diag_2 = search_pattern_in_matrix(matrix, "MAS", row - 1, col + 1, 1, -1) + search_pattern_in_matrix(matrix, "MAS", row + 1, col - 1, -1, 1)
      sum += diag_1 * diag_2
    end
  end

  return sum
end


function main()
  test_matrix = create_char_matrix("testinput.txt")
  matrix = create_char_matrix("input.txt")

  println(count_xmas(test_matrix))
  println(count_xmas(matrix))

  println(count_cross_mas(test_matrix))
  println(count_cross_mas(matrix))
end

main()
