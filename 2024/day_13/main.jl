using LinearAlgebra

mutable struct ClawMachine
  buttons::Matrix{Int}
  prize::Vector{Int}
end

function parse_file(filename::String)::Vector{ClawMachine}
  content = read(filename, String)

  pattern = r"Button A: X\+(\d+), Y\+(\d+)\s+Button B: X\+(\d+), Y\+(\d+)\s+Prize: X=(\d+), Y=(\d+)"

  return [
    ClawMachine(
      [parse(Int, match.captures[1]) parse(Int, match.captures[3]); parse(Int, match.captures[2]) parse(Int, match.captures[4])],
      [parse(Int, match.captures[5]); parse(Int, match.captures[6])])
    for match in eachmatch(pattern, content)
  ]
end

function solve_claw_machine(claw_machine::ClawMachine)::Int
  solution = claw_machine.buttons \ claw_machine.prize
  if isnothing(solution)
    return 0
  end
  int_solution = Int.(round.(solution))
  if all(>=(0), int_solution) && claw_machine.buttons * int_solution == claw_machine.prize
    return 3 * int_solution[1] + int_solution[2]
  end
  return 0
end

function total_cost(claw_machines::Vector{ClawMachine})::Int
  sum(solve_claw_machine(claw_machine) for claw_machine in claw_machines)
end

function adjust_claw_machines!(claw_machines::Vector{ClawMachine})
  for claw_machine in claw_machines
    claw_machine.prize = claw_machine.prize .+ 10000000000000
  end
end

function main()
  test_claw_machines = parse_file("testinput.txt")
  claw_machines = parse_file("input.txt")

  println("Challenge 1 test: ", total_cost(test_claw_machines))
  println("Challenge 1: ", total_cost(claw_machines))

  adjust_claw_machines!(test_claw_machines)
  adjust_claw_machines!(claw_machines)

  println("Challenge 2 test: ", total_cost(test_claw_machines))
  println("Challenge 2: ", total_cost(claw_machines))
end

main()
