local possib = {
  red = 12,
  green = 13,
  blue = 14,
}


local function strip2(str)
  local min_T = {
    red = 0,
    green = 0,
    blue = 0,
  }
  local game = string.sub(str, string.find(str, ':') + 1)
  for round in string.gmatch(game, "[^;]+") do
    for value, key in string.gmatch(round, "(%w+) (%w+)") do
      local value = tonumber(value)
      if min_T[key] < value then
        min_T[key] = value
      end
    end
  end
  return min_T.red * min_T.green * min_T.blue

end

local function strip(str)
  -- Create sub string of 'Game: 123' after the colon
  -- +1 becuase the find will be the index of colon
  local game = string.sub(str, string.find(str, ':') + 1)
  -- for k in string.gmatch(str, "([^;]+)") do
  -- strip the string on ';' and iterate through the strips
  for round in string.gmatch(game, "[^;]+") do
    -- table.insert(T, round)
    -- strip round ' 5 green, 1 blue' to key and value
    -- so k,v = 5, green & 1, blue
    for value, key in string.gmatch(round, "(%w+) (%w+)") do
      if possib[key] < tonumber(value) then return false end
    end
  end

  return true
end

local function read_file(file_path)
  local score = 0
  local score2 = 0
  local file = io.open(file_path)
  local i = 1
  for line in file:lines() do
    if strip(line) then
      score = score + i
    end
    score2 = score2 + strip2(line)
    i = i + 1
  end
  print("score " .. score, "2149")
  print("score2 " .. score2, "71274")
end

read_file("day_2_input.txt")
