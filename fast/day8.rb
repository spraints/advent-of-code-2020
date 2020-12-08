def main(input)
  prog = input.lines.map { |l|
    l =~ /(acc|jmp|nop) +([+-])(\d+)/
    inst = $1
    sign = $2
    val = $3.to_i
    val = -val if sign == '-'
    [inst, val]
  }
  acc = 0
  seen = []
  pc = 0
  loop do
    seen[pc] = true
    inst, val = prog[pc]
    case inst
    when 'acc'
      acc += val
      pc += 1
    when 'nop'
      pc += 1
    when 'jmp'
      pc += val
    end
    break if seen[pc]
  end
  puts "part 1: #{acc}"
  acc = run2(prog)
  puts "part 2: #{acc}"
end

def run2(prog, pc: 0, acc: 0, seen: {}, mutate: true)
  raise LoopDetected if seen[pc]
  seen = seen.merge(pc => true)
  inst, val = prog[pc]
  return acc if inst.nil?
  puts "pc=#{pc} #{inst} #{val}"
  case inst
  when 'acc'
    return run2 prog,
      pc: pc + 1,
      acc: acc + val,
      seen: seen,
      mutate: mutate
  when 'jmp'
    if mutate
      begin
        puts "jmp => nop"
        return run2 prog,
          pc: pc + 1,
          acc: acc,
          seen: seen,
          mutate: false
      rescue LoopDetected
        puts "BACKTRACK TO pc=#{pc}"
        # fall through
      end
    end
    return run2 prog,
      pc: pc + val,
      acc: acc,
      seen: seen,
      mutate: mutate
  when 'nop'
    if mutate
      begin
        puts "nop => jmp"
        return run2 prog,
          pc: pc + val,
          acc: acc,
          seen: seen,
          mutate: false
      rescue LoopDetected
        puts "BACKTRACK TO pc=#{pc}"
        # fall through
      end
    end
    return run2 prog,
      pc: pc + 1,
      acc: acc,
      seen: seen,
      mutate: mutate
  end
  raise "unexpected #{inst.inspect} pc=#{pc}"
end

class LoopDetected < StandardError
end

main($stdin.read)
