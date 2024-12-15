
function parse_input(filename::String)::Tuple{Matrix{Char},Tuple{Int,Int},Vector{Tuple{Int,Int}}}
    content = readlines(filename)
    border = content[1]
    num_rows = 0
    for idx in 2:length(content)
        if content[idx] == border
            num_rows = idx
        end
    end

    num_cols = length(border)
    warehouse = Matrix{Char}(undef, num_rows, num_cols)

    robot_position = (0, 0)

    for row in 1:num_rows, col in 1:num_cols
        if content[row][col] != '@'
            warehouse[row, col] = content[row][col]
            col
        else
            warehouse[row, col] = '.'
            robot_position = (row, col)
        end
    end

    moves = Vector{Tuple{Int,Int}}()

    for row in num_rows+2:length(content)
        for move in content[row]
            if move == '<'
                push!(moves, (0, -1))
            elseif move == '^'
                push!(moves, (-1, 0))
            elseif move == '>'
                push!(moves, (0, 1))
            elseif move == 'v'
                push!(moves, (1, 0))
            end
        end
    end

    warehouse, robot_position, moves
end

function perform_move!(warehouse::Matrix{Char}, robot_position::Tuple{Int,Int}, move_direction::Tuple{Int,Int})::Tuple{Int,Int}
    free_pos = (-1, -1)

    steps = 1
    while true
        pos = robot_position .+ steps .* move_direction
        if warehouse[pos...] == '.'
            free_pos = pos
            break
        elseif warehouse[pos...] == '#'
            break
        else
            steps += 1
        end
    end

    new_robot_position = robot_position
    if free_pos != (-1, -1)
        new_robot_position = robot_position .+ move_direction
        if free_pos != new_robot_position
            warehouse[new_robot_position...] = '.'
            warehouse[free_pos...] = 'O'
        end
    end

    new_robot_position
end

function perform_moves!(warehouse::Matrix{Char}, robot_position::Tuple{Int,Int}, move_directions::Vector{Tuple{Int,Int}})
    new_robot_position = robot_position
    # warehouse[new_robot_position...] = '@'
    # println(join(map(row -> join(row, ""), eachrow(warehouse)), "\n"))
    # warehouse[new_robot_position...] = '.'
    for move_direction in move_directions
        # if move_direction == (0, -1)
        #     println('<')
        # elseif move_direction == (-1, 0)
        #     println('^')
        # elseif move_direction == (0, 1)
        #     println('>')
        # elseif move_direction == (1, 0)
        #     println('v')
        # end
        new_robot_position = perform_move!(warehouse, new_robot_position, move_direction)
        # warehouse[new_robot_position...] = '@'
        # println(join(map(row -> join(row, ""), eachrow(warehouse)), "\n"))
        # println()
        # warehouse[new_robot_position...] = '.'
    end
end

function sum_gps_coords(warehouse::Matrix{Char})::Int
    sum = 0
    for row in axes(warehouse, 1), col in axes(warehouse, 2)
        if warehouse[row, col] == 'O'
            sum += 100 * (row - 1) + (col - 1)
        end
    end

    sum
end

function main()
    test_warehouse, test_robot_position, test_moves = parse_input("testinput.txt")
    warehouse, robot_position, moves = parse_input("input.txt")

    perform_moves!(test_warehouse, test_robot_position, test_moves)
    println("Challenge 1 test: ", sum_gps_coords(test_warehouse))

    perform_moves!(warehouse, robot_position, moves)
    println("Challenge 1: ", sum_gps_coords(warehouse))
end

main()
