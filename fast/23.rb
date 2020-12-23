require_relative "./lib"
require "set"

SOLUTIONS.update \
  "32658947" => [1, :in]

def main(input)
  cups = input.split(//).map(&:to_i)
  100.times do 
    #p cups
    cups = play(cups, 9)
  end
  until (c = cups.shift) == 1
    cups.push c
  end
  p1done cups.join

  # ------

  cups = input.split(//).map(&:to_i)
  cups += (10..1_000_000).to_a
  n = 0
  10_000_000.times do
    n += 1
    print "." if n % 10_000 == 0
    cups = play(cups, 1_000_000)
  end
  p cups.first

  #p2done score(winner)

ensure
  bm_done
end

def play(cups, max)
  current = cups.shift
  pickedup = cups.take(3)
  rest = cups.drop(3)
  destn = current - 1
  destn = max if destn == 0
  while pickedup.include?(destn)
    destn -= 1
    destn = max if destn == 0
  end
  i = rest.index(destn) or raise "boom"
  rest.insert(i + 1, *pickedup)
  rest << current
  rest
end

#main("389125467")
main("598162734")
