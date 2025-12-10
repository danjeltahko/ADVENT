package main

import (
	"fmt"
	// "math"
	// "slices"
	// "strconv"
	// "strings"
)

func part_one(puzzle [][]int) {
	var max_area int = 0
	// iterate through each line of the puzzle
	// and skip last line, because it will be
	// compared with j index (i, j := i+1)
	for i := 0; i < len(puzzle)-1; i++ {
		// compare with not yet compared (j := i + 1)
		for j := i + 1; j < len(puzzle); j++ {

			if puzzle[i][0] < puzzle[j][0] && puzzle[i][1] < puzzle[j][1] {
				// if first pos is left top
				area := (((puzzle[j][0] - puzzle[i][0]) + 1) * ((puzzle[j][1] - puzzle[i][1]) + 1))
				// fmt.Println(puzzle[i], puzzle[j], " | Area:", area)
				if area > max_area {
					max_area = area
				}

			} else if puzzle[i][0] < puzzle[j][0] && puzzle[i][1] > puzzle[j][1] {
				// if first pos is left bottom
				area := (((puzzle[j][0] - puzzle[i][0]) + 1) * ((puzzle[i][1] - puzzle[j][1]) + 1))
				// fmt.Println(puzzle[i], puzzle[j], " | Area:", area)
				if area > max_area {
					max_area = area
				}

			} else if puzzle[i][0] > puzzle[j][0] && puzzle[i][1] < puzzle[j][1] {
				// if first pos is right top
				area := (((puzzle[i][0] - puzzle[j][0]) + 1) * ((puzzle[j][1] - puzzle[i][1]) + 1))
				// fmt.Println(puzzle[i], puzzle[j], " | Area:", area)
				if area > max_area {
					max_area = area
				}

			} else if puzzle[i][0] > puzzle[j][0] && puzzle[i][1] > puzzle[j][1] {
				// if first pos is right bottom
				area := (((puzzle[i][0] - puzzle[j][0]) + 1) * ((puzzle[i][1] - puzzle[j][1]) + 1))
				// fmt.Println(puzzle[i], puzzle[j], " | Area:", area)
				if area > max_area {
					max_area = area
				}

			}
		}
	}
	fmt.Println("Max area:", max_area)
}
