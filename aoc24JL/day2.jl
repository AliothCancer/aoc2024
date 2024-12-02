using Pipe: @pipe

using Match

read_input() = read("input/input_day2.txt", String)


function is_safe_report(rep)
    (n, rep) = rep
    deltas = [
        rep[i]-rep[i+1] 
        for i in 1:length(rep)-1
    ]
    
    if maximum(abs, deltas) <= 3
        return @pipe deltas .|>
            sign |> 
            reduce(*,_, init=1) |>
            isequal(1,_)
    else
        println("n. $n rep: $rep")
        return false
    end
    
end

function part1()
    input = read_input()
    test = 
"""\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 1 1 1 1
1 3 6 7 9"""
    output = 
    @pipe input|>
        split(_, "\n", keepempty=false) |>
        map(x -> parse.(Int, split(x)), _) |>
        filter(is_safe_report,_) |>
        length

    println(
        output
    )
end

part1()