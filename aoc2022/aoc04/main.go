package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	f, _ := os.ReadFile("input.txt")
	file := string(f)

	lines := strings.Split(strings.TrimSpace(file), "\n")

	var containing, overlapping int
	for _, line := range lines {
		line = strings.TrimSpace(line)
		elfs := strings.Split(line, ",")

		elf1 := strings.Split(elfs[0], "-")
		elf1Start, _ := strconv.Atoi(elf1[0])
		elf1End, _ := strconv.Atoi(elf1[1])

		elf2 := strings.Split(elfs[1], "-")
		elf2Start, _ := strconv.Atoi(elf2[0])
		elf2End, _ := strconv.Atoi(elf2[1])

		if (elf1Start >= elf2Start && elf1End <= elf2End) ||
			(elf2Start >= elf1Start && elf2End <= elf1End) {
			containing++
		}

		start := int(math.Max(float64(elf1Start), float64(elf2Start)))
		end := int(math.Min(float64(elf1End), float64(elf2End)))
		if start <= end {
			overlapping += 1
		}
	}

	fmt.Println("Task 1 result is: ", containing)
	fmt.Println("Task 2 result is: ", overlapping)
}
