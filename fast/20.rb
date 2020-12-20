require_relative "./lib"

def main(input)
  bm "parse"

  tiles = parse_tiles(input)
  #p tiles

  bm "part 1"
  sols = {
    22878471088273 => "IN!",
    20899048083289 => "SAMPLE!",
  }

  dim = Math.sqrt(tiles.size).to_i
  filled = fill([], dim: dim, tiles: tiles, row: 0, col: 0, used: {})
  corners = [
    filled[0][0].first,
    filled[0][-1].first,
    filled[-1][-1].first,
    filled[-1][0].first,
  ]
  part1 = corners.map(&:to_i).inject(&:*)
  puts "part 1: #{part1} #{sols[part1]}"

  bm "part 2"
  sols2 = {
    273 => "SAMPLE!",
    1680 => "IN!",
  }

  image = filled.flat_map { |row| render(row, trim: true) }
  each_rotation2(image) do |i|
    if res = color_monsters(i)
      _, found = res
      part2 = i.inject(-found.size * MONSTER.size) { |sum, row| sum + row.scan(/#/).size }
      puts "part 2: #{part2} #{sols2[part2]}"
    end
  end

ensure
  bm_done
end

#             1111111111
#   01234567890123456789
# 0:                  # 
# 1:#    ##    ##    ###
# 2: #  #  #  #  #  #   
MONSTER = [
  [0, 18],
  [1, 0],
  [1, 5],
  [1, 6],
  [1, 11],
  [1, 12],
  [1, 17],
  [1, 18],
  [1, 19],
  [2, 1],
  [2, 4],
  [2, 7],
  [2, 10],
  [2, 13],
  [2, 16],
]
def color_monsters(img)
  dim = img.size
  row = 0
  found = []
  while row + 3 < dim
    col = 0
    while col + 19 < dim
      if MONSTER.all? { |r,c| img[row + r][col + c] == "#" }
        found << [row, col]
      end
      col += 1
    end
    row += 1
  end
  return nil if found.empty?
  [img, found]
  #found.each do |row, col|
  #  MONSTER.each do |r, c|
  #    img[row + r][col + c] = "O"
  #  end
  #end
  #img
end

def render(row, trim:)
  tiles = trim ?
    row.map { |_, tile, _| trim_edges(orient(**tile)) } :
    row.map { |_, tile, _| orient(**tile) }

  t = tiles.shift
  res = t.zip(*tiles).map { |line| line.join(trim ? "" : " ") }
  res << [""] unless trim
  res
end

def trim_edges(raw_tile)
  raw_tile[1..-2].map { |line| line[1..-2] }
end

def fill(filled, dim:, tiles:, row:, col:, used:)
  return filled if row == dim
  if col < dim
    tiles.each do |id, tile|
      if !used.key?(id)
        each_rotation(tile[:edges]) do |rtile, rot|
          if can_place?(grid: filled, tile: rtile, row: row, col: col)
            if res = fill(add_to_grid(filled, [id, tile.merge(rot), rtile], row: row, col: col), dim: dim, tiles: tiles, row: row, col: col + 1, used: used.merge(id => true))
              return res
            end
          end
        end
      end
    end
    nil
  elsif row < dim
    fill(filled, dim: dim, tiles: tiles, row: row + 1, col: 0, used: used)
  else
    raise "unexpected"
  end
end

def each_rotation(tile)
  tile = tile.dup
  4.times do |i|
    # rot counter-clockwise
    yield tile, {step: i, rev: false}
    tile.push tile.shift
  end
  # flip top-right to bottom-left
  tile = tile.map(&:reverse).reverse
  4.times do |i|
    # rot counter-clockwise
    yield tile, {step: i, rev: true}
    tile.push tile.shift
  end
end

def each_rotation2(raw)
  4.times do |i|
    yield raw
    # rot counter-clockwise
    raw = raw.size.times.map { |i| raw.map { |row| row[i] }.join }.reverse
  end
  # flip top-right to bottom-left
  raw = raw.size.times.map { |i| raw.map { |row| row[i] }.join }
  4.times do |i|
    yield raw
    # rot counter-clockwise
    raw = raw.size.times.map { |i| raw.map { |row| row[i] }.join }.reverse
  end
end

def orient(raw:, step:, rev:, **)
  4.times do |i|
    return raw if i == step && !rev
    # rot counter-clockwise
    raw = raw.size.times.map { |i| raw.map { |row| row[i] }.join }.reverse
  end
  # flip top-right to bottom-left
  raw = raw.size.times.map { |i| raw.map { |row| row[i] }.join }
  4.times do |i|
    return raw if i == step
    # rot counter-clockwise
    raw = raw.size.times.map { |i| raw.map { |row| row[i] }.join }.reverse
  end
end

def can_place?(grid:, tile:, row:, col:)
  if col > 0
    neighbor = grid[row][col-1].last
    nedge = neighbor[1]
    tedge = tile[3].reverse
    if nedge != tedge
      return false
    end
  end
  if row > 0
    neighbor = grid[row-1][col].last
    nedge = neighbor[2]
    tedge = tile[0].reverse
    if nedge != tedge
      return false
    end
  end
  true
end

def add_to_grid(grid, tile, row:, col:)
  grid = grid.dup
  grid[row] = grid[row]&.dup || []
  grid[row][col] = tile
  grid
end

def parse_tiles(input)
  lines = input.lines
  res = {}
  until lines.empty?
    lines.shift =~ /Tile (\d+)/ or raise "oops"
    tile_id = $1
    tile = []
    while line = lines.shift
      line = line.strip
      break if line.empty?
      tile << line
    end
    res[tile_id] = {edges: get_edges(tile), raw: tile}
  end
  res
end

def get_edges(tile)
  [tile.first, tile.map { |s| s[-1] }.join, tile.last.reverse, tile.map { |s| s[0] }.join.reverse]
end

main($stdin.read)
