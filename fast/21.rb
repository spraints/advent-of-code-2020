require_relative "./lib"
require "set"

SOLUTIONS.update \
  2659 => [1, :input],
  "rcqb,cltx,nrl,qjvvcvz,tsqpn,xhnk,tfqsb,zqzmzl" => [2, :input],
  5 => [1, :sample],
  "mxmxvkd,sqjhc,fvjkl" => [2, :sample]

def main(input)
  #bm "parse"
  bm_size input.lines.size

  bm "part 1"

  all_ingredients = []
  possible_allergens = {}
  input.lines.each do |line|
    line =~ /(.*) \(contains (.*)\)/ or raise line.inspect
    ingredients = $1.split(" ")
    allergens = $2.split(", ")
    all_ingredients += ingredients
    allergens.each do |a|
      bm_step
      if y = possible_allergens[a]
        possible_allergens[a] = y & ingredients
      else
        possible_allergens[a] = ingredients
      end
    end
  end

  bad = {}
  until possible_allergens.empty?
    np = {}
    possible_allergens.sort_by { |a, b| b.size }.each do |a, i|
      bm_step
      j = i - bad.keys
      if j.size == 0
        raise "impossible #{[a, i, j, bad].inspect}"
      elsif j.size == 1
        bad[j[0]] = a
      else
        np[a] = j
      end
    end
    possible_allergens = np
  end

  # in/21: 2659
  p1done (all_ingredients - bad.keys).size

  bm "part 2"

  # in/21: rcqb,cltx,nrl,qjvvcvz,tsqpn,xhnk,tfqsb,zqzmzl
  p2done bad.sort_by(&:last).map(&:first).join(",")

ensure
  bm_done
end

main($stdin.read)
