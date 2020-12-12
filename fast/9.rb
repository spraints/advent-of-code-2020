require_relative "./lib"

def main(input)
  bm "parse"

  input = input.split.map(&:to_i)

  preamble = ARGV.first.to_i
  preamble = 25 if preamble < 1

  bm "part 1"
  w1 = weakness1(input, preamble: preamble)
  puts "part 1: #{w1}"

  bm "part 2"
  w2 = weakness2(input, sum: w1)
  puts "part 2: #{w2.min + w2.max} (#{w2.size})"

  bm_done
end

def weakness2(arr, sum:)
  acc = arr.take(2).sum
  first = 0
  last = 1
  loop do
    if acc == sum
      return arr.slice(first..last)
    elsif last - first < 2 || acc < sum
      last += 1
      if last < arr.size
        acc += arr[last]
      else
        return nil
      end
    else
      acc -= arr[first]
      first += 1
    end
  end
end

def weakness1(arr, preamble:)
  arr.each_cons(preamble + 1) do |nums|
    sum = nums.pop
    if sum2(nums, sum: sum).nil?
      return sum
    end
  end
  raise "no weakness found"
end

main($stdin.read)
