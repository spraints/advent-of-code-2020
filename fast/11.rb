require_relative "./lib"

def main(input)
  bm_size input.lines.size

  input = input.lines.map(&:strip).map(&:chars)

  cur = input
  n = 0
  loop do
    n += 1
    step = fill_seats(cur, tol: 4, part2: false)
    break if step == cur
    cur = step
  end

  occupied = cur.flatten.grep("#").size
  puts "part 1: #{occupied} (should be 2211)"

  bm "part 2"

  cur = input
  n = 0
  loop do
    #puts "STEP"
    n += 1
    step = fill_seats(cur, tol: 5, part2: true)
    #pc step
    break if step == cur
    cur = step
  end

  occupied = cur.flatten.grep("#").size
  puts "part 2: #{occupied} (should be 1995)"

  bm_done
end

def pc(chart)
  puts chart.map(&:join), "\n"
  sleep 1
end

def fill_seats(chart, tol:, part2:)
  step = chart.map(&:dup)
  chart.each_with_index do |line, i|
    line.each_with_index do |seat, j|
      bm_step
      next if seat == "."
      occ = neighbors(chart, i, j, skip_empty_space: part2)
      #puts "#{i},#{j}=#{seat} #{occ}"
      if seat == "L" && occ == 0
        step[i][j] = "#"
      elsif seat == "#" && occ >= tol
        step[i][j] = "L"
      end
    end
  end
  step
end

DIRECTIONS = [
  [-1, -1],
  [0, -1],
  [1, -1],
  [1, 0],
  [1, 1],
  [0, 1],
  [-1, 1],
  [-1, 0],
]

def neighbors(chart, x, y, skip_empty_space: false)
  maxx = chart.size
  maxy = chart[x].size
  count = 0
  DIRECTIONS.each do |dx, dy|
    nx, ny = x + dx, y + dy
    while nx >= 0 && nx < maxx && ny >= 0 && ny < maxy
      bm_step
      case chart[nx][ny]
      when "L"
        break
      when "#"
        count += 1
        break
      when "."
        break unless skip_empty_space
      end
      nx, ny = nx + dx, ny + dy
    end
  end
  count
end

main($stdin.read)
