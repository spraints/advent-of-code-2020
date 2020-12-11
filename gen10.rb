vals = []
last = 0
ARGV.first.to_i.times do
  last += [1,1,3].shuffle.first
  vals << last
end
puts vals.shuffle
