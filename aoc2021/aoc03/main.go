package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func task1(input []string) int {
	var g, e string

	for i := 0; i < len(input[0]); i++ {
		var zeros int
		for _, num := range input {
			if num[i] == '0' {
				zeros++
			}
		}

		if zeros > len(input)-zeros {
			g += "0"
			e += "1"
		} else {
			g += "1"
			e += "0"
		}

	}

	gamma, _ := strconv.ParseInt(g, 2, 0)
	epsilon, _ := strconv.ParseInt(e, 2, 0)

	return int(gamma * epsilon)
}

func task2(input []string) int {
	numbers := make([]string, len(input))

	copy(numbers, input)
	for i := 0; len(numbers) > 1; i++ {
		var zeros int
		for _, num := range numbers {
			if num[i] == '0' {
				zeros++
			}
		}

		// keep most common; if equal keep 1
		ones := len(numbers) - zeros
		if ones >= zeros {
			numbers = keep(numbers, i, '1')
		} else {
			numbers = keep(numbers, i, '0')
		}
	}

	oxygen, _ := strconv.ParseInt(numbers[0], 2, 0)

	numbers = make([]string, len(input))
	copy(numbers, input)
	for i := 0; len(numbers) > 1; i++ {
		var zeros int
		for _, num := range numbers {
			if num[i] == '0' {
				zeros++
			}
		}

		// keep least common; if equal keep 0
		ones := len(numbers) - zeros
		if ones >= zeros {
			numbers = keep(numbers, i, '0')
		} else {
			numbers = keep(numbers, i, '1')
		}
	}

	co2, _ := strconv.ParseInt(numbers[0], 2, 0)

	return int(oxygen * co2)
}

// Keeps elements from the slice that have the val character at the index position
func keep(slice []string, index int, val byte) []string {
	var newSlice []string
	for _, number := range slice {
		if number[index] == val {
			newSlice = append(newSlice, number)
		}
	}

	return newSlice
}

func parseFile(name string) []string {
	file, err := os.Open(name)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	var input []string

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		input = append(input, scanner.Text())
	}
	return input
}

func main() {
	input := parseFile("input.txt")
	fmt.Println("Task 1 result is:", task1(input))
	fmt.Println("Task 2 result is:", task2(input))
}
