def main
  setup_data = gets.chomp.split(": ")[1].split(" ").collect{|s| s.to_i}
  gets

  seed_data = []
  is_first_seed = true
  first_seed = nil
  setup_data.each do |number|
    if is_first_seed
      is_first_seed = false
      first_seed = number
    else
      for i in 0..number do
        seed_data << [first_seed + i, false]
      end
      is_first_seed = true
    end
  end

  first_line = true
  loop do
    begin
      line = gets.chomp
      if first_line
        first_line = false
      elsif line.length == 0
        first_line = true
        seed_data.each do |data|
          data[1] = false
        end
      else
        destination, source, length = line.split(" ").collect{|s| s.to_i}
        next_seed_data = []
        seed_data.each do |data|
          location, moved = data
          within_range = location >= source && location <= source + length
          if within_range and not moved then
            next_seed_data << [destination + location - source, true]
          else
            next_seed_data << data
          end
        end
        seed_data = next_data
      end
    rescue
      break
    end
  end

  min_location = nil
  seed_data.each do |data|
    location = data[0]
    if min_location.nil? || location < min_location
      min_location = location
    end
  end

  puts min_location
end

if __FILE__ == $0
  main
end

