def traverse_graph graph, start_node
  distances = {
    start_node => 0,
  }
  queue = [[start_node, 0]]

  while queue.length > 0 do
    node, distance = queue[0]
    next_nodes = graph[node] || []

    next_nodes.each do |next_node|
      unless distances.has_key? next_node then
        queue << [next_node, distance + 1]
        distances[next_node] = distance + 1
      end
      distances[next_node] = [distances[next_node], distance + 1].min
    end

    queue = queue[1..]
  end

  return distances
end

def north_check(raw_lines, i, j)
  line_length = raw_lines[0].length
  if (j > 0) && ("|F7".include? raw_lines[j - 1][i]) then
    return i + (j - 1) * line_length
  else
    return nil
  end
end

def south_check(raw_lines, i, j)
  lines_length = raw_lines.length
  line_length = raw_lines[0].length
  if (j < lines_length - 1) && ("|JL".include? raw_lines[j + 1][i]) then
    return i + (j + 1) * line_length
  else
    return nil
  end
end

def west_check(raw_lines, i, j)
  line_length = raw_lines[0].length
  if (i > 0) && ("-LF".include? raw_lines[j][i - 1]) then
    return (i - 1) + j * line_length
  else
    return nil
  end
end

def east_check(raw_lines, i, j)
  line_length = raw_lines[0].length
  if (i < line_length - 1) && ("-J7".include? raw_lines[j][i + 1]) then
    return (i + 1) + j * line_length
  else
    return nil
  end
end

def main
  raw_lines = []
  loop do
    begin
      raw_lines << gets.chomp
    rescue
      break
    end
  end

  graph = {}
  line_length = raw_lines[0].length
  lines_length = raw_lines.length
  start_node = nil
  raw_lines.each_index do |j|
    line = raw_lines[j]
    i = 0
    line.split("").each do |node|
      node_key = i + j * line_length
      next_nodes = []

      case node
      when '|'
        next_nodes << north_check(raw_lines, i, j)
        next_nodes << south_check(raw_lines, i, j)
      when '-'
        next_nodes << east_check(raw_lines, i, j)
        next_nodes << west_check(raw_lines, i, j)
      when 'L'
        next_nodes << north_check(raw_lines, i, j)
        next_nodes << east_check(raw_lines, i, j)
      when 'J'
        next_nodes << north_check(raw_lines, i, j)
        next_nodes << west_check(raw_lines, i, j)
      when '7'
        next_nodes << south_check(raw_lines, i, j)
        next_nodes << west_check(raw_lines, i, j)
      when 'F'
        next_nodes << south_check(raw_lines, i, j)
        next_nodes << east_check(raw_lines, i, j)
      when 'S'
        start_node = i + j * line_length
        next_nodes << north_check(raw_lines, i, j)
        next_nodes << south_check(raw_lines, i, j)
        next_nodes << west_check(raw_lines, i, j)
        next_nodes << east_check(raw_lines, i, j)
      end

      next_nodes.compact!
      if next_nodes.length > 0 then
        graph[node_key] = next_nodes
      end

      i += 1
    end
  end

  distances = traverse_graph graph, start_node 

  max_distance = -1
  distances.each do |node, distance|
    if distance > max_distance then
      max_distance = distance
    end
  end
  puts max_distance
end

if __FILE__ == $0 then
  main
end

