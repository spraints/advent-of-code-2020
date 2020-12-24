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
    pos = [0,0,0]
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
  x, y, z = pos
  [
    [1, -1, 0],
    [-1, 1, 0],
    [1, 0, -1],
    [-1, 0, 1],
    [0, 1, -1],
    [0, -1, 1],
  ].map { |dx, dy, dz|
    [x+dx, y+dy, z+dz]
  }
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
    x, y, z = pos
    dx, dy, dz =
      case a = @chars.shift
      when "e"
        [1, -1, 0]
      when "w"
        [-1, 1, 0]
      when "n"
        case b = @chars.shift
        when "e"
          [1, 0, -1]
        when "w"
          [0, 1, -1]
        else
          raise "unexpected #{b.inspect}"
        end
      when "s"
        case b = @chars.shift
        when "e"
          [0, -1, 1]
        when "w"
          [-1, 0, 1]
        else
          raise "unexpected #{b.inspect}"
        end
      else
        raise "unexpected #{a.inspect}"
      end
    [x+dx, y+dy, z+dz]
  end
end

main($stdin.read)
