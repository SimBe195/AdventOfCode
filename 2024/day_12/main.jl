function parse_file(filename::String)::Matrix{Char}
  lines = readlines(filename)
  num_rows = length(lines)
  num_cols = length(first(lines))

  grid = Matrix{Char}(undef, num_rows, num_cols)

  for (row, line) in enumerate(lines)
    for (col, char) in enumerate(line)
      grid[row, col] = char
    end
  end

  grid
end

function get_regions(grid::Matrix{Char})::Vector{Set{Tuple{Int,Int}}}
  regions = Vector{Set{Tuple{Int,Int}}}()

  for row in axes(grid, 1), col in axes(grid, 2)
    merged_region = Set{Tuple{Int,Int}}()
    push!(merged_region, (row, col))
    new_regions = Vector{Set{Tuple{Int,Int}}}()
    for region in regions
      if (row > 1 && grid[row-1, col] == grid[row, col] && (row - 1, col) in region) || (col > 1 && grid[row, col-1] == grid[row, col] && (row, col - 1) in region)
        union!(merged_region, region)
      else
        push!(new_regions, region)
      end
    end
    push!(new_regions, merged_region)
    regions = new_regions
  end

  regions
end

function get_perimeter(grid::Matrix{Char}, row::Int, col::Int)::Int
  result = 0
  for (delta_row, delta_col) in [(-1, 0), (0, 1), (1, 0), (0, -1)]
    if row + delta_row < 1 || row + delta_row > size(grid, 1)
      result += 1
    elseif col + delta_col < 1 || col + delta_col > size(grid, 1)
      result += 1
    elseif grid[row, col] != grid[row+delta_row, col+delta_col]
      result += 1
    end
  end
  result
end

function get_price(grid::Matrix{Char})::Int
  regions = get_regions(grid)
  total_price = 0
  for region in regions
    region_perimeter = 0
    for (row, col) in region
      region_perimeter += get_perimeter(grid, row, col)
    end
    total_price += length(region) * region_perimeter
  end

  total_price
end

function get_num_corners(grid::Matrix{Char}, region::Set{Tuple{Int,Int}})::Int
  num_corners = 0
  for (row, col) in region
    neighbors_eq = Matrix{Bool}(undef, 3, 3)
    for delta_row in [-1, 0, 1], delta_col in [-1, 0, 1]
      neighbors_eq[2+delta_row, 2+delta_col] = (1 <= row + delta_row <= size(grid, 1) && 1 <= col + delta_col <= size(grid, 2) && grid[row, col] == grid[row+delta_row, col+delta_col])
    end

    # top left convex
    if !neighbors_eq[1, 2] && !neighbors_eq[2, 1]
      num_corners += 1
    end

    # top left concave
    if !neighbors_eq[1, 1] && neighbors_eq[1, 2] && neighbors_eq[2, 1]
      num_corners += 1
    end

    # top right convex
    if !neighbors_eq[1, 2] && !neighbors_eq[2, 3]
      num_corners += 1
    end

    # top right concave
    if neighbors_eq[1, 2] && !neighbors_eq[1, 3] && neighbors_eq[2, 3]
      num_corners += 1
    end

    # bottom left convex
    if !neighbors_eq[2, 1] && !neighbors_eq[3, 2]
      num_corners += 1
    end

    # bottom left concave
    if neighbors_eq[2, 1] && !neighbors_eq[3, 1] && neighbors_eq[3, 2]
      num_corners += 1
    end

    # bottom right convex
    if !neighbors_eq[2, 3] && !neighbors_eq[3, 2]
      num_corners += 1
    end

    # bottom right concave
    if neighbors_eq[2, 3] && neighbors_eq[3, 2] && !neighbors_eq[3, 3]
      num_corners += 1
    end
  end

  num_corners
end

function get_price_for_sides(grid::Matrix{Char})::Int
  regions = get_regions(grid)
  total_price = 0
  for region in regions
    region_sides = get_num_corners(grid, region)
    total_price += length(region) * region_sides
  end

  total_price
end


function main()
  testgrid = parse_file("testinput.txt")
  grid = parse_file("input.txt")
  println("Challenge 1 test: ", get_price(testgrid))
  println("Challenge 1: ", get_price(grid))

  println("Challenge 2 test: ", get_price_for_sides(testgrid))
  println("Challenge 2: ", get_price_for_sides(grid))
end

main()
