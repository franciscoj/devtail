#!/usr/bin/env ruby

origin_path = ARGV[0]
puts "Printing '#{origin_path}'"

File.open(origin_path, 'r').each_line do |line|
  puts line
  sleep 0.05
end

puts "\nDone!"
