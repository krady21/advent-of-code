package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func parse(input string) [][]int {
	lines := strings.Split(input, "\n")
	rows, cols := len(lines), len(lines[0])
	mat := allocMatrix[int](rows, cols)

	for i, line := range lines {
		line = strings.TrimSpace(line)
		for j, ch := range line {
			value, _ := strconv.Atoi(string(ch))
			mat[i][j] = value
		}
	}

	return mat
}

func allocMatrix[T int | bool](rows, cols int) [][]T {
	mat := make([][]T, rows)
	for i := 0; i < rows; i++ {
		mat[i] = make([]T, cols)
	}
	return mat
}

func checkRight(mat [][]int, i, j int) bool {
	for k := j + 1; k < len(mat[i]); k++ {
		if mat[i][j] <= mat[i][k] {
			return false
		}
	}
	return true
}

func checkLeft(mat [][]int, i, j int) bool {
	for k := j - 1; k >= 0; k-- {
		if mat[i][j] <= mat[i][k] {
			return false
		}
	}
	return true
}

func checkUp(mat [][]int, i, j int) bool {
	for k := i + 1; k < len(mat); k++ {
		if mat[i][j] <= mat[k][j] {
			return false
		}
	}
	return true
}

func checkDown(mat [][]int, i, j int) bool {
	for k := i - 1; k >= 0; k-- {
		if mat[i][j] <= mat[k][j] {
			return false
		}
	}
	return true
}

func isVisible(mat [][]int, i, j int) bool {
	if i == 0 || i == len(mat)-1 || j == 0 || j == len(mat[i])-1 {
		return true
	}
	return checkRight(mat, i, j) ||
		checkLeft(mat, i, j) ||
		checkUp(mat, i, j) ||
		checkDown(mat, i, j)
}

func task1(mat [][]int) int {
	var count int
	for i := 0; i < len(mat); i++ {
		for j := 0; j < len(mat[i]); j++ {
			if isVisible(mat, i, j) {
				count++
			}
		}
	}
	return count
}

func checkRight2(mat [][]int, i, j int) int {
	var count int
	for k := j + 1; k < len(mat[i]); k++ {
		if mat[i][j] < mat[i][k] {
			return count
		} else if mat[i][j] == mat[i][k] {
			count++
			return count
		} else {
			count++
		}
	}
	return count
}

func checkLeft2(mat [][]int, i, j int) int {
	var count int
	for k := j - 1; k >= 0; k-- {
		if mat[i][j] < mat[i][k] {
			return count
		} else if mat[i][j] == mat[i][k] {
			count++
			return count
		} else {
			count++
		}
	}
	return count
}

func checkUp2(mat [][]int, i, j int) int {
	var count int
	for k := i + 1; k < len(mat); k++ {
		if mat[i][j] < mat[k][j] {
			return count
		} else if mat[i][j] == mat[k][j] {
			count++
			return count
		} else {
			count++
		}
	}
	return count
}

func checkDown2(mat [][]int, i, j int) int {
	var count int
	for k := i - 1; k >= 0; k-- {
		if mat[i][j] < mat[k][j] {
			return count
		} else if mat[i][j] == mat[k][j] {
			count++
			return count
		} else {
			count++
		}
	}
	return count
}

func scenicScore(mat [][]int, i, j int) int {
	score := checkUp2(mat, i, j) *
		checkDown2(mat, i, j) *
		checkRight2(mat, i, j) *
		checkLeft2(mat, i, j)

	return score
}

func task2(mat [][]int) int {
	var max int
	for i := 0; i < len(mat); i++ {
		for j := 0; j < len(mat[i]); j++ {
			score := scenicScore(mat, i, j)
			if score > max {
				max = score
			}
		}
	}
	return max
}

func main() {
	f, _ := os.ReadFile("input.txt")
	file := string(f)
	file = strings.TrimSpace(file)

	trees := parse(file)
	result1 := task1(trees)
	result2 := task2(trees)

	fmt.Println("Task 1 result is:", result1)
	fmt.Println("Task 2 result is:", result2)
}
