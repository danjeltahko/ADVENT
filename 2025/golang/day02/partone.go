package main

import (
	"fmt"
	"strconv"
	"strings"
)

func part_one(puzzle []string) {
	var invalids int = 0

	for _, line := range puzzle {
		// split line by first and last id
		ids := strings.Split(line, "-")
		first_id, _ := strconv.Atoi(ids[0])
		last_id, _ := strconv.Atoi(ids[1])
		// loop through each id
		for id := first_id; id <= last_id; id++ {
			// check if id can be repeated
			id_str := strconv.Itoa(id)
			if len(id_str)%2 == 0 {
				first_seq := id_str[:len(id_str)/2]
				last_seq := id_str[len(id_str)/2:]
				// add id if repeatable
				if first_seq == last_seq {
					invalids += id
				}
			} else {
				continue
			}
		}
	}
	fmt.Println("Repeated pettern:", invalids)
}
