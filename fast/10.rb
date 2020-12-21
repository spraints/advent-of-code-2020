require_relative "./lib"

def main(input)
  input = input.split.map(&:to_i)
  input << 0
  input << input.max + 3

  #preamble = ARGV.first.to_i
  #preamble = 25 if preamble < 1

  steps = Hash.new { |h,k| h[k] = 0 }
  input = input.sort
  input.each_cons(2) do |a,b|
    steps[b - a] += 1
  end
  puts "part 1: #{steps[1] * steps[3]}"

  bm "part 2"

  puts "part 2: #{DP.new(input).dp(0)}"

  bm_done
end

class DP
  def initialize(input)
    @input = input
    @dp = {}
    @n = 0
  end

  def dp(i)
    return @dp[i] if @dp[i]
    j = i + 1
    return 1 unless j < @input.size
    ways = 0
    lim = @input[i] + 3
    while j < @input.size && @input[j] <= lim
      ways += dp(j)
      j += 1
    end
    @dp[i] = ways
  end
end

main($stdin.read)
