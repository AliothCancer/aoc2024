using Pipe: @pipe

input = read("input/input_day1.txt", String)


split_in_lines(input::String) = split(input, "\n", keepempty=false)

function parse_line(line::SubString)::AbstractArray{Int}
    @pipe line |> split |> parse.(Int, _)
end

to_matrix(parsed_lines::AbstractArray) = reduce(hcat, parsed_lines)

parsed_input = input |>
               split_in_lines .|>
               parse_line |>
               to_matrix

(col_a, col_b) = parsed_input[1, :], parsed_input[2, :]

sort!(col_a)
sort!(col_b)

total_distance = @pipe zip(col_a, col_b) |>
                       map(
                           pair -> begin
                               a, b = pair
                               abs(a - b)
                           end, _) |>
                       sum
println("total distance: $total_distance")