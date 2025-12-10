package main

import (
	"fmt"
	"math"
	"slices"
	"strconv"
	"strings"
)

type Boxes struct {
	p []float64
	q []float64
	d float64
}

func contains(circuit [][]float64, pos []float64) bool {
	/*
		apparently in golang, you cant compare float values..
		therefore need this solution, and tbh this was thanks
		to chatgpt, couldn't find anything online.
	*/
	found := slices.ContainsFunc(circuit, func(x []float64) bool {
		return slices.Equal(x, pos)
	})
	return found
}

func convert(p []string) []float64 {
	/* Convert string array to int array */
	var int_list []float64
	for _, i := range p {
		i_int, _ := strconv.Atoi(i)
		int_list = append(int_list, float64(i_int))
	}
	return int_list
}

func slice_str(xs []float64) string {
	parts := make([]string, len(xs))
	for i, v := range xs {
		parts[i] = fmt.Sprintf("%.1f", v) // -> 2.0, 1.0, 3.6
	}
	return "{" + strings.Join(parts, ", ") + "}"
}

func slice_float(s string) ([]float64, error) {
	s = strings.TrimSpace(s)

	// Require (or at least handle) surrounding braces
	if len(s) < 2 || s[0] != '{' || s[len(s)-1] != '}' {
		return nil, fmt.Errorf("expected {...}, got %q", s)
	}

	inner := strings.TrimSpace(s[1 : len(s)-1])
	if inner == "" {
		return []float64{}, nil
	}

	parts := strings.Split(inner, ",")
	out := make([]float64, 0, len(parts))

	for _, p := range parts {
		p = strings.TrimSpace(p)
		v, err := strconv.ParseFloat(p, 64)
		if err != nil {
			return nil, fmt.Errorf("bad number %q: %w", p, err)
		}
		out = append(out, v)
	}

	return out, nil
}

func distance(p, q []float64) float64 {
	d := math.Sqrt(math.Pow((p[0]-q[0]), 2) + math.Pow((p[1]-q[1]), 2) + math.Pow((p[2]-q[2]), 2))
	// fmt.Println(d, p, " | ", q)
	return d
}

func bubble_sort(array []Boxes) []Boxes {
	for i := 0; i < len(array)-1; i++ {
		for j := 0; j < len(array)-i-1; j++ {
			if array[j].d > array[j+1].d {
				array[j], array[j+1] = array[j+1], array[j]
			}
		}
	}
	return array
}

func quicsort(array []Boxes) []Boxes {
	if len(array) < 2 {
		return array
	} else {
		pivot := array[0]
		var less []Boxes
		for _, box := range array[1:] {
			if box.d <= pivot.d {
				less = append(less, box)
			}
		}
		var greater []Boxes
		for _, box := range array[1:] {
			if box.d > pivot.d {
				greater = append(greater, box)
			}
		}
		less_array := quicsort(less)
		less_array = append(less_array, pivot)
		greater_array := quicsort(greater)
		less_array = append(less_array, greater_array...)
		return less_array
	}
}

func part_one(puzzle [][]string) {

	var junction_boxes []Boxes
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
			converted_q := convert(puzzle[j])
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
	fmt.Println("Created Circuits")

	// sorting the array by shortest distance
	// junction_boxes = bubble_sort(junction_boxes)
	junction_boxes = quicsort(junction_boxes)

	fmt.Println("Sorted Circuits")

	// iterating through each junction box struct
	// in array, to add them to a new array so we
	// can calculate the circuits
	var connection_counter int = 0
	// var circuits_array []Circuit
	for _, box := range junction_boxes {
		fmt.Println(box)
		if connection_counter == 10 {
			break
		}

		if !contains(circuits[slice_str(box.p)], box.q) && !contains(circuits[slice_str(box.q)], box.p) {
			circuits[slice_str(box.p)] = append(circuits[slice_str(box.p)], box.q)
			circuits[slice_str(box.q)] = append(circuits[slice_str(box.q)], box.p)
			connection_counter++
		}

		// if !contains(circuits[slice_str(box.q)], box.p) {
		// 	circuits[slice_str(box.q)] = append(circuits[slice_str(box.q)], box.p)
		// 	connection_counter++
		// }

		// check the first 10 shortest connections

	}

	var clusters [][][]float64
	var checked [][]float64
	fmt.Println("-------------------------------------")
	for key, node := range circuits {
		fmt.Println(key, node)
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
		clusters = append(clusters, cluster)
	}
	fmt.Println("-------------------------------------")
	for _, c := range clusters {
		fmt.Println(c)
	}
	var sizes []int
	for _, s := range clusters {
		sizes = append(sizes, len(s))
	}

	slices.Sort(sizes)
	slices.Reverse(sizes)
	fmt.Println("Result:", (sizes[0] * sizes[1] * sizes[2]))
}
