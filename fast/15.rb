require_relative "./lib"

def main(input)
  bm "parse"

  input = input.lines.first.strip.split(",").map(&:to_i)

  bm "part 1"

  init = input.dup
  last = nil
  said = {}
  1.upto(2020) do |turn|
    n =
      if x = init.shift
        x
      elsif x = said[last]
        turn - 1 - x
      else
        0
      end
    said[last] = turn - 1 if last
    last = n
  end
  puts "part 1: #{last}"

  bm "part 2"

  init = input.dup
  last = nil
  said = {}
  1.upto(30000000) do |turn|
    n =
      if x = init.shift
        x
      elsif x = said[last]
        turn - 1 - x
      else
        0
      end
    said[last] = turn - 1 if last
    last = n
  end
  puts "part 1: #{last}"

ensure
  bm_done
end

main($stdin.read)
