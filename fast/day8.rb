def main(input)
  prog = input.lines.map { |l|
    l =~ /(acc|jmp|nop) +([+-])(\d+)/
    inst = $1
    sign = $2
    val = $3.to_i
    val = -val if sign == '-'
    [inst, val].freeze
  }.freeze
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

  puts "part 1: #{Comp.new(prog).run}"
end

class Comp
  def initialize(prog)
    @prog = prog
  end

  def run
    pc = 0
    acc = 0
    seen = []
    steps = 0
    loop do
      if seen[pc]
        return {acc: acc, pc: pc, steps: steps, loop: true}
      end
      seen[pc] = true
      if inst = @prog[pc]
        steps += 1
        pc, acc = runinst(*inst, pc: pc, acc: acc)
      else
        return {acc: acc, pc: pc, steps: steps, loop: false}
      end
    end
  end

  def runinst(inst, val, pc:, acc:)
    case inst
    when 'acc'
      [pc + 1, acc + val]
    when 'nop'
      [pc + 1, acc]
    when 'jmp'
      [pc + val, acc]
    end
  end
end

def run2(prog, pc: 0, acc: 0, seen: {}, mutate: true)
  raise LoopDetected if seen[pc]
  seen = seen.merge(pc => true)
  inst, val = prog[pc]
  return acc if inst.nil?
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
        return run2 prog,
          pc: pc + 1,
          acc: acc,
          seen: seen,
          mutate: false
      rescue LoopDetected
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
        return run2 prog,
          pc: pc + val,
          acc: acc,
          seen: seen,
          mutate: false
      rescue LoopDetected
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
