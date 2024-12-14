mutable struct Robot
    position::Tuple{Int, Int}
    velocity::Tuple{Int, Int}
end

function parse_file(filename::String)::Vector{Robot}
    content = read(filename, String)
    pattern = r"p=([-\d]+),([-\d]+) v=([-\d]+),([-\d]+)"
    return [
        Robot(
            (parse(Int, match.captures[1]), parse(Int, match.captures[2])),
            (parse(Int, match.captures[3]), parse(Int, match.captures[4]))
        ) for match in eachmatch(pattern, content)
    ]
end

function advanced_robot(robot::Robot, steps::Int, width::Int, height::Int)::Robot
    new_position = (
        mod(robot.position[1] + steps * robot.velocity[1], width),
        mod(robot.position[2] + steps * robot.velocity[2], height)
    )
    return Robot(new_position, robot.velocity)
end

function advanced_robots(robots::Vector{Robot}, steps::Int, width::Int, height::Int)::Vector{Robot}
    return [advanced_robot(robot, steps, width, height) for robot in robots]
end

function display_robots(robots::Vector{Robot}, width::Int, height::Int)
    matrix = fill('.', height, width)
    for robot in robots
        matrix[robot.position[2] + 1, robot.position[1] + 1] = '#'
    end
    println(join(map(row -> join(row, ""), eachrow(matrix)), "\n"))
end

function quadrant_counts(robots::Vector{Robot}, width::Int, height::Int)::Tuple{Int, Int, Int, Int}
    counts = zeros(Int, 4)
    mid_x, mid_y = div(width, 2), div(height, 2)
    for robot in robots
        x, y = robot.position
        if x < mid_x && y < mid_y
            counts[1] += 1  # Top-left
        elseif x > mid_x && y < mid_y
            counts[2] += 1  # Top-right
        elseif x < mid_x && y > mid_y
            counts[3] += 1  # Bottom-left
        elseif x > mid_x && y > mid_y
            counts[4] += 1  # Bottom-right
        end
    end
    return tuple(counts...)
end

function safety_factor(robots::Vector{Robot}, steps::Int, width::Int, height::Int)::Int
    prod(quadrant_counts(advanced_robots(robots, steps, width, height), width, height))
end

function main()
    test_robots = parse_file("testinput.txt")
    robots = parse_file("input.txt")

    println("Challenge 1 test: ", safety_factor(test_robots, 100, 11, 7))
    println("Challenge 1: ", safety_factor(robots, 100, 101, 103))

    println("Challenge 2:")
    display_robots(advanced_robots(robots, 6475, 101, 103), 101, 103)
end

main()
