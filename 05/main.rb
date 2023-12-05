def read_map
  map_name = gets.chomp.split(" ")[0]
  map = {}

  line = gets.chomp
  while line.length > 0 do
    destination, source, length = line.split(" ").collect{|s| s.to_i}
    for i in 0...length do
      map[source + i] = destination + i
    end
    line = gets.chomp
  end

  return map_name, map
end

def find_location seed, steps, maps
  location = seed

  steps.each do |step|
    map = maps[step]
    location = (map.key? location)? map[location] : location
  end

  return location
end

def main
  seeds = gets.chomp.split(": ")[1].split(" ").collect{|s| s.to_i}
  gets
  maps = {}
  steps = []
  
  loop do
    begin
      map_name, map = read_map
      maps[map_name] = map
      steps << map_name
    rescue
      break
    end
  end

  locations = {}
  seeds.each do |seed|
    locations[seed] = find_location seed, steps, maps
  end

  result_seed = nil
  result_location = nil
  locations.each do |seed, location|
    if result_location.nil? || location < result_location
      result_seed = seed
      result_location = location
    end
  end

  puts result_location
end

if __FILE__ == $0
  main
end

