
mutable struct Untyped
    name::String
    outputs::Vector{String}
end

function receive(un::Untyped, from::String, pulse::String)
    return nothing
end

mutable struct FlipFlop
    name::String
    on::Bool
    outputs::Vector{String}
end

function receive(ff::FlipFlop, from::String, pulse::String)
    if pulse == "low"
        ff.on = !ff.on
        if ff.on
            # Off to on case
            return [(ff.name, "high", output) for output in ff.outputs]
        else
            # On to off case
            return [(ff.name, "low", output) for output in ff.outputs]
        end
    end
end

mutable struct Conjunction
    name::String
    input_states::Dict{String, String}
    outputs::Vector{String}

    Conjunction(name, keys) = new(name, Dict(), keys)
end

function receive(c::Conjunction, from::String, pulse::String)
    c.input_states[from] = pulse
    if all(value == "high" for value in values(c.input_states))
        return [(c.name, "low", output) for output in c.outputs]
    else
        return [(c.name, "high", output) for output in c.outputs]
    end
end

mutable struct Broadcast
    name::String
    outputs::Vector{String}
end

function receive(b::Broadcast, from::String, pulse::String)
    return [(b.name, pulse, output) for output in b.outputs]
end


function part_1(input)
    modules = Dict{String, Any}()
    conjunctions = Vector{String}()
    for line in split(input, "\n")
        rawKey, raw_outputs = split(line, " -> ")
        outputs = [String(s) for s in split(raw_outputs, ", ")]
        if startswith(rawKey, "%")
            key = String(rawKey[2:end])
            modules[key] = FlipFlop(key, false, outputs)
        elseif startswith(rawKey, "&")
            key = String(rawKey[2:end])
            modules[key] = Conjunction(key, outputs)
            conjunctions = vcat(conjunctions, key)
        elseif startswith(rawKey, "broadcaster")
            modules["broadcaster"] = Broadcast("broadcaster", outputs)
        else
            # Untyped case
            modules[key] = Untyped(rawKey, [])
        end
    end

     # Make sure all conjunction inputs are known
    for (key, mod) in pairs(modules)
        for output in mod.outputs
            if output in conjunctions
                modules[output].input_states[key] = "low"
            end
        end
    end
   
    #println(modules)
    
    num_signals = Dict{String, Int}("low" => 0, "high" => 0)
    for i in 1:1000
        #println("########### Round $i")
        # Start new round with same first signal
        signals = [("button", "low", "broadcaster")]
        while length(signals) > 0
            signal = popfirst!(signals)
            #println(signal)
            from_key, pulse, to_key = signal
            num_signals[pulse] += 1
            if !(to_key in keys(modules))
                modules[to_key] = Untyped(to_key, [])
            end
            
            new_signals = receive(modules[to_key], from_key, pulse)
            if new_signals != nothing
                # Deal with no signals from one flip flop case
                signals = vcat(signals, new_signals)
            end
        end
    end

    println(num_signals["low"] * num_signals["high"])
end

function part_2(input)
    modules = Dict{String, Any}()
    conjunctions = Vector{String}()
    for line in split(input, "\n")
        rawKey, raw_outputs = split(line, " -> ")
        outputs = [String(s) for s in split(raw_outputs, ", ")]
        if startswith(rawKey, "%")
            key = String(rawKey[2:end])
            modules[key] = FlipFlop(key, false, outputs)
        elseif startswith(rawKey, "&")
            key = String(rawKey[2:end])
            modules[key] = Conjunction(key, outputs)
            conjunctions = vcat(conjunctions, key)
        elseif startswith(rawKey, "broadcaster")
            modules["broadcaster"] = Broadcast("broadcaster", outputs)
        else
            # Untyped case
            modules[key] = Untyped(rawKey, [])
        end
    end

     # Make sure all conjunction inputs are known
    for (key, mod) in pairs(modules)
        for output in mod.outputs
            if output in conjunctions
                modules[output].input_states[key] = "low"
            end
        end
    end
   
    num_presses = 0
    cycles = Dict{String, Int}()
    while (true)
        num_presses += 1
   
        # Start new round with same first signal
        signals = [("button", "low", "broadcaster")]
        while length(signals) > 0
            signal = popfirst!(signals)
            from_key, pulse, to_key = signal

            # Nodes to rx goes through gh which has 4 inputs which all need to
            # be low simultaneously
            if (to_key in ["zf", "qx", "rk", "cd"]) && (pulse == "low")
                if !(to_key in keys(cycles))
                    cycles[to_key] = num_presses
                end
                if length(cycles) == 4
                    println(prod(values(cycles)))
                    return
                end
            end

            if !(to_key in keys(modules))
                modules[to_key] = Untyped(to_key, [])
            end
            
            new_signals = receive(modules[to_key], from_key, pulse)
            if new_signals != nothing
                # Deal with no signals from one flip flop case
                signals = vcat(signals, new_signals)
            end
        end
    end
end

open("puzzle_20/example.txt") do example
    part_1(read(example, String))
end

open("puzzle_20/example_2.txt") do example
    part_1(read(example, String))
end

open("puzzle_20/input.txt") do input
    input = read(input, String)
    part_1(input)
    part_2(input)
end