package main

import (
	"fmt"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

func parse(input string) []int {
	var numbers []int

	split := strings.Split(input, ",")
	for _, el := range split {
		s := strings.TrimFunc(el, func(r rune) bool { return r == ' ' || r == '\n' })
		num, err := strconv.Atoi(s)
		if err == nil {
			numbers = append(numbers, num)
		}
	}
	return numbers
}

func task1(input string) int {
	var median, count int

	positions := parse(input)
	sort.Ints(positions)

	if len(positions)%2 != 0 {
		median = positions[len(positions)/2]
	} else {
		median = (positions[(len(positions)-1)/2] + positions[len(positions)/2]) / 2
	}

	for _, pos := range positions {
		count += int(math.Abs(float64(pos - median)))
	}
	return count
}

func Sum(arr []int) int {
	var sum int
	for _, el := range arr {
		sum += el
	}
	return sum
}

func task2(input string) int {
	var count int

	positions := parse(input)
	sort.Ints(positions)

	mean := int(math.Round(float64(Sum(positions)) / float64(len(positions))))

	for _, pos := range positions {
		n := int(math.Abs(float64(pos - mean)))
		count += int(math.Floor(float64(n*(n+1)) / 2))
	}
	return count
}

func main() {
	contents, _ := os.ReadFile("input.txt")
	input := string(contents)

	fmt.Println("Task 1 result is:", task1(input))
	fmt.Println("Task 2 result is:", task2(input))
}
