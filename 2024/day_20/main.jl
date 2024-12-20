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

function get_distances(maze::Matrix{Bool}, start_pos::Tuple{Int,Int})::Matrix{Int}
    unvisited = Set([(row, col) for row in axes(maze, 1) for col in axes(maze, 2) if maze[row, col]])
    distances = Dict(location => typemax(Int) for location in unvisited)
    distances[(start_pos[1], start_pos[2])] = 0

    while !isempty(unvisited)
        best_location = (0, 0)
        best_distance = typemax(Int)
        for location in unvisited
            if distances[location] < best_distance
                best_location = location
                best_distance = distances[location]
            end
        end
        pop!(unvisited, best_location)

        row, col = best_location
        neighbors = []
        if row > 1 && maze[row-1, col]
            push!(neighbors, (row - 1, col))
        end
        if row < size(maze, 1) - 1 && maze[row+1, col]
            push!(neighbors, (row + 1, col))
        end
        if col < size(maze, 2) - 1 && maze[row, col+1]
            push!(neighbors, (row, col + 1))
        end
        if col > 1 && maze[row, col-1]
            push!(neighbors, (row, col - 1))
        end

        for (neighbor_row, neighbor_col) in neighbors
            if best_distance + 1 < distances[(neighbor_row, neighbor_col)]
                distances[(neighbor_row, neighbor_col)] = best_distance + 1
            end
        end
    end

    distance_maze = Matrix{Int}(undef, size(maze, 1), size(maze, 2))
    fill!(distance_maze, typemax(Int))

    for (pos, dist) in distances
        distance_maze[pos...] = dist
    end

    return distance_maze
end

function taxicab_dist(pos_1::Tuple{Int,Int}, pos_2::Tuple{Int,Int})::Int
    abs(pos_1[1] - pos_2[1]) + abs(pos_1[2] - pos_2[2])
end

function cheated_time(start_distance_maze::Matrix{Int}, end_distance_maze::Matrix{Int}, cheat_pos_1::Tuple{Int,Int}, cheat_pos_2::Tuple{Int,Int})::Int
    cheat_1_distance = start_distance_maze[cheat_pos_1...]
    cheat_2_distance = end_distance_maze[cheat_pos_2...]

    return cheat_1_distance + taxicab_dist(cheat_pos_1, cheat_pos_2) + cheat_2_distance
end

function get_all_cheat_times(start_distance_maze::Matrix{Int}, end_distance_maze::Matrix{Int}, max_cheat_length::Int)::Dict{Tuple{Tuple{Int,Int},Tuple{Int,Int}},Int}
    cheat_times = Dict{Tuple{Tuple{Int,Int},Tuple{Int,Int}},Int}()

    for row in axes(start_distance_maze, 1), col in axes(start_distance_maze, 2)
        cheat_pos_1 = (row, col)
        if start_distance_maze[cheat_pos_1...] == typemax(Int)
            continue
        end

        for target_row in axes(start_distance_maze, 1), target_col in axes(start_distance_maze, 2)
            cheat_pos_2 = (target_row, target_col)
            if start_distance_maze[cheat_pos_2...] == typemax(Int)
                continue
            end
            if taxicab_dist(cheat_pos_1, cheat_pos_2) > max_cheat_length
                continue
            end

            time = cheated_time(start_distance_maze, end_distance_maze, cheat_pos_1, cheat_pos_2)
            cheat_times[(cheat_pos_1, cheat_pos_2)] = time
        end
    end

    return cheat_times
end

function num_good_cheat_times(maze::Matrix{Bool}, start_pos::Tuple{Int,Int}, end_pos::Tuple{Int,Int}, threshold::Int, max_cheat_length::Int)::Int
    start_distance_maze = get_distances(maze, start_pos)
    end_distance_maze = get_distances(maze, end_pos)

    best_normal_time = start_distance_maze[end_pos...]

    cheat_times = get_all_cheat_times(start_distance_maze, end_distance_maze, max_cheat_length)

    num_good_cheats = 0
    for cheat_time in values(cheat_times)
        if cheat_time <= best_normal_time - threshold
            num_good_cheats += 1
        end
    end

    num_good_cheats
end

function main()
    test_maze, test_start_pos, test_end_pos = parse_file("testinput.txt")
    maze, start_pos, end_pos = parse_file("input.txt")

    println("Challenge 1 test: ", num_good_cheat_times(test_maze, test_start_pos, test_end_pos, 64, 2))
    println("Challenge 1: ", num_good_cheat_times(maze, start_pos, end_pos, 100, 2))

    println("Challenge 2 test: ", num_good_cheat_times(test_maze, test_start_pos, test_end_pos, 72, 20))
    println("Challenge 2: ", num_good_cheat_times(maze, start_pos, end_pos, 100, 20))
end

main()
