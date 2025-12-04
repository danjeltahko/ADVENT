package main

import (
	"fmt"
)

func inspect(puzzle []string, pos_y, pos_x int) bool {

	var puzzle_y_len int = len(puzzle)
	var puzzle_x_len int = len(puzzle[0])
	var counter int = 0

	// iterate around the position
	for y := -1; y <= 1; y++ {
		for x := -1; x <= 1; x++ {

			// skip if position is the one we're watching
			if y == 0 && x == 0 {
				continue
			}

			var new_y_pos int = pos_y + y
			var new_x_pos int = pos_x + x

			// check boundaries
			if new_y_pos >= 0 && new_y_pos < puzzle_y_len && new_x_pos >= 0 && new_x_pos < puzzle_x_len {
				if string(puzzle[new_y_pos][new_x_pos]) == "@" {
					counter++
				}
			}
		}
	}
	return counter < 4
}

func part_one(puzzle []string) {

	var paper_rolls int = 0
	// iterate each row
	for y := 0; y < len(puzzle); y++ {
		// iterate each col
		for x := 0; x < len(puzzle[y]); x++ {
			// check if pos has paper roll
			if string(puzzle[y][x]) == "@" {
				// inspect condition
				if inspect(puzzle, y, x) {
					paper_rolls++
				}
			}
		}
	}
	fmt.Println("Paper rolls accessed:", paper_rolls)
}
