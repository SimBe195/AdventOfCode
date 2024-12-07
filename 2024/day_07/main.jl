struct Equation
    goal::Int
    current::Int
    remaining::Vector{Int}
end

function parse_equation(line::String)::Equation
    words = split(line, ' ')
    goal = parse(Int, chop(words[1], tail=1))
    current = parse(Int, words[2])
    remaining = parse.(Int, words[3:end])

    Equation(goal, current, remaining)
end


function create_equations(filename::String)::Vector{Equation}
    [parse_equation(line) for line in readlines(filename)]
end

function equation_solvable(equation::Equation)::Bool
    if equation.current > equation.goal
        return false
    end

    if isempty(equation.remaining)
        return equation.current == equation.goal
    end

    head, tail = first(equation.remaining), equation.remaining[2:end]

    equation_solvable(Equation(equation.goal, equation.current + head, tail)) ||
        equation_solvable(Equation(equation.goal, equation.current * head, tail))
end

function equation_solvable_with_concat(equation::Equation)::Bool
    if equation.current > equation.goal
        return false
    end

    if isempty(equation.remaining)
        return equation.current == equation.goal
    end

    head, tail = first(equation.remaining), equation.remaining[2:end]

    equation_solvable_with_concat(Equation(equation.goal, equation.current + head, tail)) ||
        equation_solvable_with_concat(Equation(equation.goal, equation.current * head, tail)) ||
        equation_solvable_with_concat(Equation(equation.goal, parse(Int, string(equation.current) * string(head)), tail))
end

function num_solvable_equations(equations::Vector{Equation})::Int
    sum(equation.goal for equation in equations if equation_solvable(equation))
end

function num_solvable_equations_with_concat(equations::Vector{Equation})::Int
    sum(equation.goal for equation in equations if equation_solvable_with_concat(equation))
end

function main()
    println("Challenge 1 test: ", num_solvable_equations(create_equations("testinput.txt")))
    println("Challenge 1: ", num_solvable_equations(create_equations("input.txt")))

    println("Challenge 2 test: ", num_solvable_equations_with_concat(create_equations("testinput.txt")))
    println("Challenge 2: ", num_solvable_equations_with_concat(create_equations("input.txt")))
end

main()
