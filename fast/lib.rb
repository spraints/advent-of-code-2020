def sum2(arr, sum:)
  arr = arr.sort
  _sum2(arr, sum: sum, start: 0)
end

def _sum2(arr, sum:, start:)
  a, b = start, arr.size - 1
  until arr[a] + arr[b] == sum
    return nil if a + 1 == b
    n = arr[a] + arr[b]
    if n < sum
      a += 1
    else
      b -= 1
    end
  end
  [arr[a], arr[b]]
end

def sum3(arr, sum:)
  arr = arr.sort
  (0..(arr.size - 3)).each do |a|
    if others = _sum2(arr, sum: sum - arr[a], start: a + 1)
      b, c = others
      return [arr[a], b, c]
    end
  end
  nil
end
