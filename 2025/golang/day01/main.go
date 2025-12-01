package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func read_file(f_path string) ([]string, error) {

	file, err := os.Open(f_path)
	if err != nil {
		return nil, err
	}
	defer file.Close()
	var lines []string
	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	return lines, scanner.Err()

}

func password_logic(start int, line string, counter int) (int, int) {
	dir := line[0]
	times, _ := strconv.Atoi(line[1:])

	var new_value int
	var start_on_zero bool
	if start == 0 {
		start_on_zero = true
	} else {
		start_on_zero = false
	}
	// var found string = ""
	if string(dir) == "L" {
		new_value = start - times
		for new_value < 0 {
			// part TWO
			if !start_on_zero {
				counter++
			}
			if start_on_zero {
				start_on_zero = false
			}
			new_value = 100 + new_value
		}
	} else if string(dir) == "R" {
		new_value = start + times
		for new_value > 99 {
			// PART TWO
			if new_value != 100 {
				counter++
			}
			new_value = (new_value - 100) + 0
		}
	}
	if new_value == 0 {
		counter += 1
	}
	return new_value, counter
}

func main() {
	lines, err := read_file("inputs/day01.txt")
	if err != nil {
		fmt.Println(err)
	}

	var start int = 50
	var counter int = 0
	for _, line := range lines {
		start, counter = password_logic(start, line, counter)
	}
	fmt.Println("Times pointing at 0:", counter)
}
