
def acc(val)
  {inst: "acc", val: val}
end
def jmp(val)
  {inst: "jmp", val: val}
end
def nop(val)
  {inst: "nop", val: val}
end
def main(input)
  prog = (input.lines.map { |l| eval(l) })

  puts "part 1: #{run(prog)} (expect 1832)"

  prog << {inst: :END}
  prog.each_with_index do |x, i|
    inst = x[:inst]
    val = x[:val]
    case inst
    when 'jmp'
      (prog[i + val][:from] ||= []) << i
      (prog[i + 1][:mfrom] ||= []) << i
    when 'nop'
      (prog[i + 1][:from] ||= []) << i
      (prog[i + val][:mfrom] ||= []) << i
    when 'acc'
      (prog[i + 1][:from] ||= []) << i
    end
  end

  end_reachable = [prog.size - 1]
  broken = true
  while broken && pc = end_reachable.shift
    end_reachable += (prog[pc][:from] || [])
    (prog[pc][:mfrom] || []).each do |i|
      if prog[i][:reachable]
        p broken: prog[i], i: i
        prog[i][:inst] = 
          if prog[i][:inst] == 'jmp'
            'nop'
          else
            'jmp'
          end
        broken = false
        break
      end
    end
  end

  puts "part 2: #{run(prog)} (expect 662)"
end

def run(prog)
  prog.each { |x| x.delete :reachable }
  acc = 0
  pc = 0
  loop do
    x = prog[pc]
    x[:reachable] = true
    case x.fetch(:inst)
    when 'acc'
      acc += x.fetch(:val)
      pc += 1
    when 'nop'
      pc += 1
    when 'jmp'
      pc += x.fetch(:val)
    when :END
      return [acc, true]
    end
    if prog[pc][:reachable]
      return [acc, false]
    end
  end
end

main($stdin.read)
