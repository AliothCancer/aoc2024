using Pipe: @pipe


split_in_lines(input) =
    split(input, "\n", keepempty=false)

parse_line(line) =
    @pipe line |> split |> parse.(Int, _)

to_matrix(parsed_lines) =
    reduce(hcat, parsed_lines)

    
input = read(
    "input/input_day1.txt",
    String
)

parsed_input =
    input |>
    split_in_lines .|>
    parse_line |>
    to_matrix

(col_a, col_b) =
    parsed_input[1, :], 
    parsed_input[2, :]

sort!(col_a)
sort!(col_b)

total_distance =
    @pipe zip(col_a, col_b) |>
          map(
              pair -> begin
                  a, b = pair
                  abs(a - b)
              end, _) |>
          sum
println(
    "total distance: $total_distance"
)