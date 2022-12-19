package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

const (
	Air int = iota
	Rock
	Sand
)

var startx, starty int = 500, 0

func getPos(p string) (int, int) {
	split := strings.Split(p, ",")
	x, _ := strconv.Atoi(split[0])
	y, _ := strconv.Atoi(split[1])

	return x, y
}

func addRock(scan [][]int, p1, p2 string) [][]int {
	p1x, p1y := getPos(p1)
	p2x, p2y := getPos(p2)

	var min, max int
	if p1x == p2x {
		min = int(math.Min(float64(p1y), float64(p2y)))
		max = int(math.Max(float64(p1y), float64(p2y)))
		for i := min; i <= max; i++ {
			scan[p1x][i] = Rock
		}
	} else if p1y == p2y {
		min = int(math.Min(float64(p1x), float64(p2x)))
		max = int(math.Max(float64(p1x), float64(p2x)))
		for i := min; i <= max; i++ {
			scan[i][p1y] = Rock
		}
	} 
	return scan
}

func parse(input string) [][]int {
	lines := strings.Split(input, "\n")

	scan := make([][]int, 1000)
	for i := range scan {
		scan[i] = make([]int, 200)
	}

	for _, line := range lines {
		points := strings.Split(line, " -> ")

		for i := 0; i < len(points)-1; i++ {
			scan = addRock(scan, points[i], points[i+1])
		}
	}
	return scan
}

func lowestRock(scan [][]int) int {
	for j := len(scan[1]) - 1; j > 0; j-- {
		for i := 0; i < len(scan); i++ {
			if scan[i][j] == Rock {
				return j
			}
		}
	}
	panic("should not get here")
}

func task1(scan [][]int) int {
	highesty := lowestRock(scan)
	return simulate(scan, func(y int) bool {
		return y <= highesty
	})
}

func task2(scan [][]int) int {
	return simulate(scan, func(_ int) bool {
		return scan[startx][starty] != Sand
	})
}

func simulate(scan [][]int, stopFn func(currentY int) bool) int {
	highesty := lowestRock(scan)
	bottom := highesty + 2
	for i := 0; i < len(scan); i++ {
		scan[i][bottom] = Rock
	}

	var count int
	for x, y := startx, starty; stopFn(y); {
		if scan[x][y+1] == Air {
			y = y + 1
		} else if scan[x-1][y+1] == Air {
			x, y = x-1, y+1
		} else if scan[x+1][y+1] == Air {
			x, y = x+1, y+1
		} else {
			scan[x][y] = Sand
			x, y = startx, starty
			count += 1
		}
	}
	return count
}

func main() {
	f, _ := os.ReadFile("input.txt")
	file := string(f)
	file = strings.TrimSpace(file)

	matrix := parse(file)
	units := task1(matrix)

	matrix = parse(file)
	result2 := task2(matrix)

	fmt.Println("Task 1 result is:", units)
	fmt.Println("Task 2 result is:", result2)
}
