def main
  seeds = gets.chomp.split(": ")[1].split(" ").collect{|s| s.to_i}
  gets
  seed_data = []
  seeds.each do |seed|
    seed_data << {
      "location" => seed,
      "moved" => false,
    }
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
          data["moved"] = false
        end
      else
        destination, source, length = line.split(" ").collect{|s| s.to_i}
        seed_data.each do |data|
          location = data["location"]
          moved = data["moved"]
          within_range = location >= source && location <= source + length
          if within_range and not moved
            data["location"] = destination + location - source
            data["moved"] = true
          end
        end
      end
    rescue
      break
    end
  end

  min_location = nil
  seed_data.each do |data|
    location = data["location"]
    if min_location.nil? || location < min_location
      min_location = location
    end
  end

  puts min_location
end

if __FILE__ == $0
  main
end

