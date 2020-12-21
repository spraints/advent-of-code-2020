require_relative "./lib"

def main(input)
  earliest_departure, bus_ids = input.lines
  earliest_departure = earliest_departure.to_i
  bus_ids = bus_ids.split(",")

  bm_size bus_ids.size

  bus_id, wait = bus_ids.reject { |id| bm_step; id == "x" }.map(&:to_i).map { |bus_id| bm_step; [bus_id, bus_id - earliest_departure % bus_id] }.sort_by { |_, x| x }.first
  puts "part 1: #{bus_id * wait}"

  bm "part 2"

  checks = bus_ids.each_with_index.map { |bus_id, i| bm_step; bus_id == "x" ? nil : [bus_id.to_i, i] }.compact

  maybe_x = nil
  step = 1
  # https://en.wikipedia.org/wiki/Chinese_remainder_theorem
  checks.sort_by { |bus_id, _| -bus_id }.each do |nk, offset|
    ak = (nk - offset) % nk
    if maybe_x.nil?
      bm_step
      maybe_x = ak
      step = nk
    else
      until ak == maybe_x % nk
        bm_step
        maybe_x += step
      end
      step *= nk
    end
  end

  puts "part 2: #{maybe_x}"

  bm_done
end

main($stdin.read)
