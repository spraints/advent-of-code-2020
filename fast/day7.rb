def main(input)
  rules = Hash[input.lines.map { |l| parse(l) }]
  can_be_in = Hash.new { |h, k| h[k] = [] }
  rules.each do |outer, inner|
    inner.each do |_, i|
      can_be_in[i] << outer
    end
  end
  found = {}
  todo = ["shiny gold"]
  while x = todo.shift
    found[x] = true
    can_be_in[x].each do |c|
      todo.push c unless found[c]
    end
  end
  puts "part 1: #{found.size - 1}"

  puts "part 2: #{pack(rules, "shiny gold") - 1}"
end

def pack(rules, cur)
  return 1 unless rules[cur]
  rules[cur].inject(1) { |n, (m, t)| n + m * pack(rules, t) }
end

def parse(line)
  outer, inner = line.split("contain", 2)
  outer =~ /(.*) bags.*/ and outer = $1
  inner = inner.split(/[,.]/).map { |i| i =~ /(\d) (.*) bag/ and [$1.to_i, $2] }.compact
  [outer, inner]
end

main($stdin.read)
