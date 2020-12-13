require_relative "./lib"

def main(input)
  bm "parse"

  earliest_departure, bus_ids = input.lines
  earliest_departure = earliest_departure.to_i
  bus_ids = bus_ids.split(",")

  bm "part 1"

  bus_id, wait = bus_ids.reject { |id| id == "x" }.map(&:to_i).map { |bus_id| [bus_id, bus_id - earliest_departure % bus_id] }.sort_by { |_, x| x }.first
  p bus_id => wait
  puts "part 1: #{bus_id * wait}"
  #puts "part 1: #{move(input)} should be 319"

  bm "part 2"

  # (n +  0) %  19 == 0
  # (n +  9) %  41 == 0
  # (n + 19) % 859 == 0
  # (n + 27) %  23 == 0
  # (n + 32) %  13 == 0
  # (n + 36) %  17 == 0
  # (n + 48) %  29 == 0
  # (n + 50) % 373 == 0
  # (n + 87) %  37 == 0

##  n = 0
##  steps = 0
##  bus_ids = bus_ids.map { |x| x == "x" ? nil : x.to_i }
##  loop do
##    puts "#{n}..." if steps % 1_000_000 == 0
##    #if n == 1068781
##    #  bus_ids.each_with_index do |b, i|
##    #    if b
##    #      p i: i, b: b, m: (n + i) % b
##    #    end
##    #  end
##    #end
##    break if bus_ids.each_with_index.all? { |bus_id, i| bus_id.nil? || 0 == (n + i) % bus_id }
##    n += bus_ids[0]
##    steps += 1
##  end

  checks = bus_ids.each_with_index.map { |bus_id, i| bus_id == "x" ? nil : [bus_id.to_i, i] }.compact
##  p checks
##  step_size, offset = checks.sort_by(&:first).last
##  n = step_size - offset
##  steps = 0
##  loop do
##    puts "#{n}..." if steps % 1_000_000 == 0
##    break if checks.all? { |bus_id, offset| 0 == (n + offset) % bus_id }
##    n += step_size
##    steps += 1
##  end
##
##  puts "part 2: #{n}"
##  if n == 1068781
##    puts " ... SAMPLE!"
##  end

  # 1068781 / 7 = 152683 R 0
  #

  # X == I (mod B)
  # X == 0 (mod 7)  ->  X      =  7*e
  # X == 1 (mod 13) ->  X +  1 = 13*d
  # X == 4 (mod 59) ->  X +  4 = 59*c
  # X == 6 (mod 31) ->  X +  6 = 31*b
  # X == 7 (mod 19) ->  X +  7 = 19*a
  #                 -> 5X + 18 = 7*e + 13*d + 59*c + 31*b + 19*a
  # (x**n + I) mod B == 0
#  n = 23834237830
#  checks.each do |bus_id, offset|
#    puts "(#{n} + #{offset}) % #{bus_id} == #{(n + offset) % bus_id}"
#  end

  maybe_x = nil
  step = 1
  i = 0
  # https://en.wikipedia.org/wiki/Chinese_remainder_theorem
  checks.sort_by { |bus_id, _| -bus_id }.each do |nk, offset|
    i += 1
    ak = (nk - offset) % nk
    p ak: ak, nk: nk, offset: offset
    if maybe_x.nil?
      maybe_x = ak
      step = nk
    else
      until ak == maybe_x % nk
        i += 1
        maybe_x += step
      end
      step *= nk
    end
    p maybe_x: maybe_x, step: step
  end

  puts "part 2: #{maybe_x} (#{i})"

  bm_done
end

main($stdin.read)
