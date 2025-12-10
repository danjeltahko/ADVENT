package main

import (
	"bufio"
	"os"
	"strconv"
	"strings"
)

func read_file(f_path string) ([][]int, error) {

	file, err := os.Open(f_path)
	if err != nil {
		return nil, err
	}
	defer file.Close()
	var lines [][]int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {

		line := scanner.Text()
		items := strings.Split(line, ",")
		var int_items []int
		for i := range items {
			int_item, _ := strconv.Atoi(items[i])
			int_items = append(int_items, int_item)
		}
		lines = append(lines, int_items)
	}
	return lines, scanner.Err()
}

func main() {
	puzzle, _ := read_file("inputs/day09-test.txt")
	// part_one(puzzle)
	part_two(puzzle)
}
