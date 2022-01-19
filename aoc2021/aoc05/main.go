package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

type Line struct {
	x1, y1 int
	x2, y2 int
}

func rowsColumns(lines []Line) (int, int) {
	var rows, columns int

	for _, line := range lines {
		if line.x1 > rows {
			rows = line.x1
		}
		if line.x2 > rows {
			rows = line.x2
		}
		if line.y1 > columns {
			columns = line.y1
		}
		if line.y2 > columns {
			columns = line.y2
		}
	}
	return rows + 1, columns + 1
}

func task1(input []Line) int {
	x, y := rowsColumns(input)
	diagram := make([][]int, x)
	for i := range diagram {
		diagram[i] = make([]int, y)
	}

	for _, line := range input {
		if line.x1 == line.x2 {
			max := int(math.Max(float64(line.y1), float64(line.y2)))
			min := int(math.Min(float64(line.y1), float64(line.y2)))
			for i := min; i <= max; i++ {
				diagram[line.x1][i]++
			}
		} else if line.y1 == line.y2 {
			max := int(math.Max(float64(line.x1), float64(line.x2)))
			min := int(math.Min(float64(line.x1), float64(line.x2)))
			for i := min; i <= max; i++ {
				diagram[i][line.y1]++
			}
		}
	}

	var overlapped int
	for _, arr := range diagram {
		for _, val := range arr {
			if val >= 2 {
				overlapped++
			}
		}
	}
	return overlapped
}

func task2(input []Line) int {
	x, y := rowsColumns(input)
	diagram := make([][]int, x)
	for i := range diagram {
		diagram[i] = make([]int, y)
	}

	for _, line := range input {
		if line.x1 == line.x2 {
			max := int(math.Max(float64(line.y1), float64(line.y2)))
			min := int(math.Min(float64(line.y1), float64(line.y2)))

			for i := min; i <= max; i++ {
				diagram[line.x1][i]++
			}
		} else if line.y1 == line.y2 {
			max := int(math.Max(float64(line.x1), float64(line.x2)))
			min := int(math.Min(float64(line.x1), float64(line.x2)))

			for i := min; i <= max; i++ {
				diagram[i][line.y1]++
			}
		} else if math.Abs(float64(line.x1-line.x2)) == math.Abs(float64(line.y1-line.y2)) {
			if line.x1 < line.x2 && line.y1 < line.y2 {
				for i, j := line.x1, line.y1; i <= line.x2 && j <= line.y2; i, j = i+1, j+1 {
					diagram[i][j]++
				}
			} else if line.x1 > line.x2 && line.y1 > line.y2 {
				for i, j := line.x1, line.y1; i >= line.x2 && j >= line.y2; i, j = i-1, j-1 {
					diagram[i][j]++
				}
			} else if line.x1 < line.x2 && line.y1 > line.y2 {
				for i, j := line.x1, line.y1; i <= line.x2 && j >= line.y2; i, j = i+1, j-1 {
					diagram[i][j]++
				}
			} else if line.x1 > line.x2 && line.y1 < line.y2 {
				for i, j := line.x1, line.y1; i >= line.x2 && j <= line.y2; i, j = i-1, j+1 {
					diagram[i][j]++
				}
			}
		}
	}

	var overlapped int
	for _, arr := range diagram {
		for _, val := range arr {
			if val >= 2 {
				overlapped++
			}
		}
	}
	return overlapped

}

func parse(contents string) []Line {
	var lines []Line

	// TODO: Find out why strings.Split creates one
	// more empty string at the end
	split := strings.Split(contents, "\n")
	split = split[:len(split)-1]

	for _, line := range split {
		value := strings.FieldsFunc(line, func(r rune) bool {
			return r == ',' || r == '-' || r == ' ' || r == '>'
		})

		x1, _ := strconv.Atoi(value[0])
		y1, _ := strconv.Atoi(value[1])
		x2, _ := strconv.Atoi(value[2])
		y2, _ := strconv.Atoi(value[3])

		lines = append(lines, Line{x1, y1, x2, y2})
	}

	return lines
}

func main() {
	contents, _ := os.ReadFile("input.txt")
	input := parse(string(contents))

	fmt.Println("Task 1 result is:", task1(input))
	fmt.Println("Task 2 result is:", task2(input))
}
