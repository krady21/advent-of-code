package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
)

func task1(measurements []int) int {
	var nrIncreases int
	for i := 1; i < len(measurements); i++ {
		if measurements[i] > measurements[i-1] {
			nrIncreases++
		}
	}
	return nrIncreases
}

func task2(measurements []int) int {
	var nrIncreases int
	first := math.MaxInt
	for i := 1; i < len(measurements)-1; i++ {
		second := measurements[i-1] + measurements[i] + measurements[i+1]
		if second > first {
			nrIncreases++
		}
		first = second
	}
	return nrIncreases
}

func parseFile(name string) []int {
	file, err := os.Open(name)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var measurements []int
	for scanner.Scan() {
		m, _ := strconv.Atoi(scanner.Text())
		measurements = append(measurements, m)
	}
	return measurements
}

func main() {
	measurements := parseFile("input.txt")

	fmt.Println("Task 1 result is:", task1(measurements))
	fmt.Println("Task 2 result is:", task2(measurements))
}
