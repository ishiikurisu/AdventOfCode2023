def all_zeroes? inlet
  inlet.all?{|x| x == 0}
end

def find_previous_value(inlet)
  if all_zeroes? inlet then
    return 0
  end
  next_inlet = []
  for i in 1...inlet.length do
    next_inlet << inlet[i] - inlet[i-1]
  end
  return inlet [0] - find_previous_value(next_inlet)
end

def main
  outlet = 0
  loop do
    begin
      line = gets.chomp
      outlet += find_previous_value line.split(" ").collect{|x| x.to_i}
    rescue
      break
    end
  end
  puts outlet
end

if __FILE__ == $0 then
  main
end

