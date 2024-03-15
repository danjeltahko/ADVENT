

local test_string = {"1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"}
local test_string_2 = {
  "two1nine", 
  "eightwothree", 
  "abcone2threexyz", 
  "xtwone3four", 
  "4nineeightseven2", 
  "zoneight234", 
  "7pqrstsixteen"
}

-- 29, 83, 13, 24, 42, 14, 76 

local digits = {
  one = '1',
  two = '2',
  three = '3',
  four = '4',
  five = '5',
  six = '6',
  seven = '7',
  eight = '8',
  nine = '9'
}

-- Find first and last digit in line
-- @param line (str) line to find the digits
-- @returns x, y (str) the digits
local function calculate_value(line)
  local x = string.match(line, '[0-9]')
  local y = string.match(string.reverse(line), '[0-9]')
  return x, y -- Concatenate string
end

local function find_digit_pos(line)

  local X = {}
  local Y = {}

  -- Finds the int position in string
  local found_x, found_y = calculate_value(line)
  if found_x then X = {string.find(line, found_x), found_x} end
  -- if found_y then Y = {string.find(line, found_y), found_y} end
  if found_y then Y = {#line - string.find(string.reverse(line), found_y), found_y} end
  
  -- if X and #X > 0 then print("X is " .. X[1] .. " - " .. X[2]) end
  -- if Y and #Y > 0 then print("Y is " .. Y[1] .. " - " .. Y[2]) end

  for k, v in pairs(digits) do
    -- find first and last index of matching word 
    -- local index = string.find(line, k)
    local T = {}

    local i = 1
    while i < #line do
      -- print("try to find "..k.." in "..line.." on index "..i)
      local first, last = string.find(line,k, i)
      if first then
        -- print("-- found "..k.." in "..line.." on index "..i)
        -- print(first, last, i)
        table.insert(T, first)
        i = last
        -- print("after added last to i "..i)
      end
      i = i + 1
    end

    for _, index in pairs(T) do
      -- print(k.." on index "..index)
      if index then
        if #X > 0 then
          if index < X[1] then
            X = {index, v}
          end
        else
          X = {index, v}
        end
        if #Y > 0 then
          if index > Y[1] then
            -- print("index "..index.." is greater than "..Y[1])
            -- 6 should be on line 40
            Y = {index, v}
          end
        else
          Y = {index, v}
        end
      end
    end
  end
  -- print(line .. " = " .. "(" .. X[2] .. " , " .. Y[2] .. ")")
  -- print("---------------")
  return X[2], Y[2]
end

local function calibration(path)
  local file = io.open(path)
  local sum = 0
  local sum2 = 0
  for line in file:lines() do
    x, y = calculate_value(line)
    value1 = x .. y
    sum = sum + value1
    _x, _y = find_digit_pos(line)
    value2 = _x .. _y
    sum2 = sum2 + value2
  end
  file:close()
  print(sum, "54601")
  print(sum2, "54078")
end

calibration("day_1_input.txt")
