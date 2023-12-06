

local function calculate_value(line)
  -- print(line)
  x = string.match(line, '[0-9]')
  y = string.match(string.reverse(line), '[0-9]')
  return x .. y -- Concatenate string
end

local function calculate_part_two(line)
  local digits = {"one"='1', "two"='2', "three"='3', "four"='4', "five"='5', "six"='6', "seven"='7', "eight"='8', "nine"='9'}
  x = string.match(line, digits)
end

local function calibration(path)
  local file = io.open(path)
  local sum = 0
  for line in file:lines() do
    sum = sum + calculate_value(line)
  end
  file:close()
  print(sum)
end

calibration("day_1_input.txt")

