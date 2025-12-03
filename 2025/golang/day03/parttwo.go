package main

import (
	"fmt"
	"strconv"
)

func part_two(puzzle []string) {
	var total_joltage int = 0

	// iterate through each line
	for _, line := range puzzle {

		// create array out of line
		var batteries []int
		for i := 0; i < len(line); i++ {
			int_value, _ := strconv.Atoi(string(line[i]))
			batteries = append(batteries, int_value)
		}

		var highest int = 0
		var highest_idx int = 0
		var last_idx int = 0
		var bank []int

		// add a battery 12 times
		for range 12 {
			// check for highest number while not checking possible numbers
			for i := last_idx; i < len(batteries)-(11-len(bank)); i++ {
				if batteries[i] > highest {
					highest = int(batteries[i])
					highest_idx = i
				}
			}
			bank = append(bank, batteries[highest_idx])
			last_idx = highest_idx + 1
			highest = 0
		}

		var joined_bank string = ""
		for i := 0; i < len(bank); i++ {
			joined_bank += strconv.Itoa(bank[i])
		}

		joltage, _ := strconv.Atoi(joined_bank)
		total_joltage += joltage
	}
	fmt.Println("Total Joltage:", total_joltage)
}
