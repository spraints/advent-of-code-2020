require_relative "./lib"

def main(input)
  bm "parse"

  input = input.lines.map { |line| line =~ /(.)(\d+)/ && [$1, $2.to_i] }

  bm "part 1"

  puts "part 1: #{move(input)} should be 319"

  bm "part 2"

  puts "part2: #{move2(input)} should be 50157"

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
  dx, dy = 1, 0
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
      dx, dy = rot(dx, dy, 360-d)
    when "R"
      dx, dy = rot(dx, dy, d)
    when "F"
      x += d * dx
      y += d * dy
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
      dx, dy = rot(dx, dy, 360-d)
    when "R"
      dx, dy = rot(dx, dy, d)
    when "F"
      x += d * dx
      y += d * dy
    end
  end
  x.abs + y.abs
end

def rot(x, y, deg)
  case deg
  when 90
    [y, -x]
  when 180
    [-x, -y]
  when 270
    [-y, x]
  else
    raise "rot #{deg}"
  end
end

main($stdin.read)
