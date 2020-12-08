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
end

main($stdin.read)
