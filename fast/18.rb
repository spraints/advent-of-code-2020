require_relative "./lib"

def main(input)
  bm "parse"

  bm "part 1"

  sum = 0
  input.lines.each do |line|
    sum += do_math(line.strip)
  end

  puts "part 1: #{sum}"

  bm "part 2"

  sum = 0
  input.lines.each do |line|
    sum += do_math2(line.strip)
  end

  puts "part 2: #{sum}"

ensure
  bm_done
end

def do_math2(line)
  #puts line
  tokens = line.scan(/\(|\)|\d+|\+|\*/)
  stack = []
  cur = []
  tokens.each do |t|
    #p t
    case t
    when '('
      stack << cur
      cur = []
    when ')'
      oldcur = stack.pop
      oldcur << cur
      cur = oldcur
    when '+', '*'
      cur << t
    else
      cur << t.to_i
    end
    #p cur: cur, stack: stack
  end
  make_math(cur)
end

def make_math(tokens)
  case tokens
  when Numeric
    p number: tokens
    tokens
  else
    p evalme: tokens
    tokens2 = [make_math(tokens.shift)]
    until tokens.empty?
      raise "not enough #{tokens.inspect}" if tokens.size < 2
      op = tokens.shift
      val = make_math(tokens.shift)
      if op == "+"
        tokens2.push(tokens2.pop + val)
      else
        tokens2 << val
      end
    end
    p prod: tokens2
    tokens2.inject(&:*)
  end
end

def do_math(line)
  stack = []
  res = nil
  op = nil
  #puts line
  line.scan(/\(|\)|\d+|\+|\*/).each do |x|
    #p x: x, res: res, op: op, stack: stack
    case x
    when '('
      stack.push res, op
      res = op = nil
    when ')'
      raise "expect no op here #{op.inspect}" unless op.nil?
      inres = res
      op = stack.pop
      res = stack.pop
      case op
      when '+'
        res = res + inres
      when '*'
        res = res * inres
      when nil
        res = inres
      else
        raise op.inspect
      end
      op = nil
    when '+', '*'
      op = x
    when /\d+/
      if res.nil?
        res = x.to_i
      else
        case op
        when '+'
          res += x.to_i
        when '*'
          res *= x.to_i
        else
          raise "unknown op #{op.inspect}"
        end
        op = nil
      end
    else
      raise "wat #{x.inspect} (#{line.inspect})"
    end
  end
  #puts "=> #{res}"
  res
end

main($stdin.read)
