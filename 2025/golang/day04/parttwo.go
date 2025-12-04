package main

import (
	"fmt"
)

func remove_paper_rolls(puzzle []string) [][]int {
	var found_paper_rolls [][]int
	for y := 0; y < len(puzzle); y++ {
		// iterate each col
		for x := 0; x < len(puzzle[y]); x++ {
			// check if pos has paper roll
			if string(puzzle[y][x]) == "@" {
				// inspect condition
				if inspect(puzzle, y, x) {
					found_paper_rolls = append(found_paper_rolls, []int{y, x})
				}
			}
		}
	}
	return found_paper_rolls
}

func part_two(puzzle []string) {

	var moved_paper_rolls int = 0

	for {

		// while loop until no more paper rolls are movable
		found_paper_rolls := remove_paper_rolls(puzzle)
		if len(found_paper_rolls) < 1 {
			break
		}

		moved_paper_rolls += len(found_paper_rolls)

		// modify the map
		for i := range len(found_paper_rolls) {
			y := found_paper_rolls[i][0]
			x := found_paper_rolls[i][1]
			puzzle[y] = puzzle[y][:x] + "." + puzzle[y][x+1:]
		}

	}
	fmt.Println("Paper rolls accessed:")
	fmt.Println(moved_paper_rolls)
}
