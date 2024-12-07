using Pipe: @pipe
using Base.Enums;


@enum Direction begin
    Up
    Down
    Left
    Right
end

mutable struct Position
    col::Int
    row::Int
    direction::Direction
end

function get_position(coor::Position)::Tuple
    return (coor.row, coor.col)
end

function change_direction!(coor::Position)
    if coor.direction == Up::Direction
        coor.direction = Right::Direction

    elseif coor.direction == Right::Direction
        coor.direction = Down::Direction

    elseif coor.direction == Down::Direction
        coor.direction = Left::Direction

    elseif coor.direction == Left::Direction
        coor.direction = Up::Direction
    else
        @error "No direction matched for $coor"
        exit()
    end
end

function move!(coor::Position)
    if coor.direction == Up::Direction
        coor.row -= 1
    elseif coor.direction == Down::Direction
        coor.row += 1
    elseif coor.direction == Left::Direction
        coor.col -= 1
    elseif coor.direction == Right::Direction
        coor.col += 1
    end
end

function meet_obstacle(coor::Position)::Bool
    global parsed_input
    if coor.direction == Up::Direction
        #if coor.x < 126
        #    println(parsed_input[coor.x-4:coor.x, coor.y])
        #end
        return parsed_input[coor.row - 1][coor.col] == '#'
        
    elseif coor.direction == Down::Direction
        return parsed_input[coor.row + 1][coor.col] == '#'
        
    elseif coor.direction == Left::Direction
        return parsed_input[coor.row][coor.col - 1] == '#'
        
    elseif coor.direction == Right::Direction
        #if coor.y > 4
        #    println(parsed_input[coor.x, coor.y:coor.y+4])
        #end
        return parsed_input[coor.row][coor.col + 1] == '#'

    else
        @error "No direction matched: Not able to establish obstacles correctly"
        exit()
    end
end

function find_guard(parsed_input, orientation::Char='^')::Tuple
    global parsed_input
    guard_position = nothing
    for (n_row, row) in enumerate(parsed_input)
        n_col = findall(orientation, row |> String)
        if !isempty(n_col)
            guard_position = n_col[1],n_row
        end
    end
    if isnothing(guard_position)
        @error "guard not found!!"
        exit()
    else
        println("guard position: $guard_position")
    end
    return guard_position
end

function part1()
    global parsed_input
    input = readlines("input/input_day6.txt")

    parsed_input = input .|> collect

    (row_len, col_len) = length(parsed_input),length(parsed_input[1]) 
    
    (
        guard_position_col,
        guard_position_row
    ) = find_guard(parsed_input)


    guard = Position(
        guard_position_col,
        guard_position_row,
        Up::Direction
    )
    println("verifica: $(parsed_input[guard_position_row][guard_position_col]) == ^")
    
    unique_positions = Set{Tuple}()
    
    counter = 0
    while true
        guard_position = guard |> get_position
        println("guard_position: $guard_position direction: $(guard.direction)")
        if !(guard.col > 2 && guard.col < col_len) || !(guard.row > 2 && guard.row < col_len-1)
            push!(unique_positions, guard_position)
            break
        else
            if guard |> meet_obstacle
                guard |> change_direction!
                counter = 0
            else
                #println("added position: $guard_position")
                push!(unique_positions, guard_position)

                guard |> move!
                counter += 1
                println("same direction moves $counter")
            end
        end
    end
    total_positions = unique_positions |> length
    println("unique positions: $total_positions")

end


part1()