require_relative "./lib"
require "set"

SOLUTIONS.update \
  10548634 => [1, :in],
  11328376 => [1, :oliver]

def main(input)

  pk1, pk2 = input.lines.map(&:to_i)

  ls1 = bfpk(pk1)
  ls2 = bfpk(pk2)
  p [ls1, ls2]

  enc = transform subject: pk1, ls: ls2
  #enc = transform subject: pk2, ls: ls1

  p1done enc

  #card_subj = 7
  #door_subj = 7
  #
  #card_ls = 8
  #door_ls = 11
  #
  # pk = 7^ls mod 20201227
  #card_pk = transform subject: card_subj, ls: card_ls
  #door_pk = transform subject: door_subj, ls: door_ls
  #
  #enc_key = transform subject: card_pk, ls: door_ls
  #enc_key2 = transform subject: door_pk, ls: card_ls
  #
  #p1done \
  #  card_pk: card_pk,
  #  door_pk: door_pk,
  #  enc_key: [enc_key, enc_key2]

  # ---------------------------------

  p2done 0

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
