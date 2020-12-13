require_relative "./lib"

def main(input)
  bm "parse"

  earliest_departure, bus_ids = input.lines
  earliest_departure = earliest_departure.to_i
  bus_ids = bus_ids.split(",")

  bm "part 1"

  bus_id, wait = bus_ids.reject { |id| id == "x" }.map(&:to_i).map { |bus_id| [bus_id, bus_id - earliest_departure % bus_id] }.sort_by { |_, x| x }.first
  puts "part 1: #{bus_id * wait}"

  bm "part 2"

  checks = bus_ids.each_with_index.map { |bus_id, i| bus_id == "x" ? nil : [bus_id.to_i, i] }.compact

  maybe_x = nil
  step = 1
  i = 0
  # https://en.wikipedia.org/wiki/Chinese_remainder_theorem
  checks.sort_by { |bus_id, _| -bus_id }.each do |nk, offset|
    i += 1
    ak = (nk - offset) % nk
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
  end

  puts "part 2: #{maybe_x} (#{i})"

  bm_done
end

main($stdin.read)
