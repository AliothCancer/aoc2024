using Pipe: @pipe

# PART 1
split_in_lines(input) =
    split(input, "\n", keepempty=false)

parse_line(line) =
    line |>
    split .|>
    parser |>
    x -> !isnothing(x) ? x :
         @warn "Impossibile parsare la line"

parser(value) =
    try
        parse(Int, value)
    catch e
        @error "Errore durante il parsing:\n$e"
        exit()
    end

to_matrix(parsed_lines) =
    reduce(hcat, parsed_lines)

function part1()
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
    return total_distance
end
#println("total distance: $part1()")

# PART 2
"""
Contare quante volte l'id della colonna a sinistra
compare nella colonna di destra
"""
function part2()
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

    tot_id = 0
    for id in col_a
        global tot_id
        times = count(x -> x == id, col_b)
        tot_id += id * times
    end
end
#println("Parte II: similarity score: $tot_id")

@time part1()