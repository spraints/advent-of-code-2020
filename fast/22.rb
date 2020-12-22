require_relative "./lib"
require "set"

SOLUTIONS.update \
  306 => [1, :sample],
  291 => [2, :sample],
  32489 => [1, :in],
  35676 => [2, :in]

def main(input)
  player1, player2 = read_decks(input.lines)
  bm_size player1.size

  dupes = player1 & player2
  raise dupes.inspect unless dupes.empty?

  _, winner = play(player1, player2)
  p1done score(winner)

  # ------

  player1, player2 = read_decks(input.lines)

  _, winner = play(player1, player2, rec: true)
  p2done score(winner)

ensure
  bm_done
end

def score(winner)
  winner.reverse.zip((1..winner.size).to_a).map { |a, b| a * b }.sum
end

def play(player1, player2, level: 0, rec: false)
  seen = Set.new
  until player1.empty? || player2.empty?
    bm_step

    key = Marshal.dump([player1, player2])
    if seen.include?(key)
      return [:p1, player1]
    end
    seen.add(key)

    p1 = player1.shift
    p2 = player2.shift
    winner = 
      if rec && p1 <= player1.size && p2 <= player2.size
        play(player1.take(p1), player2.take(p2), level: level + 1, rec: true).first
      else
        p1 > p2 ? :p1 : :p2
      end
    if winner == :p1
      player1 += [p1, p2]
    else
      player2 += [p2, p1]
    end
  end
  if player1.empty?
    [:p2, player2]
  else
    [:p1, player1]
  end
end

def read_decks(lines)
  player1 = []
  lines.shift
  loop do
    line = lines.shift.to_i
    break if line == 0
    player1 << line
  end
  lines.shift
  player2 = lines.map(&:to_i)
  [player1, player2]
end

main($stdin.read)
