require_relative "./lib"
require "set"

SOLUTIONS.update \
  10 => [1, :sample],
  300 => [1, :in]
#  "32658947" => [1, :in],
#  149245887792 => [2, :sample],
#  683486010900 => [2, :in]

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

  #p1done cups.join

  # ---------------------------------

  #p2done a*b

ensure
  bm_done
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
