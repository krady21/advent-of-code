package main

import (
	"fmt"
	"github.com/golang-collections/collections/stack"
	"os"
	"strconv"
	"strings"
)

func parse(input string) [][]int {
	var hmap [][]int
	for _, line := range strings.Split(strings.TrimSpace(input), "\n") {
		var l []int
		for _, x := range strings.Split(strings.TrimSpace(line), "") {
			num, _ := strconv.Atoi(x)
			l = append(l, num)
		}
		hmap = append(hmap, l)
	}
	return hmap
}

func isLowPoint(hmap [][]int, i, j int) bool {
	if i-1 >= 0 {
		if hmap[i-1][j] <= hmap[i][j] {
			return false
		}
	}
	if i+1 <= len(hmap)-1 {
		if hmap[i+1][j] <= hmap[i][j] {
			return false
		}
	}
	if j-1 >= 0 {
		if hmap[i][j-1] <= hmap[i][j] {
			return false
		}
	}
	if j+1 <= len(hmap[i])-1 {
		if hmap[i][j+1] <= hmap[i][j] {
			return false
		}
	}
	return true
}

func task1(input string) int {
	hmap := parse(input)

	var sum int
	for i := 0; i < len(hmap); i++ {
		for j := 0; j < len(hmap[i]); j++ {
			if isLowPoint(hmap, i, j) {
				sum += (hmap[i][j] + 1)
			}
		}
	}
	return sum
}

type Point struct {
	i, j int
}

func (p *Point) Neighbors(hmap [][]int, discovered [][]bool) []Point {
	var neighbors []Point
	if p.i-1 >= 0 {
		if hmap[p.i-1][p.j] < 9 && hmap[p.i-1][p.j] > hmap[p.i][p.j] && !discovered[p.i-1][p.j] {
			neighbors = append(neighbors, Point{p.i - 1, p.j})
		}
	}
	if p.i+1 <= len(hmap)-1 {
		if hmap[p.i+1][p.j] < 9 && hmap[p.i+1][p.j] > hmap[p.i][p.j] && !discovered[p.i+1][p.j] {
			neighbors = append(neighbors, Point{p.i + 1, p.j})
		}
	}
	if p.j-1 >= 0 {
		if hmap[p.i][p.j-1] < 9 && hmap[p.i][p.j-1] > hmap[p.i][p.j] && !discovered[p.i][p.j-1] {
			neighbors = append(neighbors, Point{p.i, p.j - 1})
		}
	}
	if p.j+1 <= len(hmap[p.i])-1 {
		if hmap[p.i][p.j+1] < 9 && hmap[p.i][p.j+1] > hmap[p.i][p.j] && !discovered[p.i][p.j+1] {
			neighbors = append(neighbors, Point{p.i, p.j + 1})
		}
	}
	return neighbors
}

func findBasin(hmap [][]int, i, j int) int {
	var count int
	discovered := make([][]bool, len(hmap))
	for i := range discovered {
		discovered[i] = make([]bool, len(hmap[i]))
	}

	// dfs
	st := stack.New()
	st.Push(Point{i, j})
	for st.Len() > 0 {
		v := st.Pop().(Point)
		if !discovered[v.i][v.j] {
			discovered[v.i][v.j] = true
			count += 1
			for _, n := range v.Neighbors(hmap, discovered) {
				st.Push(n)
			}
		}
	}
	return count
}

// l1 > l2 > l3
func update(l1, l2, l3, size int) (int, int, int) {
	if size > l1 {
		l1, l2, l3 = size, l1, l2
	} else if size > l2 {
		l2, l3 = size, l2
	} else if size > l3 {
		l3 = size
	}

	return l1, l2, l3
}

func task2(input string) int {
	hmap := parse(input)

	var l1, l2, l3 = 1, 1, 1
	for i := 0; i < len(hmap); i++ {
		for j := 0; j < len(hmap[i]); j++ {
			if isLowPoint(hmap, i, j) {
				size := findBasin(hmap, i, j)
				l1, l2, l3 = update(l1, l2, l3, size)
			}
		}
	}
	return l1 * l2 * l3
}

func main() {
	contents, _ := os.ReadFile("input.txt")
	input := string(contents)

	fmt.Println("Task 1 result is:", task1(input))
	fmt.Println("Task 2 result is:", task2(input))
}
