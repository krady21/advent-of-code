package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	var max1, max2, max3 int
	f, _ := os.ReadFile("input.txt")
	file := string(f)

	elfs := strings.Split(strings.TrimSpace(file), "\n\n")
	for _, elf := range elfs {
		elf = strings.TrimSpace(elf)
		calories := strings.Split(elf, "\n")

		var sum int
		for _, line := range calories {
			line = strings.TrimSpace(line)
			calorie, _ := strconv.Atoi(line)
			sum += calorie
		}

		if sum > max1 {
			max3 = max2
			max2 = max1
			max1 = sum
		} else if sum > max2 {
			max3 = max2
			max2 = sum
		} else if sum > max3 {
			max3 = sum
		}
	}

	fmt.Println("Task 1 result is: ", max1)
	fmt.Println("Task 2 result is: ", max1 + max2 + max3)
}
