function parse_file(filename::String)::Matrix{Int}
    lines = readlines(filename)
    num_rows, num_cols = length(lines), length(first(lines))
    matrix = Matrix{Int}(undef, num_rows, num_cols)

    for (row, line) in enumerate(lines)
        for (col, char) in enumerate(line)
            matrix[row, col] = parse(Int, char)
        end
    end

    matrix
end

function neighbors(position::Tuple{Int,Int}, max_row::Int, max_col::Int)::Vector{Tuple{Int,Int}}
    row, col = position
    deltas = [(-1, 0), (0, 1), (1, 0), (0, -1)]
    filter(x -> 1 <= x[1] <= max_row && 1 <= x[2] <= max_col, [(row + dr, col + dc) for (dr, dc) in deltas])
end

function neighbors(positions::Vector{Tuple{Int,Int}}, max_row::Int, max_col::Int)::Vector{Tuple{Int,Int}}
    unique(Set(reduce(vcat, neighbors.(positions, max_row, max_col))))
end

mutable struct ScoredPosition
    position::Tuple{Int,Int}
    score::Int
end

function neighbors(positions::Vector{ScoredPosition}, max_row::Int, max_col::Int)::Vector{ScoredPosition}
    neighbor_map = Dict{Tuple{Int,Int},Int}()
    for scored_pos in positions
        for neighbor in neighbors(scored_pos.position, max_row, max_col)
            neighbor_map[neighbor] = get(neighbor_map, neighbor, 0) + scored_pos.score
        end
    end
    [ScoredPosition(pos, score) for (pos, score) in neighbor_map]
end

function get_higher_neighbors(positions::Vector{Tuple{Int,Int}}, topo_map::Matrix{Int})::Vector{Tuple{Int,Int}}
    max_row, max_col = size(topo_map)
    current_height = topo_map[positions[1]...]
    filter(pos -> topo_map[pos...] == current_height + 1, neighbors(positions, max_row, max_col))
end

function get_higher_neighbors(scored_positions::Vector{ScoredPosition}, topo_map::Matrix{Int})::Vector{ScoredPosition}
    max_row, max_col = size(topo_map)
    current_height = topo_map[scored_positions[1].position...]
    filter(pos -> topo_map[pos.position...] == current_height + 1, neighbors(scored_positions, max_row, max_col))
end

function get_score(trailhead::Tuple{Int,Int}, topo_map::Matrix{Int})::Int
    positions = [trailhead]
    for _ in 1:9
        positions = get_higher_neighbors(positions, topo_map)
        if isempty(positions)
            break
        end
    end
    length(positions)
end

function get_rating(trailhead::Tuple{Int,Int}, topo_map::Matrix{Int})::Int
    scored_positions = [ScoredPosition(trailhead, 1)]
    for _ in 1:9
        scored_positions = get_higher_neighbors(scored_positions, topo_map)
        if isempty(scored_positions)
            break
        end
    end
    sum(p -> p.score, scored_positions)
end

function sum_trailhead_scores(topo_map::Matrix{Int})::Int
    sum(get_score((row, col), topo_map) for row in axes(topo_map, 1) for col in axes(topo_map, 2) if topo_map[row, col] == 0)
end

function sum_trailhead_ratings(topo_map::Matrix{Int})::Int
    sum(get_rating((row, col), topo_map) for row in axes(topo_map, 1) for col in axes(topo_map, 2) if topo_map[row, col] == 0)
end

function main()
    test_topo_map = parse_file("testinput.txt")
    topo_map = parse_file("input.txt")

    println("Challenge 1 test: ", sum_trailhead_scores(test_topo_map))
    println("Challenge 1: ", sum_trailhead_scores(topo_map))
    println("Challenge 2 test: ", sum_trailhead_ratings(test_topo_map))
    println("Challenge 2: ", sum_trailhead_ratings(topo_map))
end

main()
