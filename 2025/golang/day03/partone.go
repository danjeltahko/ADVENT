package main

import (
	"fmt"
	"strconv"
)

func part_one(puzzle []string) {
	var total_joltage int = 0

	// iterate through each line
	for _, line := range puzzle {

		// create array out of line
		var batteries []int
		for i := 0; i < len(line); i++ {
			int_value, _ := strconv.Atoi(string(line[i]))
			batteries = append(batteries, int_value)
		}

		var joltage int = 0

		// iterate index ranges for first digit
		for i := range len(batteries) {
			// check second digit
			for j := i + 1; j < len(batteries); j++ {
				// put string digits together and convert to int
				test_jolt, _ := strconv.Atoi(strconv.Itoa(batteries[i]) + strconv.Itoa(batteries[j]))
				if test_jolt > joltage {
					joltage = test_jolt
				}
			}
		}
		total_joltage += joltage
	}
	fmt.Println("Repeated pettern:", total_joltage)
}
