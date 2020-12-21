require_relative "./lib"

def main(input)
  input = input.lines
  rules = {}
  loop do
    line = input.shift.strip
    break if line.empty?
    label, rule = parse_rule(line)
    rules[label] = rule
  end
  messages = input
  #p rules
  #p messages

  puts "part 1: #{messages.count { |m| rules_match?(rules, m.strip.chars) }}"

  bm "part 2"

  replacements = <<-R
    8: 42 | 42 8
    11: 42 31 | 42 11 31
  R
  replacements.lines.each do |line|
    label, rule = parse_rule(line.strip)
    rules[label] = rule
  end

  puts "part 3: #{messages.count { |m| rules_match?(rules, m.strip.chars) }}"

ensure
  bm_done
end

def rules_match?(rules, chars)
  each_match(chars, rules, "0") do |rest|
    return true if rest.empty?
  end
  false
end

def each_match(chars, rules, r)
  #puts "OK? #{chars.join} vs #{r.inspect} #{rules[r].inspect}"
  case rule = rules.fetch(r)
  when :a
 #   puts " ... is a?"
    yield chars.drop(1) if chars.first == "a"
  when :b
 #   puts " ... is b?"
    yield chars.drop(1) if chars.first == "b"
  else
    rule.each do |opts|
 #     puts " ... matches #{opts.inspect}?"
      try_rule(opts, chars, rules) do |rest|
        yield rest
      end
    end
  end
end

def try_rule(opts, chars, rules)
 # p try: opts, chars: chars
  r, *opts = opts
  each_match(chars, rules, r) do |rest|
    if opts.empty?
      yield rest
    else
      try_rule(opts, rest, rules) do |rest|
        yield rest
      end
    end
  end
end

def parse_rule(line)
  label, rules = line.split(": ")
  if rules == '"a"'
    return [label, :a]
  elsif rules == '"b"'
    return [label, :b]
  end
  rules = rules.split("|").map { |r| r.split(" ").map(&:strip) }
  [label, rules]
end

main($stdin.read)
