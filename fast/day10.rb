require_relative "./lib"

def main(input)
  bm "parse"

  input = input.split.map(&:to_i)
  input << 0
  input << input.max + 3

  #preamble = ARGV.first.to_i
  #preamble = 25 if preamble < 1

  bm "part 1"

  steps = Hash.new { |h,k| h[k] = 0 }
  input = input.sort
  input.each_cons(2) do |a,b|
    steps[b - a] += 1
  end
  puts "part 1: #{steps[1] * steps[3]}"

  bm "part 2"

  next_steps = {}
  j = 0
  input.each_with_index do |x, i|
    while j < input.size && input[j] <= x + 3
      j += 1
    end
    next_steps[x] = input.slice((i+1)..(j-1))
  end
  choices = {}
  input.reverse.each do |x|
    s = next_steps[x]
    if s.empty?
      choices[x] = 1
    else
      choices[x] = s.map { |y| choices[y] }.sum
    end
  end
  puts "part2: #{choices[0]}"

  bm_done
end

main($stdin.read)
