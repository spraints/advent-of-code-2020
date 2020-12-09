def main(input)
  input = input.split.map(&:to_i)

  preamble = ARGV.first.to_i
  preamble = 25 if preamble < 1

  w1 = weakness1(pre: input.take(preamble), rest: input.drop(preamble))
  puts "part 1: #{w1}"
  w2 = weakness2(w1, input)
  puts "part 2: #{w2.sum} => #{w2.min + w2.max}"
end

def weakness2(n, arr)
  res = []
  acc = 0
  until acc == n || arr.empty?
    if acc < n
      x = arr.shift
      res << x
      acc += x
    elsif acc > n
      acc -= res.shift
    end
  end
  res
end

def weakness1(pre:, rest:)
  #p pre
  while has_sum2?(rest.first, pre)
    pre.shift
    pre.push(rest.shift)
    #p pre
  end
  rest.first
end

def has_sum2?(n, arr)
  return false if n.nil?
  arr = arr.sort
  a = arr.shift
  b = arr.pop
    #p a: a, b: b, arr: arr
  until arr.empty? || a + b == n
    if a + b < n
      a = arr.shift
    elsif a + b > n
      b = arr.pop
    end
    #p a: a, b: b, arr: arr
  end
  #puts "(#{a + b == n} ~ #{n})"
  a + b == n
end

main($stdin.read)
