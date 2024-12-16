function parse_file(filename::String)::Tuple{Matrix{Bool},Tuple{Int,Int},Tuple{Int,Int}}
    content = readlines(filename)

    num_rows = length(content)
    num_cols = length(content[1])

    maze = Matrix{Bool}(undef, num_rows, num_cols)

    start_pos = (0, 0)
    end_pos = (0, 0)

    for row in 1:num_rows, col in 1:num_cols
        if content[row][col] == '.'
            maze[row, col] = true
        elseif content[row][col] == '#'
            maze[row, col] = false
        elseif content[row][col] == 'S'
            maze[row, col] = true
            start_pos = (row, col)
        elseif content[row][col] == 'E'
            maze[row, col] = true
            end_pos = (row, col)
        end
    end

    maze, start_pos, end_pos
end

function dijkstra_search(maze::Matrix{Bool}, start_pos::Tuple{Int,Int}, end_pos::Tuple{Int,Int})::Int
    unvisited = Set([(row, col, direction) for row in axes(maze, 1) for col in axes(maze, 2) for direction in ['n', 's', 'e', 'w'] if maze[row, col]])
    distances = Dict(location => typemax(Int) for location in unvisited)
    distances[(start_pos[1], start_pos[2], 'e')] = 0

    while !isempty(unvisited)
        best_location = (0, 0, 'n')
        best_distance = typemax(Int)
        for location in unvisited
            if distances[location] < best_distance
                best_location = location
                best_distance = distances[location]
            end
        end
        pop!(unvisited, best_location)

        if (best_location[1], best_location[2]) == end_pos
            return best_distance
        end

        row, col, direction = best_location
        neighbors = []
        if direction == 'n'
            neighbors = [(row, col, 'e', 1000), (row, col, 'w', 1000)]
            if row > 1 && maze[row-1, col]
                push!(neighbors, (row - 1, col, 'n', 1))
            end
        elseif direction == 's'
            neighbors = [(row, col, 'e', 1000), (row, col, 'w', 1000)]
            if row < size(maze, 1) - 1 && maze[row+1, col]
                push!(neighbors, (row + 1, col, 's', 1))
            end
        elseif direction == 'e'
            neighbors = [(row, col, 'n', 1000), (row, col, 's', 1000)]
            if col < size(maze, 2) - 1 && maze[row, col+1]
                push!(neighbors, (row, col + 1, 'e', 1))
            end
        elseif direction == 'w'
            neighbors = [(row, col, 'n', 1000), (row, col, 's', 1000)]
            if col > 1 && maze[row, col-1]
                push!(neighbors, (row, col - 1, 'w', 1))
            end
        end

        for (neighbor_row, neighbor_col, neighbor_direction, transition_cost) in neighbors
            if best_distance + transition_cost < distances[(neighbor_row, neighbor_col, neighbor_direction)]
                distances[(neighbor_row, neighbor_col, neighbor_direction)] = best_distance + transition_cost
            end
        end
    end

    return typemax(Int)
end

function dijkstra_search_best_path_tiles(maze::Matrix{Bool}, start_pos::Tuple{Int,Int}, end_pos::Tuple{Int,Int})::Int
    unvisited = Set([(row, col, direction) for row in axes(maze, 1) for col in axes(maze, 2) for direction in ['n', 's', 'e', 'w'] if maze[row, col]])
    distances = Dict(location => typemax(Int) for location in unvisited)
    distances[(start_pos[1], start_pos[2], 'e')] = 0

    best_predecessors = Dict(location => Set{Tuple{Int,Int,Char}}() for location in unvisited)

    while !isempty(unvisited)
        best_location = (0, 0, 'n')
        best_distance = typemax(Int)
        for location in unvisited
            if distances[location] < best_distance
                best_location = location
                best_distance = distances[location]
            end
        end
        pop!(unvisited, best_location)

        if !any((end_pos[1], end_pos[2], direction) in unvisited for direction in ['n', 's', 'e', 'w'])
            min_distance = minimum([distances[(end_pos[1], end_pos[2], direction)] for direction in ['n', 's', 'e', 'w']])
            predecessor_stack = [(end_pos[1], end_pos[2], direction) for direction in ['n', 's', 'e', 'w']]
            predecessor_stack = [location for location in predecessor_stack if distances[location] == min_distance]
            best_path_tiles = Set([end_pos])
            while !isempty(predecessor_stack)
                elem = pop!(predecessor_stack)
                union!(best_path_tiles, Set([(row, col) for (row, col, _) in best_predecessors[elem]]))
                for pred in best_predecessors[elem]
                    push!(predecessor_stack, pred)
                end
            end
            return length(best_path_tiles)
        end

        row, col, direction = best_location
        neighbors = []
        if direction == 'n'
            neighbors = [(row, col, 'e', 1000), (row, col, 'w', 1000)]
            if row > 1 && maze[row-1, col]
                push!(neighbors, (row - 1, col, 'n', 1))
            end
        elseif direction == 's'
            neighbors = [(row, col, 'e', 1000), (row, col, 'w', 1000)]
            if row < size(maze, 1) - 1 && maze[row+1, col]
                push!(neighbors, (row + 1, col, 's', 1))
            end
        elseif direction == 'e'
            neighbors = [(row, col, 'n', 1000), (row, col, 's', 1000)]
            if col < size(maze, 2) - 1 && maze[row, col+1]
                push!(neighbors, (row, col + 1, 'e', 1))
            end
        elseif direction == 'w'
            neighbors = [(row, col, 'n', 1000), (row, col, 's', 1000)]
            if col > 1 && maze[row, col-1]
                push!(neighbors, (row, col - 1, 'w', 1))
            end
        end

        for (neighbor_row, neighbor_col, neighbor_direction, transition_cost) in neighbors
            if best_distance + transition_cost < distances[(neighbor_row, neighbor_col, neighbor_direction)]
                distances[(neighbor_row, neighbor_col, neighbor_direction)] = best_distance + transition_cost
                best_predecessors[(neighbor_row, neighbor_col, neighbor_direction)] = Set([best_location])
            elseif best_distance + transition_cost == distances[(neighbor_row, neighbor_col, neighbor_direction)]
                push!(best_predecessors[(neighbor_row, neighbor_col, neighbor_direction)], best_location)
            end
        end
    end

    return 0
end

function main()
    test_maze, test_start_pos, test_end_pos = parse_file("testinput.txt")
    maze, start_pos, end_pos = parse_file("input.txt")

    println("Challenge 1 test: ", dijkstra_search(test_maze, test_start_pos, test_end_pos))
    println("Challenge 1: ", dijkstra_search(maze, start_pos, end_pos))

    println("Challenge 2 test: ", dijkstra_search_best_path_tiles(test_maze, test_start_pos, test_end_pos))
    println("Challenge 2: ", dijkstra_search_best_path_tiles(maze, start_pos, end_pos))
end

main()
