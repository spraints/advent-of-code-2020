require_relative "./lib"

SOLUTIONS.update \
  880 => [1, :in],
  731 => [2, :in]

def main(input)
  seats = {}
  input.lines.each do |line|
    seat_id = line.tr('FL', '0').tr('BR', '1').to_i(2)
    seats[seat_id] = true
  end

  max = seats.keys.max
  p1done max

  cur = max
  loop do
    if !seats[cur]
      p2done cur
      break
    end
    cur -= 1
  end

ensure
  bm_done
end

main($stdin.read)
