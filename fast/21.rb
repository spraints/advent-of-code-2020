require_relative "./lib"
require "set"

def main(input)
  #bm "parse"

  bm "part 1"

  all_ingredients = []
  possible_allergens = {}
  input.lines.each do |line|
    line =~ /(.*) \(contains (.*)\)/ or raise line.inspect
    ingredients = $1.split(" ")
    allergens = $2.split(", ")
    all_ingredients += ingredients
    allergens.each do |a|
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
  puts "part 1: #{(all_ingredients - bad.keys).size}"

  bm "part 2"

  # in/21: rcqb,cltx,nrl,qjvvcvz,tsqpn,xhnk,tfqsb,zqzmzl
  puts "part 2: #{bad.sort_by(&:last).map(&:first).join(",")}"

ensure
  bm_done
end

main($stdin.read)
