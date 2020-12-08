def main(input)
  prog = (input.lines.map { |l|
    l =~ /(acc|jmp|nop) +([+-])(\d+)/
    inst = $1
    sign = $2
    val = $3.to_i
    val = -val if sign == '-'
    {inst: inst, val: val, ok: [], nok: [], step: []}.freeze
  } + [{ok: [], nok: [], step: [:EOF]}]).freeze
  acc = 0
  pc = 0
  steps = 0
  loop do
    x = prog[pc]
    x[:step] << steps
    steps += 1
    case x.fetch(:inst)
    when 'acc'
      acc += x.fetch(:val)
      pc += 1
    when 'nop'
      pc += 1
    when 'jmp'
      pc += x.fetch(:val)
    end
    break unless prog[pc][:step].empty?
  end
  puts "part 1: #{acc} (expect 1832)"
  acc = run2(prog)
  puts "part 2: #{acc} (expect 662)"

  prog.each_with_index do |x, i|
    inst = c.fetch(:inst)
    val = c.fetch(:val)
    case inst
    when 'acc'
      prog[i+1][:ok] << i
    when 'jmp'
      prog[i+val][:ok] << i
      prog[i+1][:nok] << i
    when 'nop'
      prog[i+1][:ok] << i
      prog[i+val][:nok] << i
    end
  end
  pc = prog.size - 1
  loop do
    # https://www.geeksforgeeks.org/minimum-cost-of-simple-path-between-two-nodes-in-a-directed-and-weighted-graph/ ?
  end
end

def run2(prog, pc: 0, acc: 0, seen: {}, mutate: true)
  raise LoopDetected if seen[pc]
  seen = seen.merge(pc => true)
  x = prog[pc]
  inst = x[:inst]
  val = x[:val]
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
