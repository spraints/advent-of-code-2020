a = nil
b = 0
readlines.each do |line|
  ans = line.strip.split //
  if ans.empty?
    b += a.size
    a = nil
  elsif a.nil?
    a = ans
  else
    a &= ans
  end
end
b += a.size
puts b
