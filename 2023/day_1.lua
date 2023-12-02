


local function calibration(path)
  -- local sum = 0
  local file = io.open(path)
  for line in file:lines() do
    print(line)
    for i, char in pairs(line) do
      print(i, char) 
    end
  end
  file:close()
end

calibration("day_1_input.txt")
