require_relative "./lib"

def main(input)
  bm "parse"

  tiles = parse_tiles(input)
  #p tiles

  bm "part 1"

  dim = Math.sqrt(tiles.size).to_i
  filled = fill([], dim: dim, tiles: tiles, row: 0, col: 0, used: {})
  filled.each do |row|
    row.each do |col|
      p col
    end
    puts "--"
  end
  corners = [
    filled[0][0].first,
    filled[0][-1].first,
    filled[-1][-1].first,
    filled[-1][0].first,
  ]
  p corners
  puts "part 1: #{corners.map(&:to_i).inject(&:*)}"

  bm "part 2"

  #puts "part 3: #{messages.count { |m| rules_match?(rules, m.strip.chars) }}"

ensure
  bm_done
end

def fill(filled, dim:, tiles:, row:, col:, used:)
  return filled if row == dim
  p fill: [row, col]
  if col < dim
    tiles.each do |id, tile|
      if !used.key?(id)
        puts "[#{row},#{col}] #{id}? (used: #{used.keys.inspect})"
        each_rotation(tile) do |rtile|
          if can_place?(grid: filled, tile: rtile, row: row, col: col)
            puts "[#{row},#{col}] #{id} ==> YES"
            if res = fill(add_to_grid(filled, [id, rtile], row: row, col: col), dim: dim, tiles: tiles, row: row, col: col + 1, used: used.merge(id => true))
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
  4.times do
    yield tile
    tile.push tile.shift
  end
  tile = tile.map(&:reverse).reverse
  4.times do
    yield tile
    tile.push tile.shift
  end
end

def can_place?(grid:, tile:, row:, col:)
  if col > 0
    neighbor = grid[row][col-1].last
    nedge = neighbor[1]
    tedge = tile[3].reverse
    p left: [nedge, tedge]
    if nedge != tedge
      return false
    end
  end
  if row > 0
    neighbor = grid[row-1][col].last
    nedge = neighbor[2]
    tedge = tile[0].reverse
    p up: [nedge, tedge]
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
    res[tile_id] = get_edges(tile)
  end
  res
end

def get_edges(tile)
  [tile.first, tile.map { |s| s[-1] }.join, tile.last.reverse, tile.map { |s| s[0] }.join.reverse]
end

main($stdin.read)
