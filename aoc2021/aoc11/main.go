package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Pair struct {
	i, j int
}

func (p *Pair) Neighbors(mat [][]int) []Pair {
	var pairs []Pair

	// last index of line and column
	n := len(mat) - 1
	m := len(mat[0]) - 1

	if p.i-1 >= 0 && p.j-1 >= 0 {
		pairs = append(pairs, Pair{p.i - 1, p.j - 1})
	}
	if p.i-1 >= 0 {
		pairs = append(pairs, Pair{p.i - 1, p.j})
	}
	if p.i-1 >= 0 && p.j+1 <= m {
		pairs = append(pairs, Pair{p.i - 1, p.j + 1})
	}
	if p.j-1 >= 0 {
		pairs = append(pairs, Pair{p.i, p.j - 1})
	}
	if p.j+1 <= m {
		pairs = append(pairs, Pair{p.i, p.j + 1})
	}
	if p.i+1 <= n && p.j-1 >= 0 {
		pairs = append(pairs, Pair{p.i + 1, p.j - 1})
	}
	if p.i+1 <= n {
		pairs = append(pairs, Pair{p.i + 1, p.j})
	}
	if p.i+1 <= n && p.j+1 <= m {
		pairs = append(pairs, Pair{p.i + 1, p.j + 1})
	}
	return pairs
}

func parse(input string) [][]int {
	var m [][]int
	for _, line := range strings.Split(strings.TrimSpace(input), "\n") {
		var l []int
		for _, ch := range strings.Split(strings.TrimSpace(line), "") {
			n, _ := strconv.Atoi(ch)
			l = append(l, n)
		}
		m = append(m, l)
	}
	return m
}

func flash(m [][]int, i, j int, flashed [][]bool) {
	m[i][j] = 0
	flashed[i][j] = true

	for _, p := range (&Pair{i, j}).Neighbors(m) {
		if !flashed[p.i][p.j] {
			m[p.i][p.j]++
		}
		if m[p.i][p.j] > 9 && !flashed[p.i][p.j] {
			flash(m, p.i, p.j, flashed)
		}
	}
}

func count(m [][]bool) int {
	var c int
	for _, l := range m {
		for _, b := range l {
			if b {
				c++
			}
		}
	}
	return c
}

func simulate(m [][]int) int {
	flashed := make([][]bool, len(m))
	for i := range flashed {
		flashed[i] = make([]bool, len(m[i]))
	}

	for i := 0; i < len(m); i++ {
		for j := 0; j < len(m[i]); j++ {
			m[i][j]++
		}
	}

	for i := 0; i < len(m); i++ {
		for j := 0; j < len(m[i]); j++ {
			if m[i][j] > 9 && !flashed[i][j] {
				flash(m, i, j, flashed)
			}
		}
	}

	return count(flashed)
}

func task1(input string, steps int) int {
	energies := parse(input)

	var flashes int
	for i := 0; i < steps; i++ {
		flashes += simulate(energies)
	}
	return flashes
}

func task2(input string) int {
	energies := parse(input)
	all := len(energies) * len(energies[0])

	for i := 1; ; i++ {
		flashes := simulate(energies)
		if flashes == all {
			return i
		}
	}
}
func main() {
	contents, _ := os.ReadFile("input.txt")
	input := string(contents)

	fmt.Println("Task 1 result is:", task1(input, 100))
	fmt.Println("Task 2 result is:", task2(input))
}
