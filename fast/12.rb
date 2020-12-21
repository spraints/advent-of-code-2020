require_relative "./lib"

def main(input)
  input = input.lines.map { |line| line =~ /(.)(\d+)/ && [$1, $2.to_i] }

  puts "part 1: #{move(input)} should be 319"

  bm "part 2"

  puts "part2: #{move2(input)} should be 50157"

  bm_done
end

N = Complex(0,1)
S = Complex(0,-1)
E = Complex(1,0)
W = Complex(-1,0)
L = Complex(0,1)
R = Complex(0,-1)

# Action N means to move north by the given value.
# Action S means to move south by the given value.
# Action E means to move east by the given value.
# Action W means to move west by the given value.
# Action L means to turn left the given number of degrees.
# Action R means to turn right the given number of degrees.
# Action F means to move forward by the given value in the direction the ship is currently facing.
def move(steps)
  pos = 0.to_c
  dir = 1.to_c
  steps.each do |t, d|
    case t
    when "N"
      pos += N*d
    when "S"
      pos += S*d
    when "E"
      pos += E*d
    when "W"
      pos += W*d
    when "L"
      dir *= L**(d/90)
    when "R"
      dir *= R**(d/90)
    when "F"
      pos += dir*d
    end
  end
  pos.rect.map(&:abs).sum
end

# Action N means to move the waypoint north by the given value.
# Action S means to move the waypoint south by the given value.
# Action E means to move the waypoint east by the given value.
# Action W means to move the waypoint west by the given value.
# Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
# Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
# Action F means to move forward to the waypoint a number of times equal to the given value.
def move2(steps)
  pos = 0.to_c
  way = Complex(10,1)
  steps.each do |t, d|
    case t
    when "N"
      way += N*d
    when "S"
      way += S*d
    when "E"
      way += E*d
    when "W"
      way += W*d
    when "L"
      way *= L**(d/90)
    when "R"
      way *= R**(d/90)
    when "F"
      pos += way*d
    end
  end
  pos.rect.map(&:abs).sum
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
