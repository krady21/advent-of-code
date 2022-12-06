package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func getCrates() [][]rune {
	stacks := make([][]rune, 10)
	stacks[1] = []rune{'T', 'D', 'W', 'Z', 'V', 'P'}
	stacks[2] = []rune{'L', 'S', 'W', 'V', 'F', 'J', 'D'}
	stacks[3] = []rune{'Z', 'M', 'L', 'S', 'V', 'T', 'B', 'H'}
	stacks[4] = []rune{'R', 'S', 'J'}
	stacks[5] = []rune{'C', 'Z', 'B', 'G', 'F', 'M', 'L', 'W'}
	stacks[6] = []rune{'Q', 'W', 'V', 'H', 'Z', 'R', 'G', 'B'}
	stacks[7] = []rune{'V', 'J', 'P', 'C', 'B', 'D', 'N'}
	stacks[8] = []rune{'P', 'T', 'B', 'Q'}
	stacks[9] = []rune{'H', 'G', 'Z', 'R', 'C'}

	return stacks
}

func task1(instructions string) string {
	stacks := getCrates()

	for _, line := range strings.Split(instructions, "\n") {
		line = strings.TrimSpace(line)
		instruction := strings.Split(line, " ")

		crates, _ := strconv.Atoi(instruction[1])
		source, _ := strconv.Atoi(instruction[3])
		dest, _ := strconv.Atoi(instruction[5])

		for i := 0; i < crates; i++ {
			index := len(stacks[source]) - 1
			top := stacks[source][index]             // get top item
			stacks[dest] = append(stacks[dest], top) // move to different crate
			stacks[source] = stacks[source][:index]  // pop
		}
	}

	var result strings.Builder
	for i := 1; i <= 9; i++ {
		s := stacks[i]
		result.WriteRune(s[len(s)-1])
	}
	return result.String()
}

func task2(instructions string) string {
	stacks := getCrates()

	for _, line := range strings.Split(instructions, "\n") {
		line = strings.TrimSpace(line)
		instruction := strings.Split(line, " ")

		crates, _ := strconv.Atoi(instruction[1])
		source, _ := strconv.Atoi(instruction[3])
		dest, _ := strconv.Atoi(instruction[5])

		for i := crates; i > 0; i-- {
			index := len(stacks[source]) - i // get items in order
			element := stacks[source][index]
			stacks[dest] = append(stacks[dest], element) // move to different crate
		}
		for i := 0; i < crates; i++ {
			stacks[source] = stacks[source][:len(stacks[source])-1] // pop
		}
	}

	var result strings.Builder
	for i := 1; i <= 9; i++ {
		s := stacks[i]
		result.WriteRune(s[len(s)-1])
	}
	return result.String()
}

func main() {
	f, _ := os.ReadFile("input.txt")
	file := string(f)
	split := strings.Split(strings.TrimSpace(file), "\n\n")
	instructions := split[1]

	result1 := task1(instructions)
	result2 := task2(instructions)

	fmt.Println("Task 1 result is: ", result1)
	fmt.Println("Task 1 result is: ", result2)
	// fmt.Println("Task 2 result is: ", overlapping)
}
