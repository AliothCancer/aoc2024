using LinearAlgebra;
using Pipe: @pipe
using Match;
using PlotlyJS

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


function part1()
    global test
    #input = read("input/input_day4.txt",String)
    parsed_input = reduce(hcat, split(test) .|> collect)
    col_start,row_start = 1,2
    view_width = 4 
    col_end, row_end = col_start+view_width-1, row_start+view_width-1
    println(parsed_input[col_start:col_end,row_start:row_end])

    total_occurences = 0

end

part1()