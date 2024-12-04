using LinearAlgebra;
using Pipe: @pipe

test = """MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
MXMXAXMASX"""
input = read("input/input_day4.txt",String)
parsed_input = reduce(hcat, split(test) .|> collect)
rev_parsed_input = reverse(parsed_input)
reg = r"XMAS|SAMX"
total_occurences = 0

for iter in [eachcol(parsed_input), eachrow(parsed_input)]
    global total_occurences
    for colrow in iter
        total_occurences +=
            eachmatch(
                reg,
                reduce(string, colrow),
                overlap=false
            ) |>
            collect |>
            length
    end
end

mat_dim = size(parsed_input)

for n in -mat_dim[1]:mat_dim[2]
    global total_occurences
    total_occurences +=
            eachmatch(
                reg,
                reduce(string, diag(parsed_input,n), init=""),
                overlap=false
            ) |>
            collect |>
            length
    total_occurences +=
            eachmatch(
                reg,
                reduce(string, diag(rev_parsed_input,n), init=""),
                overlap=false
            ) |>
            collect |>
            length
end

println("total: $total_occurences")