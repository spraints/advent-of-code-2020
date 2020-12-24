require_relative "./lib"
require "set"

SOLUTIONS.update \
  10 => [1, :sample],
  300 => [1, :in],
  2208 => [2, :sample],
  3466 => [2, :in]

def main(input)

  tiles = {}
  input.lines.each do |line|
    dir = Directions.new(line)
    pos = ORIGIN
    until dir.empty?
      pos = dir.step(pos)
    end
    tiles[pos] = !tiles[pos]
  end

  p1done tiles.values.select { |x| x }.size

  # ---------------------------------

  100.times do
    tiles = gol(tiles)
  end

  p2done tiles.values.select { |x| x }.size

ensure
  bm_done
end

ORIGIN = [0, 0, 0].freeze
E = [1, -1, 0].freeze
W = [-1, 1, 0].freeze
NE = [1, 0, -1].freeze
NW = [0, 1, -1].freeze
SE = [0, -1, 1].freeze
SW = [-1, 0, 1].freeze
DIRS = [E, W, NE, NW, SE, SW].freeze

def gol(tiles)
  # Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
  # Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
  new_tiles = {}
  black_tiles = tiles.select { |_, v| v }.map(&:first)
  white_neighbors = []
  black_tiles.each do |pos|
    bm_step
    n = neighbors(pos)
    bn, wn = n.partition { |npos| tiles[npos] }
    if bn.size == 1 || bn.size == 2
      new_tiles[pos] = true
    end
    white_neighbors += wn
  end
  white_neighbors.each do |pos|
    bm_step
    n = neighbors(pos)
    bn, _ = n.partition { |npos| tiles[npos] }
    if bn.size == 2
      new_tiles[pos] = true
    end
  end
  new_tiles
end

def neighbors(pos)
  DIRS.map { |dir| add(pos, dir) }
end

class Directions
  def initialize(line)
    @chars = line.strip.chars
  end

  def empty?
    @chars.empty?
  end

  def step(pos)
    bm_step
    dir =
      case a = @chars.shift
      when "e"
        E
      when "w"
        W
      when "n"
        case b = @chars.shift
        when "e"
          NE
        when "w"
          NW
        else
          raise "unexpected #{b.inspect}"
        end
      when "s"
        case b = @chars.shift
        when "e"
          SE
        when "w"
          SW
        else
          raise "unexpected #{b.inspect}"
        end
      else
        raise "unexpected #{a.inspect}"
      end
    add(pos, dir)
  end
end

def add(pos, dir)
  pos.zip(dir).map { |a, b| a + b }
end

main($stdin.read)
