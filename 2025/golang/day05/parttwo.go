package main

import (
	"fmt"
	"strconv"
	"strings"
)

func part_two(puzzle []string) {

	var fresh_counter int = 0
	var fresh_ranges [][]int

	// i tried to iterate each line in the slice
	// but then realized that there could be a
	// range in the middle, forcing me to split the
	// slice. therefore looping the length and
	// popping & appending to the slice instead
	for len(puzzle) > 0 {

		// split the range string and convert to int
		splitted_line := strings.Split(puzzle[0], "-")
		first_r, _ := strconv.Atoi(splitted_line[0])
		last_r, _ := strconv.Atoi(splitted_line[1])

		// add first range when noting to compare with
		if len(fresh_ranges) < 1 {
			if first_r == last_r {
				fresh_counter++
			} else {
				fresh_counter += last_r - first_r + 1
			}
			fresh_ranges = append(fresh_ranges, []int{first_r, last_r})
			puzzle = puzzle[1:]
			continue
		}

		var inside bool = true
		for _, r := range fresh_ranges {
			if first_r < r[0] && last_r >= r[0] && last_r <= r[1] {
				// if first id is lower than first int range
				// but last id is between range
				// e.g 2-4 | 3-5 = 2-2
				last_r = r[0] - 1

			} else if last_r > r[1] && first_r >= r[0] && first_r <= r[1] {
				// if last id is higher than last int range
				// but first id is between range
				// e.g 4-6 | 3-5 = 6-6
				first_r = r[1] + 1

			} else if first_r < r[0] && last_r > r[1] {
				// if there's a range between the range
				// split it and append it to puzzle
				puzzle = append(puzzle, strconv.Itoa(first_r)+"-"+strconv.Itoa(r[0]-1))
				puzzle = append(puzzle, strconv.Itoa(r[1]+1)+"-"+strconv.Itoa(last_r))
				inside = false
				puzzle = puzzle[1:]

			} else if first_r >= r[0] && first_r <= r[1] && last_r >= r[0] && last_r <= r[1] {
				// if range inside of comparing range, skip, has already been added
				inside = false
				puzzle = puzzle[1:]
				break
			}
		}

		if inside {
			if first_r == last_r {
				fresh_counter++
			} else {
				fresh_counter += (last_r - first_r) + 1
			}
			fresh_ranges = append(fresh_ranges, []int{first_r, last_r})
			puzzle = puzzle[1:]
		}
	}
	fmt.Println("Fresh ingredients:", fresh_counter)
}
