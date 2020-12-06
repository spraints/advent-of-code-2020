a1 = []
b1 = 0
a2 = nil
b2 = 0
readlines.each do |line|
  ans = line.strip.split //
  if ans.empty?
    b1 += a1.size
    b2 += a2.size
    a1 = []
    a2 = nil
  else
    if a2.nil?
      a2 = ans
    else
      a2 &= ans
    end
    a1 |= ans
  end
end
b1 += a1.size
b2 += a2.size
puts b1, b2
