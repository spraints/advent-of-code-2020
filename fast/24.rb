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
    #puts "==="
    dir = Directions.new(line)
    pos = [0,0]
    until dir.empty?
      pos = dir.step(pos)
      #p pos
    end
    pos = pos.take(2)
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
    n = neighbors(pos)
    bn, wn = n.partition { |npos| tiles[npos] }
    if bn.size == 1 || bn.size == 2
      new_tiles[pos] = true
    end
    white_neighbors += wn
  end
  white_neighbors.each do |pos|
    n = neighbors(pos)
    bn, _ = n.partition { |npos| tiles[npos] }
    if bn.size == 2
      new_tiles[pos] = true
    end
  end
  new_tiles
end

def neighbors(pos)
  x, y = pos
  dxs =
    if y % 2 == 0
      [-1, 0]
    else
      [0, 1]
    end
  dxs.inject([ [x-1,y], [x+1,y] ]) { |res, dx| res += [ [x+dx,y+1], [x+dx,y-1] ] }
end

class Directions
  def initialize(line)
    @chars = line.strip.chars
  end

  def empty?
    @chars.empty?
  end

  def step(pos)
    x, y, = pos
    case a = @chars.shift
    when "e"
      [x + 1, y, :east]
    when "w"
      [x - 1, y, :west]
    when "n"
      case b = @chars.shift
      when "e"
        if y % 2 == 0
          [x, y + 1, :ne]
        else
          [x + 1, y + 1, :ne]
        end
      when "w"
        if y % 2 == 1
          [x, y + 1, :nw]
        else
          [x - 1, y + 1, :nw]
        end
      else
        raise "unexpected #{b.inspect}"
      end
    when "s"
      case b = @chars.shift
      when "e"
        if y % 2 == 0
          [x, y - 1, :se]
        else
          [x + 1, y - 1, :se]
        end
      when "w"
        if y % 2 == 1
          [x, y - 1, :sw]
        else
          [x - 1, y - 1, :sw]
        end
      else
        raise "unexpected #{b.inspect}"
      end
    else
      raise "unexpected #{a.inspect}"
    end
  end
end

main($stdin.read)
