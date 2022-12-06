package main

import (
	"fmt"
	"os"
	"strings"
)

// rock, paper, scissors
func task1(round string) int {
	switch round {
	case "A X":
		return 1 + 3
	case "A Y":
		return 2 + 6
	case "A Z":
		return 3 + 0
	case "B X":
		return 1 + 0
	case "B Y":
		return 2 + 3
	case "B Z":
		return 3 + 6
	case "C X":
		return 1 + 6
	case "C Y":
		return 2 + 0
	case "C Z":
		return 3 + 3
	}
	panic("we should not get here")
}

// rock(1), paper(2), scissor(3)
// lose(0), draw(3), win(6)
func task2(round string) int {
	switch round {
	case "A X":
		return 0 + 3
	case "A Y":
		return 3 + 1
	case "A Z":
		return 6 + 2
	case "B X":
		return 0 + 1
	case "B Y":
		return 3 + 2
	case "B Z":
		return 6 + 3
	case "C X":
		return 0 + 2
	case "C Y":
		return 3 + 3
	case "C Z":
		return 6 + 1
	}
	panic("we should not get here")
}

func main() {
	f, _ := os.ReadFile("input.txt")
	file := string(f)

	rounds := strings.Split(strings.TrimSpace(file), "\n")
	var sum1, sum2 int
	for _, round := range rounds {
		round = strings.TrimSpace(round)
		sum1 += task1(round)
		sum2 += task2(round)
	}

	fmt.Println("Task 1 result is: ", sum1)
	fmt.Println("Task 2 result is: ", sum2)
}
