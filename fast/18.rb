require_relative "./lib"

def main(input)
  parsed = input.lines.map { |line| parse_line(line) }

  sum = parsed.inject(0) { |sum, line| sum + eval_part1(line) }

  puts "part 1: #{sum}"

  bm "part 2"

  sum = parsed.inject(0) { |sum, line| sum + eval_part2(line) }

  puts "part 2: #{sum}"

ensure
  bm_done
end

def parse_line(line)
  res = []
  stack = []
  line.scan(/\(|\)|\d+|\+|\*/).each do |t|
    case t
    when '('
      stack << res
      res = []
    when ')'
      outer = stack.pop
      outer << res.freeze
      res = outer
    when '+', '*'
      res << t
    else
      res << t.to_i
    end
  end
  res.freeze
end

def eval_part2(tokens)
  case tokens
  when Numeric
    tokens
  else
    tokens = tokens.dup
    factors = [eval_part2(tokens.shift)]
    until tokens.empty?
      raise "not enough #{tokens.inspect}" if tokens.size < 2
      op = tokens.shift
      val = eval_part2(tokens.shift)
      if op == "+"
        factors.push(factors.pop + val)
      else
        factors << val
      end
    end
    factors.inject(&:*)
  end
end

def eval_part1(tokens)
  case tokens
  when Numeric
    tokens
  else
    tokens = tokens.dup
    res = eval_part1(tokens.shift)
    until tokens.empty?
      raise "not enough #{tokens.inspect}" if tokens.size < 2
      op = tokens.shift
      val = eval_part1(tokens.shift)
      if op == "+"
        res += val
      else
        res *= val
      end
    end
    res
  end
end

main($stdin.read)
