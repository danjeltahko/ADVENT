package main

import (
	"bufio"
	"os"
	"strings"
)

func read_file(f_path string) ([]string, error) {

	file, err := os.Open(f_path)
	if err != nil {
		return nil, err
	}
	defer file.Close()
	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		items := strings.Split(line, ",")
		lines = append(lines, items...)
	}
	return lines, scanner.Err()
}

func main() {
	puzzle, _ := read_file("inputs/day03.txt")
	part_one(puzzle)
	part_two(puzzle)
}
