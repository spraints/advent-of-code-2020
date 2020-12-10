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
  input.each do |x|
    next_steps[x] = input.select { |y| y > x && y <= x + 3 }
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
  p choices[0]
  

  #w2 = weakness2(input, sum: w1)
  #puts "part 2: #{w2.min + w2.max} (#{w2.size})"

  bm_done
end

#def weakness2(arr, sum:)
#  acc = arr.take(2).sum
#  first = 0
#  last = 1
#  loop do
#    if acc == sum
#      return arr.slice(first..last)
#    elsif last - first < 2 || acc < sum
#      last += 1
#      if last < arr.size
#        acc += arr[last]
#      else
#        return nil
#      end
#    else
#      acc -= arr[first]
#      first += 1
#    end
#  end
#end
#
#def weakness1(arr, preamble:)
#  arr.each_cons(preamble + 1) do |nums|
#    sum = nums.pop
#    if sum2(nums, sum: sum).nil?
#      return sum
#    end
#  end
#  raise "no weakness found"
#end

main($stdin.read)
