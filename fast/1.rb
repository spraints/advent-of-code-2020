require_relative "./lib"

def main(input)
  nums = input.split.map(&:to_i)

  res = sum2(nums, sum: 2020)
  puts "part 1: [#{res.join(" + ")} = 2020] => #{res.inject(&:*)}"

  res = sum3(nums, sum: 2020)
  puts "part 2: [#{res.join(" + ")} = 2020] => #{res.inject(&:*)}"
end

main($stdin.read)
