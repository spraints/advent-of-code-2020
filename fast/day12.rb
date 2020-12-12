require_relative "./lib"

def main(input)
  bm "parse"

  input = input.lines.map { |line| line =~ /(.)(\d+)/ && [$1, $2.to_i] }

  bm "part 1"

  puts "part 1: #{move(input)}"
  #puts "part 1: #{occupied} (should be 2211)"

  bm "part 2"

  puts "part2: #{move2(input)} < 56981 < 74119"

  bm_done
end

# Action N means to move north by the given value.
# Action S means to move south by the given value.
# Action E means to move east by the given value.
# Action W means to move west by the given value.
# Action L means to turn left the given number of degrees.
# Action R means to turn right the given number of degrees.
# Action F means to move forward by the given value in the direction the ship is currently facing.
def move(steps)
  x, y = 0, 0
  dir = 90
  steps.each do |t, d|
    case t
    when "N"
      y += d
    when "S"
      y -= d
    when "E"
      x += d
    when "W"
      x -= d
    when "L"
      dir = (dir + 360 - d) % 360
    when "R"
      dir = (dir + d) % 360
    when "F"
      case dir
      when 0
        y += d
      when 90
        x += d
      when 180
        y -= d
      when 270
        x -= d
      else
        raise "dir #{dir}"
      end
    end
  end
  x.abs + y.abs
end

# Action N means to move the waypoint north by the given value.
# Action S means to move the waypoint south by the given value.
# Action E means to move the waypoint east by the given value.
# Action W means to move the waypoint west by the given value.
# Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
# Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
# Action F means to move forward to the waypoint a number of times equal to the given value.
def move2(steps)
  x, y = 0, 0
  dx, dy = 10, 1
  steps.each do |t, d|
    #p p: [x,y], w: [dx, dy]
    #p [t, d]
    case t
    when "N"
      dy += d
    when "S"
      dy -= d
    when "E"
      dx += d
    when "W"
      dx -= d
    when "L"
      p w: [dx, dy], L: d
      dx, dy = rot(dx, dy, 360-d)
      p w: [dx, dy]
    when "R"
      p w: [dx, dy], R: d
      dx, dy = rot(dx, dy, d)
      p w: [dx, dy]
    when "F"
      x += d * dx
      y += d * dy
    end
  end
  x.abs + y.abs
end

def rot(x, y, deg)
  p deg: deg, x: x, y: y
  case deg
  when 90
    if x > 0
      if y > 0
        [y, -x]
      else
        [y, -x]
      end
    else
      if y < 0
        [y, -x]
      else
        [y, -x]
      end
    end
  when 180
    [-x, -y]
  when 270
    if x > 0
      if y > 0
        [-y, x]
      else
        [-y, x]
      end
    else
      if y < 0
        [-y, x]
      else
        [-y, x]
      end
    end
  else
    raise "rot #{deg}"
  end
end

main($stdin.read)
