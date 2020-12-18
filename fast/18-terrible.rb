class Integer
  alias_method :add, :+
  alias_method :+, :*
  alias_method :*, :add
  alias_method :-, :add
end
lines = $stdin.readlines
p lines.map { |line|
    eval(line.tr("+*", "-+"))
}.sum
p lines.map { |line|
    eval(line.tr("+*", "*+"))
}.sum
