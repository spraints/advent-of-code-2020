require_relative "./lib"

def main(input)
  input = input.lines

  mem = {}
  ormask = 0
  andmask = 0xfffffffff
  input.each do |line|
    case line
    when /mem\[(\d+)\] = (\d+)/
      addr = $1.to_i
      val = $2.to_i
      mem[addr] = (val & andmask) | ormask
    when /mask = (.*)/
      newmask = parse_mask($1)
      ormask = newmask[0]
      andmask = newmask[1]
    else
      raise "boom: #{line}"
    end
  end

  puts "part 1: #{mem.values.sum}"

  bm "part 2"

  mem = {}
  addrs = nil
  input.each do |line|
    case line
    when /mem\[(\d+)\] = (\d+)/
      addr = $1.to_i
      val = $2.to_i
      addrs.get(addr).each do |a|
        mem[a] = val
      end
    when /mask = (.*)/
      addrs = Mask.new($1)
    else
      raise "boom: #{line}"
    end
  end

  puts "part 2: #{mem.values.sum}"

  bm_done
end

# Returns [ormask, andmask]
def parse_mask(mask)
  mask.chars.map { |bit|
    case bit
    when "X"
      [0, 1]
    when "0"
      [0, 0]
    when "1"
      [1, 1]
    else
      raise "boom: #{bit}"
    end
  }.inject([0,0]) { |(res_or, res_and), (o, a)| [res_or<<1 | o, res_and<<1 | a] }
end

class Mask
  def initialize(mask)
    @mask = mask.chars.reverse
  end

  def get(in_addr)
    res = [in_addr]
    @mask.each_with_index do |rule, i|
      mask = 0x1 << i
      case rule
      when "X"
        res = res.flat_map { |addr| [addr | mask, addr & ~mask] }
      when "0"
        # noop
      when "1"
        res = res.map { |addr| addr | mask }
      else
        raise "unrecognized [#{rule}, #{i}]"
      end
    end
    res
  end
end

main($stdin.read)
