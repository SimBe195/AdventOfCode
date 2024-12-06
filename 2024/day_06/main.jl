function create_map(filename::String)::Tuple{Matrix{Bool},Tuple{Int,Int}}
    lines = readlines(filename)

    num_rows = length(lines)
    num_cols = length(first(lines))

    guard_pos = (0, 0)

    matrix = falses(num_rows, num_cols)
    for (row, line) in enumerate(lines)
        for (col, char) in enumerate(line)
            if char == '#'
                matrix[row, col] = true
            elseif char == '^'
                guard_pos = (row, col)
            else
                matrix[row, col] = false
            end
        end
    end

    return matrix, guard_pos
end

function bounds_check(map::Matrix{Bool}, cell::Tuple{Int,Int})::Bool
    row, col = cell
    return row in (1, size(map, 1)) || col in (1, size(map, 2))
end

function turn_right(curr_orientation::Tuple{Int,Int})::Tuple{Int,Int}
    if curr_orientation == (-1, 0)
        return (0, 1)
    elseif curr_orientation == (0, 1)
        return (1, 0)
    elseif curr_orientation == (1, 0)
        return (0, -1)
    else # curr_orientation == (0, -1)
        return (-1, 0)
    end
end

function next_step(map::Matrix{Bool}, curr_cell::Tuple{Int,Int}, curr_orientation::Tuple{Int,Int})::Tuple{Tuple{Int,Int},Tuple{Int,Int}}
    next_orientation = curr_orientation

    next_cell = (curr_cell[1] + next_orientation[1], curr_cell[2] + next_orientation[2])
    while map[next_cell[1], next_cell[2]]
        next_orientation = turn_right(next_orientation)
        next_cell = (curr_cell[1] + next_orientation[1], curr_cell[2] + next_orientation[2])
    end

    return next_cell, next_orientation
end

function trace_path(map::Matrix{Bool}, start_cell::Tuple{Int,Int})::Set{Tuple{Int,Int}}
    orientation = (-1, 0)
    cell = start_cell
    visited_cells = Set{Tuple{Int,Int}}()
    push!(visited_cells, cell)
    while !bounds_check(map, cell)
        cell, orientation = next_step(map, cell, orientation)
        push!(visited_cells, cell)
    end

    return visited_cells
end

function is_path_circular(map::Matrix{Bool}, start_cell::Tuple{Int,Int})::Bool
    orientation = (-1, 0)
    cell = start_cell
    visited_cells = Set{Tuple{Tuple{Int,Int},Tuple{Int,Int}}}()
    push!(visited_cells, (cell, orientation))
    while !bounds_check(map, cell)
        cell, orientation = next_step(map, cell, orientation)
        if (cell, orientation) in visited_cells
            return true
        end
        push!(visited_cells, (cell, orientation))
    end

    return false
end

function num_circle_causing_obstructions(map::Matrix{Bool}, start_cell::Tuple{Int,Int})::Int
    possible_locations = trace_path(map, start_cell)

    result = 0
    for (row, col) in possible_locations
        map[row, col] = true
        if is_path_circular(map, start_cell)
            result += 1
        end
        map[row, col] = false
    end

    return result
end

function main()
    test_map, test_guard_pos = create_map("testinput.txt")
    println("Challenge 1 test: ", length(trace_path(test_map, test_guard_pos)))

    map, guard_pos = create_map("input.txt")
    println("Challenge 1: ", length(trace_path(map, guard_pos)))

    println("Challenge 2 test: ", num_circle_causing_obstructions(test_map, test_guard_pos))
    println("Challenge 2: ", num_circle_causing_obstructions(map, guard_pos))
end

main()
