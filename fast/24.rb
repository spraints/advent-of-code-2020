require_relative "./lib"
require "set"

SOLUTIONS.update \
  10 => [1, :sample],
  300 => [1, :in],
  2208 => [2, :sample],
  3466 => [2, :in]

def main(input)

  tiles = Set.new
  input.lines.each do |line|
    dir = Directions.new(line)
    pos = ORIGIN
    until dir.empty?
      pos = dir.step(pos)
    end
    if tiles.include?(pos)
      tiles.delete(pos)
    else
      tiles.add(pos)
    end
  end

  p1done tiles.size

  # ---------------------------------

  100.times do |i|
    #bm "p2i#{i} #{tiles.size}"
    tiles = gol(tiles)
  end

  p2done tiles.size

ensure
  bm_done
end

if ENV["CUBE"]
  ORIGIN = [0, 0, 0].freeze # [x, y, z]
  E = [1, -1, 0].freeze
  W = [-1, 1, 0].freeze
  NE = [1, 0, -1].freeze
  NW = [0, 1, -1].freeze
  SE = [0, -1, 1].freeze
  SW = [-1, 0, 1].freeze
else
  ORIGIN = [0, 0].freeze # [q, r]
  E = [1, 0].freeze
  W = [-1, 0].freeze
  NE = [1, -1].freeze
  NW = [0, -1].freeze
  SE = [0, 1].freeze
  SW = [-1, 1].freeze
end
DIRS = [E, W, NE, NW, SE, SW].freeze

def gol(tiles)
  # Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
  # Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
  new_tiles = Set.new
  white_neighbors = Hash.new(0)
  tiles.each do |pos|
    bm_step
    n = neighbors(pos)
    bn, wn = n.partition { |npos| tiles.include?(npos) }
    if bn.size == 1 || bn.size == 2
      new_tiles.add(pos)
    end
    wn.each do |npos|
      bm_step
      white_neighbors[npos] += 1
    end
  end
  white_neighbors.each do |pos, count|
    bm_step
    if count == 2
      new_tiles.add(pos)
    end
  end
  new_tiles
end

def neighbors(pos)
  DIRS.map { |dir| bm_step; add(pos, dir) }
end

class Directions
  def initialize(line)
    @line = line.strip
  end

  def empty?
    @line.empty?
  end

  def step(pos)
    bm_step
    dir =
      case @line
      when /^se/
        @line = @line[2..-1]
        SE
      when /^sw/
        @line = @line[2..-1]
        SW
      when /^ne/
        @line = @line[2..-1]
        NE
      when /^nw/
        @line = @line[2..-1]
        NW
      when /^e/
        @line = @line[1..-1]
        E
      when /^w/
        @line = @line[1..-1]
        W
      else
        raise "invalid #{@line.inspect}"
      end
    add(pos, dir)
  end
end

def add(pos, dir)
  pos.zip(dir).map { |a, b| a + b }
end

main($stdin.read)
