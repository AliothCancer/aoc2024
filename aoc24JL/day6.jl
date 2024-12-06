using Pipe: @pipe
using Base.Enums;


@enum Direction begin
    Up
    Down
    Left
    Right
end

mutable struct Position
    x::Int
    y::Int
    direction::Direction
end

function get_position(coor::Position)::Tuple
    return (coor.x, coor.y)
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
        coor.y -= 1
    elseif coor.direction == Down::Direction
        coor.y += 1
    elseif coor.direction == Left::Direction
        coor.x -= 1
    elseif coor.direction == Right::Direction
        coor.x += 1
    end
end

function meet_obstacle(coor::Position)::Bool
    global parsed_input
    if coor.direction == Up::Direction
        if coor.y > 4
            println(parsed_input[coor.y-4:coor.y, coor.x])
        end
        return parsed_input[coor.y - 1, coor.x] == '#'
        
    elseif coor.direction == Down::Direction
        return parsed_input[coor.y + 1, coor.x] == '#'
        
    elseif coor.direction == Left::Direction
        return parsed_input[coor.y, coor.x - 1] == '#'
        
    elseif coor.direction == Right::Direction
        if coor.x < 126
            println(parsed_input[coor.y, coor.x:coor.x+4])
        end
        return parsed_input[coor.y, coor.x + 1] == '#'

    else
        @error "No direction matched: Not able to establish obstacles correctly"
        exit()
    end
end

function find_guard(parsed_input, orientation::Char='^')::Tuple
    global parsed_input
    guard_position = nothing
    for (x, col) in enumerate(eachcol(parsed_input))
        y = findall(orientation, col |> String)
        if !isempty(y)
            guard_position = y[1], x
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

    parsed_input = reduce(
        hcat,
        map(collect, input)
    )

    (y_len, x_len) = size(parsed_input)
    
    (
        guard_position_y,
        guard_position_x
    ) = find_guard(parsed_input)

    guard = Position(
        guard_position_x,
        guard_position_y,
        Up::Direction
    )

    unique_positions = Set{Tuple}()
    
    while true
        guard_position = guard |> get_position
        println("guard_position: $guard_position direction: $(guard.direction)")
        if !(guard.x > 2 && guard.x < x_len) || !(guard.y > 2 && guard.y < y_len-1)
            push!(unique_positions, guard_position)
            break
        else
            if guard |> meet_obstacle
                guard |> change_direction!
            else
                println("added position: $guard_position")
                push!(unique_positions, guard_position)

                guard |> move!
            end
        end
    end
    total_positions = unique_positions |> length
    println("unique positions: $total_positions")

end


part1()