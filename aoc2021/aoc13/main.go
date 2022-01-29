package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Pair struct {
	x, y int
}

type Fold struct {
	dir   string
	value int
}

func parse(input string) (map[Pair]bool, []Fold) {
	split := strings.Split(input, "\n\n")

	pairs := make(map[Pair]bool)
	for _, line := range strings.Split(strings.TrimSpace(split[0]), "\n") {
		dots := strings.Split(strings.TrimSpace(line), ",")
		x, _ := strconv.Atoi(dots[0])
		y, _ := strconv.Atoi(dots[1])

		pairs[Pair{x: y, y: x}] = true
	}

	var folds []Fold
	for _, line := range strings.Split(strings.TrimSpace(split[1]), "\n") {
		line = strings.TrimSuffix(strings.TrimPrefix(line, "fold along "), "\n")
		spl := strings.Split(line, "=")
		val, _ := strconv.Atoi(spl[1])

		folds = append(folds, Fold{spl[0], val})
	}

	return pairs, folds
}

func simulate(paper map[Pair]bool, f Fold) {
	for p := range paper {
		if f.dir == "x" && p.y > f.value {
			q := Pair{p.x, f.value - (p.y - f.value)}
			paper[q] = true
			delete(paper, p)
		} else if f.dir == "y" && p.x > f.value {
			q := Pair{f.value - (p.x - f.value), p.y}
			paper[q] = true
			delete(paper, p)
		}
	}
}

func max(paper map[Pair]bool) (int, int) {
	var mx, my int
	for p := range paper {
		if p.x > mx {
			mx = p.x
		}
		if p.y > my {
			my = p.y
		}
	}
	return mx + 1, my + 1
}

func task1(input string) int {
	paper, folds := parse(input)
	simulate(paper, folds[0])
	return len(paper)
}

func task2(input string) {
	paper, folds := parse(input)
	for _, f := range folds {
		simulate(paper, f)
	}

	x, y := max(paper)
	mat := make([][]string, x)
	for i := range mat {
		mat[i] = make([]string, y)
		for j := range mat[i] {
			mat[i][j] = " "
		}
	}

	for p := range paper {
		mat[p.x][p.y] = "#"
	}

	for i := range mat {
		for j := range mat[i] {
			fmt.Printf("%s ", mat[i][j])
		}
		fmt.Printf("\n")
	}
}

// x=... fold left
// y=... fold up
func main() {
	contents, _ := os.ReadFile("input.txt")
	input := string(contents)

	fmt.Println("Task 1 result is:", task1(input))
	fmt.Println("Task 2 result is:")
	task2(input)
}
