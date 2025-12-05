package main

import (
	"fmt"
	"strconv"
	"strings"
)

func part_one(puzzle [][]string) {

	var fresh_counter int = 0
	// iterate through all the ids to check
	for _, id := range puzzle[1] {
		// iterate through the ranges with fresh ids
		for _, line := range puzzle[0] {
			// split the range string and convert to int
			splitted_line := strings.Split(line, "-")
			first_r, _ := strconv.Atoi(splitted_line[0])
			last_r, _ := strconv.Atoi(splitted_line[1])
			// convert str id to int
			int_id, _ := strconv.Atoi(id)
			// compare if id is between range
			if int_id >= first_r && int_id <= last_r {
				fresh_counter++
				break
			}
		}
	}
	fmt.Println("Fresh ingredients:", fresh_counter)
}
