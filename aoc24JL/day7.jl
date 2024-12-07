using Pipe: @pipe
using Combinatorics;

test = """190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"""


function generate_permutations(values, n)
    return Iterators.product(ntuple(_ -> values, n)...)
end


function apply_operator(accumulator, operation_tuple)
    operator, value = operation_tuple
    return operator(accumulator, value)
end

function calculate(comb, numbers, expected)::Union{Int, Nothing}  
    result = @pipe zip(comb,numbers[2:end]) |>
        reduce(apply_operator,_, init=numbers[1])|>
        sum
    if result == expected
        return result
    else
        return nothing
    end
end

function test_equation(equation::AbstractArray)::Union{Int, Nothing}
    expected = equation[1]
    numbers = equation[2:end]
    println("equation =  $expected:$numbers")
    operators_seats = length(numbers) - 1
    operations = [+, *]
    possible_ops = generate_permutations(operations, operators_seats)

    result = nothing

    for comb in possible_ops
        result = calculate(comb, numbers, expected)
        if !isnothing(result)
            println("comb:$comb numbers:$numbers = expected:$expected")
            break 
        end
    end
    return result

end

function part1()
    global test
    input = readlines("input/input_day7.txt")
    parsed_input = @pipe input |>
        map(split, _) |>
        map(x -> parse.(Int, replace.(x, ':' => "")), _) 

    solution = @pipe parsed_input |>
        map(test_equation, _) |>
        filter(!isnothing, _) |>
        sum

    "sum of valid test: $solution" |> println
end

function some_test()
    "calculate func test: $(calculate([*,+], [2, 5, 3], 13) == 13)" |> println
    "test_equation func test: $(test_equation([13, 2, 5, 3]) == 13)" |> println
end

@time part1()