function get_grid(filename::String)::Matrix{Char}
    lines = readlines(filename)

    num_rows = length(lines)
    num_cols = length(first(lines))

    matrix = Matrix{Char}(undef, num_rows, num_cols)
    for (row, line) in enumerate(lines)
        for (col, char) in enumerate(line)
            matrix[row, col] = char
        end
    end

    matrix
end

function get_antennas(map::Matrix{Char})::Dict{Char,Vector{Tuple{Int,Int}}}
    result = Dict{Char,Vector{Tuple{Int,Int}}}()
    for row in axes(map, 1), col in axes(map, 2)
        char = map[row, col]
        if char == '.'
            continue
        end
        if !(char in keys(result))
            result[char] = Vector{Tuple{Int,Int}}()
        end
        push!(result[char], (row, col))
    end
    result
end

function get_antinodes(antennas::Vector{Tuple{Int,Int}}, max_row::Int, max_col::Int)::Set{Tuple{Int,Int}}
    antinodes = Set{Tuple{Int,Int}}()
    for (i, (row_1, col_1)) in enumerate(antennas), (j, (row_2, col_2)) in enumerate(antennas)
        if i == j
            continue
        end
        antinode_row = 2 * row_2 - row_1
        antinode_col = 2 * col_2 - col_1
        if 1 <= antinode_row <= max_row && 1 <= antinode_col <= max_col
            push!(antinodes, (antinode_row, antinode_col))
        end
    end
    antinodes
end

function get_antinodes(antennas::Dict{Char,Vector{Tuple{Int,Int}}}, max_row::Int, max_col::Int)::Set{Tuple{Int,Int}}
    result = Set{Tuple{Int,Int}}()
    for (freq, freq_antennas) in antennas
        result = union(result, get_antinodes(freq_antennas, max_row, max_col))
    end
    result
end

function get_antinodes(grid::Matrix{Char})::Set{Tuple{Int,Int}}
    get_antinodes(get_antennas(grid), size(grid, 1), size(grid, 2))
end

function get_antinodes_with_resonance(antennas::Vector{Tuple{Int,Int}}, max_row::Int, max_col::Int)::Set{Tuple{Int,Int}}
    antinodes = Set{Tuple{Int,Int}}()
    for (i, (row_1, col_1)) in enumerate(antennas), (j, (row_2, col_2)) in enumerate(antennas)
        if i == j
            continue
        end
        delta_row = row_2 - row_1
        delta_col = col_2 - col_1
        divisor = gcd(delta_row, delta_col)
        delta_row = div(delta_row, divisor)
        delta_col = div(delta_col, divisor)

        antinode_row = row_1
        antinode_col = col_1
        while 1 <= antinode_row <= max_row && 1 <= antinode_col <= max_col
            push!(antinodes, (antinode_row, antinode_col))
            antinode_row += delta_row
            antinode_col += delta_col
        end
    end
    antinodes
end

function get_antinodes_with_resonance(antennas::Dict{Char,Vector{Tuple{Int,Int}}}, max_row::Int, max_col::Int)::Set{Tuple{Int,Int}}
    result = Set{Tuple{Int,Int}}()
    for freq_antennas in values(antennas)
        result = union(result, get_antinodes_with_resonance(freq_antennas, max_row, max_col))
    end
    result
end

function get_antinodes_with_resonance(grid::Matrix{Char})::Set{Tuple{Int,Int}}
    get_antinodes_with_resonance(get_antennas(grid), size(grid, 1), size(grid, 2))
end

function main()
    test_grid = get_grid("testinput.txt")
    grid = get_grid("input.txt")

    println("Challenge 1 test: ", length(get_antinodes(test_grid)))
    println("Challenge 1: ", length(get_antinodes(grid)))

    println("Challenge 2 test: ", length(get_antinodes_with_resonance(test_grid)))
    println("Challenge 2: ", length(get_antinodes_with_resonance(grid)))
end

main()
