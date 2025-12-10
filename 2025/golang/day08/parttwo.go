package main

import (
	"fmt"
)

func check_circuit(circuits map[string][][]float64, all_boxes int) bool {
	var clusters [][][]float64
	var checked [][]float64
	for key, node := range circuits {
		// fmt.Println(key, node)
		// convert back to float slice
		if len(node) < 1 {
			continue
		}
		float_key, _ := slice_float(key)
		// fmt.Println(str_key)
		var cluster [][]float64
		cluster = append(cluster, float_key)
		checked = append(checked, float_key)

		var queue [][]float64 = node
		for len(queue) > 0 {
			// fmt.Println("Length:", len(queue), queue)
			if !contains(cluster, queue[0]) && !contains(checked, queue[0]) {
				cluster = append(cluster, queue[0])
				checked = append(checked, queue[0])
				for _, n := range circuits[slice_str(queue[0])] {
					queue = append(queue, n)
				}
			}
			queue = queue[1:]
		}
		if len(cluster) == all_boxes {
			return true
		}
		// Here check if its only one cluster!
		// also need to add how many boxes exists
		clusters = append(clusters, cluster)
	}
	return false
}

func part_two(puzzle [][]string) {

	var junction_boxes []Boxes
	var all_junction_boxes [][]float64

	// var circuits [][][]float64
	var circuits = make(map[string][][]float64)

	// iterate through each line of the puzzle
	// and skip last line, because it will be
	// compared with j index (i, j := i+1)
	for i := 0; i < len(puzzle)-1; i++ {
		// compare with not yet compared (j := i + 1)
		for j := i + 1; j < len(puzzle); j++ {
			// convert all string items in array to float
			converted_p := convert(puzzle[i])
			if !contains(all_junction_boxes, converted_p) {
				all_junction_boxes = append(all_junction_boxes, converted_p)
			}
			converted_q := convert(puzzle[j])
			if !contains(all_junction_boxes, converted_q) {
				all_junction_boxes = append(all_junction_boxes, converted_q)
			}
			// adding the array as string to map as key
			circuits[slice_str(converted_p)] = [][]float64{}
			circuits[slice_str(converted_q)] = [][]float64{}
			// get the distance between the two points
			d := distance(converted_p, converted_q)
			// append new struct to array with the
			// two points and their distance
			junction_boxes = append(junction_boxes, Boxes{converted_p, converted_q, d})
		}
	}
	// sorting the array by shortest distance
	// junction_boxes = bubble_sort(junction_boxes)
	junction_boxes = quicsort(junction_boxes)

	// iterating through each junction box struct
	// in array, to add them to a new array so we
	// can calculate the circuits
	for i, box := range junction_boxes {
		if !contains(circuits[slice_str(box.p)], box.q) && !contains(circuits[slice_str(box.q)], box.p) {
			circuits[slice_str(box.p)] = append(circuits[slice_str(box.p)], box.q)
			circuits[slice_str(box.q)] = append(circuits[slice_str(box.q)], box.p)
		}

		if i+1/2 >= len(all_junction_boxes) {
			if check_circuit(circuits, len(all_junction_boxes)) {
				fmt.Println("Everything is one")
				fmt.Println(box.p, " | ", box.q)
				fmt.Println("Result:", box.p[0]*box.q[0])
				break
			}
		}
	}
}
