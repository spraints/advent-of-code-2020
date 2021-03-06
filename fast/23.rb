require_relative "./lib"
require "set"

SOLUTIONS.update \
  "32658947" => [1, :in],
  "67384529" => [1, :sample],
  149245887792 => [2, :sample],
  683486010900 => [2, :in]

def main(input)
  cups = Cups.new(input.split(//).map(&:to_i))
  100.times do |i|
    #p cups.to_a
    cups.play
  end
  cups = cups.to_a
  until (c = cups.shift) == 1
    cups.push c
  end
  p1done cups.join

  # ------

  cups = Cups.new(input.split(//).map(&:to_i) + (10..1_000_000).to_a)
  t = Time.now.to_f
  n = 0
  10_000_000.times do |i|
    n += 1
    if n % 500_000 == 0
      t2 = Time.now.to_f
      m = (1000.0*(t2 - t)).to_i
      print ".#{m}"
      t = t2
    end
    cups.play
  end

  a, b = cups.stars
  p2done a*b

ensure
  bm_done
end

class Cups
  def initialize(cups)
    @max = cups.max
    @nodes = []
    cups.each_cons(2) { |a,b| init_node(a).nxt = init_node(b) }
    @nodes[cups.last].nxt = @nodes[cups.first]
    @current = @nodes[cups.first]
  end

  def init_node(val)
    @nodes[val] ||= Node.new(val)
  end

  def node(val)
    @nodes[val]
  end

  def play
    pick = @current.nxt
    pick2 = pick.nxt
    pick3 = pick2.nxt
    #p picked: [pick[:val], pick2[:val], pick3[:val]]
    rest = pick3.nxt

    dest = @current.val - 1
    dest = @max if dest == 0
    while pick.val == dest || pick2.val == dest || pick3.val == dest
      dest = dest == 1 ? @max : dest - 1
    end
    #p dest: dest

    @current.nxt = rest
    #p n: {val: @nodes[dest][:val], nxt: @nodes[dest][:next][:val]}
    pick3.nxt = @nodes[dest].nxt
    @nodes[dest].nxt = pick

    @current = rest
  end

  def stars
    a = @nodes[1].nxt.val
    b = @nodes[1].nxt.nxt.val
    [a, b]
  end

  def to_a
    res = []
    cur = @current
    loop do
      res << cur.val
      cur = cur.nxt
      break if cur == @current
    end
    res
  end
end

class Node
  def initialize(val)
    @val = val
  end

  attr_reader :val
  attr_accessor :nxt
end

#main("389125467")
main("598162734")
