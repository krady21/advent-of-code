package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Action struct {
	direction string
	value     int
}

func task1(actions []Action) int {
	var horizontal, depth int
	for _, a := range actions {
		switch a.direction {
		case "forward":
			horizontal += a.value
		case "down":
			depth += a.value
		case "up":
			depth -= a.value
		}
	}

	return horizontal * depth
}

func task2(actions []Action) int {
	var horizontal, depth, aim int
	for _, a := range actions {
		switch a.direction {
		case "forward":
			horizontal += a.value
			depth += a.value * aim
		case "down":
			aim += a.value
		case "up":
			aim -= a.value
		}
	}

	return horizontal * depth
}
func parseFile(name string) []Action {
	file, err := os.Open(name)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var actions []Action
	for scanner.Scan() {
		items := strings.Fields(scanner.Text())

		distance, _ := strconv.Atoi(items[1])
		actions = append(actions, Action{items[0], distance})
	}
	return actions
}

func main() {
	actions := parseFile("input.txt")
	fmt.Println("Task 1 result is:", task1(actions))
	fmt.Println("Task 2 result is:", task2(actions))
}
