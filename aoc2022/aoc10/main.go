package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func parse(input string) []int {
	lines := strings.Split(input, "\n")
	strength := make([]int, len(lines)*2)
	strength[0] = 1

	i := 0
	for _, line := range lines {
		split := strings.Split(line, " ")
		op := split[0]

		switch op {
		case "noop":
			strength[i+1] = strength[i]
			i += 1
		case "addx":
			value, _ := strconv.Atoi(split[1])
			strength[i+1] = strength[i]
			strength[i+2] = strength[i] + value
			i += 2
		}
	}
	return strength
}

func task1(strength []int) int {
	cycles := []int{20, 60, 100, 140, 180, 220}
	var sum int
	for _, cycle := range cycles {
		sum += cycle * strength[cycle-1]
	}
	return sum
}

func task2(positions []int) string {
	var crt strings.Builder
	for i, center := range positions {
		pixel := i % 40
		if pixel == center-1 || pixel == center || pixel == center+1 {
			crt.WriteRune('#')
		} else {
			crt.WriteRune('.')
		}
		if pixel == 39 {
			crt.WriteRune('\n')
		}

		if i >= 240 {
			break
		}
	}
	return crt.String()
}

func main() {
	f, _ := os.ReadFile("input.txt")
	file := string(f)
	file = strings.TrimSpace(file)

	strength := parse(file)
	result1 := task1(strength)
	result2 := task2(strength)

	fmt.Println("Task 1 result is:", result1)
	fmt.Printf("Task 2 result is:\n%s", result2)
}
