package main

import (
	"fmt"
	"strconv"
	"strings"
)

func All(array []string) bool {
	for i := 1; i < len(array); i++ {
		if array[i] != array[0] {
			return false
		}
	}
	return true
}

func part_two(puzzle []string) {
	var invalids int = 0

	for _, line := range puzzle {
		// split line by first and last id
		ids := strings.Split(line, "-")
		first_id, _ := strconv.Atoi(ids[0])
		last_id, _ := strconv.Atoi(ids[1])
		// loop through each id
		for id := first_id; id <= last_id; id++ {

			// convert id to string to count digits
			id_str := strconv.Itoa(id)

			// iterate through dividable
			for div := 2; div <= len(id_str); div++ {
				if len(id_str)%div == 0 {
					// splitting string based on divider
					sep := len(id_str) / div
					var seqs []string
					for n := 0; n < len(id_str); n += (sep) {
						seqs = append(seqs, id_str[n:n+sep])
					}
					// add id if all items are same in slice
					if All(seqs) {
						invalids += id
						break
					}
				}

			}
		}
	}
	fmt.Println("Repeated pettern:", invalids)
}
