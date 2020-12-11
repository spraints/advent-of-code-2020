require_relative "./lib"

def main(input)
  bm "parse"

  input = input.lines.map(&:strip).map(&:chars)

  bm "part 1"

  cur = input
  n = 0
  loop do
    n += 1
    step = fill_seats(cur, tol: 4) { |i, j| occupied_neighbors(cur, i, j) }
    break if step == cur
    cur = step
  end

  occupied = cur.flatten.grep("#").size
  puts "part 1: #{occupied}"

  bm "part 2"

  cur = input
  n = 0
  loop do
    #puts "STEP"
    n += 1
    step = fill_seats(cur, tol: 5) { |i, j| visible_occupied(cur, i, j) }
    #pc step
    break if step == cur
    cur = step
  end

  occupied = cur.flatten.grep("#").size
  puts "part 2: #{occupied}"

  bm_done
end

def pc(chart)
  puts chart.map(&:join), "\n"
  sleep 1
end

def fill_seats(chart, tol:)
  step = chart.map(&:dup)
  chart.each_with_index do |line, i|
    line.each_with_index do |seat, j|
      next if seat == "."
      occ = yield(i, j)
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

def occupied_neighbors(chart, i, j)
  occ = 0
  ((i-1)..(i+1)).each do |ii|
    ((j-1)..(j+1)).each do |jj|
      next if ii < 0
      next if jj < 0
      next if ii == i && jj == j
      next unless ii < chart.size
      next unless jj < chart[ii].size
      occ += 1 if chart[ii][jj] == "#"
    end
  end
  occ
end

def visible_occupied(chart, i, j)
  occ = 0
  [
    [-1, -1],
    [0, -1],
    [1, -1],
    [1, 0],
    [1, 1],
    [0, 1],
    [-1, 1],
    [-1, 0],
  ].each do |si, sj|
    ci, cj = i + si, j + sj
    while ci >= 0 && cj >= 0 && ci < chart.size && cj < chart[ci].size
      case chart[ci][cj]
      when "L"
        break
      when "#"
        #puts "[#{i},#{j}] => # @ [#{ci}, #{cj}] ([#{si}, #{sj}])"
        occ += 1
        break
      end
      ci, cj = ci + si, cj + sj
    end
  end
  occ
end

main($stdin.read)
