def main
  setup_data = gets.chomp.split(": ")[1].split(" ").collect{|s| s.to_i}
  gets

  seed_data = []
  is_first_seed = true
  first_seed = nil
  setup_data.each do |number|
    if is_first_seed then
      is_first_seed = false
      first_seed = number
    else
      seed_data << [first_seed, number]
      is_first_seed = true
    end
  end

  first_line = true
  all_mappings = []
  mappings = []
  loop do
    begin
      line = gets.chomp
      if first_line
        first_line = false
      elsif line.length == 0
        first_line = true
        all_mappings << mappings
        mappings = []
      else
        mappings << line.split(" ").collect{|s| s.to_i}
      end
    rescue
      break
    end
  end

  min_location = nil
  seed_data.each do |first_seed, length|
    for i in 0...length do
      location = first_seed + i
      
      all_mappings.each do |mappings|
        applied = false
        mappings.each do |destination, source, length|
          unless applied then
            within_range = location >= source && location <= source + length
            if within_range == true then
              applied = true
              location = destination + location - source
            end
          end
        end
      end

      if min_location.nil? || location < min_location then
        min_location = location
      end
    end
  end

  puts min_location
end

if __FILE__ == $0
  main
end

