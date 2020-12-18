class Integer
  alias_method :add, :+
  alias_method :mul, :*
end
lines = $stdin.readlines

class Integer
  alias_method :+, :add
  alias_method :-, :mul
end
p lines.map { |line|
    eval(line.tr("+*", "+-"))
}.sum

class Integer
  alias_method :+, :mul
  alias_method :*, :add
end
p lines.map { |line|
    eval(line.tr("+*", "*+"))
}.sum
