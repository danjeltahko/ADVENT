package main

import (
	"bufio"
	"os"
)

func read_file(f_path string) ([][]string, error) {

	file, err := os.Open(f_path)
	if err != nil {
		return nil, err
	}
	defer file.Close()
	var ranges []string
	var lines []string
	var sep bool = false

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		if len(line) < 1 {
			sep = true
			continue
		}

		if sep {
			lines = append(lines, line)
		} else {
			ranges = append(ranges, line)

		}
	}
	return [][]string{ranges, lines}, scanner.Err()
}

func main() {
	puzzle, _ := read_file("inputs/day05.txt")
	part_one(puzzle)
	part_two(puzzle[0])
}
