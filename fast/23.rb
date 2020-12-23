require_relative "./lib"
require "set"

#SOLUTIONS.update \

def main(input)
  cups = input.split(//).map(&:to_i)
  100.times do 
    #p cups
    cups = play(cups)
  end
  until (c = cups.shift) == 1
    cups.push c
  end
  p1done cups.join

  #p1done score(winner)

  cups = input.split(//).map(&:to_i)
  cups += (10..1_000_000).to_a
  n = 0
  10_000_000.times do
    n += 1
    print "." if n % 10_000 == 0
    cups = play(cups)
  end
  p cups.first
  # ------

  #p2done score(winner)

ensure
  bm_done
end

def play(cups)
  m = cups.max
  current = cups.shift
  pickedup = cups.take(3)
  rest = cups.drop(3)
  #p [current, pickedup, rest]
  current.downto(0) do |n|
    if i = rest.index(n)
      rest.insert(i + 1, *pickedup)
      rest << current
      return rest
    end
  end
  m.downto(0) do |n|
    if i = rest.index(n)
      rest.insert(i + 1, *pickedup)
      rest << current
      return rest
    end
  end
  raise "boom"
end

#main("389125467")
main("598162734")
