function parse_file(filename::String)::Vector{Tuple{Int,Int}}
    content = readlines(filename)

    result = Vector{Tuple{Int,Int}}()
    for line in content
        (left, right) = split(line, ',')
        push!(result, (parse(Int, left) + 1, parse(Int, right) + 1))
    end
    result
end

function shortest_path_length(matrix::Matrix{Bool})::Int
    rows, cols = size(matrix)

    # Directions for moving in the matrix (up, down, left, right)
    directions = [(1, 0), (-1, 0), (0, 1), (0, -1)]

    # Queue for BFS: stores (position, path-so-far)
    queue = [((1, 1), [])]
    visited = Set([(1, 1)])

    while !isempty(queue)
        current_pos, path = popfirst!(queue)

        # If we reached the goal, return the path
        if current_pos == (rows, cols)
            return length(path)
        end

        # Explore neighbors
        for direction in directions
            neighbor = current_pos .+ direction

            # Check if the neighbor is valid and accessible
            if 1 <= neighbor[1] <= rows && 1 <= neighbor[2] <= cols &&
               matrix[neighbor...] && !(neighbor in visited)
                ext_path = cat(path, [neighbor], dims=1)
                push!(queue, (neighbor, ext_path))
                push!(visited, neighbor)
            end
        end
    end

    return -1
end

function dropped_memory_matrix(locations::Vector{Tuple{Int,Int}}, count::Int, rows::Int, cols::Int)::Matrix{Bool}
    matrix = Matrix{Bool}(undef, rows, cols)
    fill!(matrix, true)
    for i = 1:count
        matrix[locations[i]...] = false
    end
    matrix
end

function shortest_path_length_after_drop(locations::Vector{Tuple{Int,Int}}, count::Int, rows::Int, cols::Int)::Int
    matrix = dropped_memory_matrix(locations, count, rows, cols)
    shortest_path_length(matrix)
end

function find_first_blocking(locations::Vector{Tuple{Int,Int}}, rows::Int, cols::Int, range_start::Int, range_end::Int)::String
    if range_start == range_end
        return join([locations[range_start][1] - 1, locations[range_start][2] - 1], ",")
    end

    midpoint = div(range_start + range_end, 2)
    if shortest_path_length_after_drop(locations, midpoint, rows, cols) >= 0
        new_range_start = midpoint + 1
        new_range_end = range_end
    else
        new_range_start = range_start
        new_range_end = midpoint
    end

    find_first_blocking(locations, rows, cols, new_range_start, new_range_end)
end

function main()
    test_memory_locations = parse_file("testinput.txt")
    memory_locations = parse_file("input.txt")

    println("Challenge 1 test: ", shortest_path_length_after_drop(test_memory_locations, 12, 7, 7))
    println("Challenge 1: ", shortest_path_length_after_drop(memory_locations, 1024, 71, 71))

    println("Challenge 2 test: ", find_first_blocking(test_memory_locations, 7, 7, 13, length(test_memory_locations)))
    println("Challenge 2: ", find_first_blocking(memory_locations, 71, 71, 1025, length(memory_locations)))
end

main()
