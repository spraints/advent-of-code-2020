require_relative "./lib"

def main(input)
  bm "parse"

  rules, ticket, nearby = input.split("\n\n")
  rules = Rules.new(rules)
  ticket = ticket.lines[1]
  nearby = nearby.lines.drop(1)

  bm "part 1"

  civ = []
  ok_tix = []
  nearby.each do |t|
    t = t.strip.split(",").map(&:to_i)
    bad = rules.civ(t)
    civ += bad
    ok_tix << t if bad.empty?
  end

  puts "part 1: #{civ.sum}"

  bm "part 2"

  num_cols = ok_tix.first.size
  if rules.parsed.size != num_cols
    raise "wat"
  end

  possible_rules = num_cols.times.map { |i| [i, rules.parsed.select { |_, rule| ok_tix.all? { |t| rule === t[i] } } ] }
  possible_rules.sort_by! { |_, r| r.size }
  cols = [nil] * num_cols
  while x = possible_rules.shift
    i, rule = x
    raise "uh oh #{x}" unless rule.size == 1
    rule = rule.first
    cols[i] = rule
    possible_rules.each do |_, r| 
      r.delete(rule)
    end
  end

  my_ticket = ticket.split(",").map(&:to_i)
  res = 1
  cols.each_with_index do |(label, _), i|
    if label =~ /^departure/
      res *= my_ticket[i]
    end
  end
  puts "part 2: #{res}"

ensure
  bm_done
end

class Rules
  def initialize(raw)
    @raw = raw
  end

  def civ(vals)
    vals.reject { |val| maybe_ok?(val) }
  end

  def maybe_ok?(val)
    parsed.any? { |_, rule| rule === val }
  end

  def parsed
    @parsed ||= @raw.lines.map { |l| parse_rule(l) }
  end

  def parse_rule(line)
    label, rule = line.split(":")
    rules = rule.split(" or ").map { |r| Range.new(*r.split("-").map(&:to_i)) }
    [label, Rule.new(rules)]
  end
end

class Rule
  def initialize(ranges)
    @ranges = ranges
  end

  def ===(other)
    @ranges.any? { |r| r === other }
  end
end

main($stdin.read)
