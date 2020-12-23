require_relative "./lib"
require "set"

SOLUTIONS.update \
  "32658947" => [1, :in]

def main(input)
  cups = input.split(//).map(&:to_i)
  100.times do |i|
    p cups if i < 10
    play(cups, i%9, 9)
  end
  until (c = cups.shift) == 1
    cups.push c
  end
  p1done cups.join

  # ------

  return
  cups = input.split(//).map(&:to_i)
  cups += (10..1_000_000).to_a
  n = 0
  10_000_000.times do |i|
    n += 1
    print "." if n % 10 == 0
    cups = play(cups, i%1_000_000, 1_000_000)
  end
  p cups.first

  #p2done score(winner)

ensure
  bm_done
end

def play(cups, i, max)
  current = cups[i]
  pickedup_i = (1..3).map { |d| (i+d)%max }
  pickedup = pickedup_i.map { |j| cups[j] }
  destn = current - 1
  destn = max if destn == 0
  while pickedup.include?(destn)
    destn -= 1
    destn = max if destn == 0
  end
  j = cups.index(destn) or raise "boom"
  p cur: current, pup: pickedup, dest: destn, j: j
  if j > i
    moving = cups[(i+4)..j]
    p moving: moving
    cups[(i+1)..(j-3)] = cups[(i+4)..j]
    p insert1: cups, nxt: (j-2)..j
    cups[(j-2)..j] = pickedup
    p insert2: cups
  else
    cups[(i+1)..(max-3)] = cups[(i+4)..max]
    p insert1: cups
    cups[(max-3)..(max-j)] = cups[
    raise "boom"
  end
  #cups
  #raise "cur=[#{i}]#{current} dest=[#{j}]#{destn} [#{(i+1)..(j-1)}]"
  #rest.insert(i + 1, *pickedup)
  #rest << current
  #rest
end

main("389125467")
#main("598162734")
