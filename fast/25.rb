require_relative "./lib"
require "set"

SOLUTIONS.update \
  10548634 => [1, :in],
  11328376 => [1, :oliver]

def main(input)

  pk1, pk2 = input.lines.map(&:to_i)

  ls2 = bfpk(pk2)

  enc = transform subject: pk1, ls: ls2

  p1done enc

  # 7^ls1 = pk1 mod 20201227
  # 7^ls2 = pk2 mod 20201227
  # pk1^ls2 = enc mod 20201227
  # pk2^ls1 = enc mod 20201227
  #
  # (7^ls1)^ls2 = enc mod 20201227

ensure
  bm_done
end

def bfpk(pk)
  val = 1
  n = 0
  while val != pk
    bm_step
    n += 1
    val = (val * 7) % 20201227
  end
  n
end

def transform(subject:, ls:)
  subject.pow(ls, 20201227)
end

main($stdin.read)
