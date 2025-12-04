package main

import (
	"bufio"
	"os"
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

func main() {
	puzzle, _ := read_file("inputs/day04.txt")
	part_one(puzzle)
	part_two(puzzle)
}
