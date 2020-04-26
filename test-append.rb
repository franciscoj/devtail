#!/usr/bin/env ruby

# Reads a line by line from `origin` and appends at the end of a file called
# like "#{origin}.new".
#
# E.g.:
#
# some.log => some.log.new
origin_path = ARGV[0]
destination_path = "#{origin_path}.new"
puts "Appending '#{origin_path}' to '#{destination_path}'..."
destination = File.open(destination_path, "a")

File.open(origin_path, 'r').each_line do |line|
  print '.'
  destination.write(line)
  sleep 0.05
end

destination.close

puts "\nDone!"
