def read line
  f = line.split(' ')
  a = [f[0], f[0], f[0], f[0], f[0]].join "?"
  b = [f[1], f[1], f[1], f[1], f[1]].join ","
  return "#{a} #{b}"
end

def is_valid? arrangement, target_arrangement
  hash_counts = arrangement.split(/\.+/).select{|s| s.length > 0}.collect{|s| s.length}

  if hash_counts.length != target_arrangement.length then
    return false
  end

  hash_counts.each_index do |i|
    hash_count = hash_counts[i]
    target_count = target_arrangement[i]

    if hash_count != target_count then
      return false
    end
  end

  return true
end

def evaluate inlet
  fields = inlet.split(' ')
  raw_springs = fields[0]
  target_arrangement = fields[1].split(',').collect{|x| x.to_i}
  possible_arrangements = 2**(raw_springs.split('').select{|x|x=='?'}.count)
  outlet = 0

  for i in 0...possible_arrangements do
    j = 0
    arrangement = ""
    raw_springs.each_char do |c|
      if c == '?' then
        d  = (((i >> j) & 0x1) == 0x1)? '#' : '.'
        arrangement << d
        j += 1
      else
        arrangement << c
      end
    end
    if is_valid? arrangement, target_arrangement
      outlet += 1
    end
  end
  
  return outlet
end

def main
  outlet = 0
  line = gets
  until line.nil? do
    inlet = read line.chomp
    outlet += evaluate inlet
    line = gets
  end
  puts outlet
end

if __FILE__ == $0 then
  main
end

