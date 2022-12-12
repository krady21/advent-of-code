package main

import (
	"fmt"
	"math"
	"os"
	"strings"
)

type Pos struct {
	x, y int
}

func parse(input string) ([][]rune, Pos, Pos, []Pos) {
	var start, end Pos
	var small []Pos
	lines := strings.Split(input, "\n")
	hmap := make([][]rune, len(lines))

	for i, line := range lines {
		hmap[i] = make([]rune, len(lines[i]))
		for j, ch := range line {
			switch ch {
			case 'S':
				start.x, start.y, ch = i, j, 'a'
			case 'E':
				end.x, end.y, ch = i, j, 'z'
			case 'a':
				small = append(small, Pos{i, j})
			}
			hmap[i][j] = ch
		}
	}
	return hmap, start, end, small
}

func walkableNeighbors(hmap [][]rune, p Pos) []Pos {
	neighbors := make([]Pos, 0, 4)
	if p.x < len(hmap)-1 && hmap[p.x+1][p.y]-hmap[p.x][p.y] <= 1 {
		neighbors = append(neighbors, Pos{x: p.x + 1, y: p.y})
	}
	if p.y < len(hmap[1])-1 && hmap[p.x][p.y+1]-hmap[p.x][p.y] <= 1 {
		neighbors = append(neighbors, Pos{x: p.x, y: p.y + 1})
	}
	if p.x > 0 && hmap[p.x-1][p.y]-hmap[p.x][p.y] <= 1 {
		neighbors = append(neighbors, Pos{x: p.x - 1, y: p.y})
	}
	if p.y > 0 && hmap[p.x][p.y-1]-hmap[p.x][p.y] <= 1 {
		neighbors = append(neighbors, Pos{x: p.x, y: p.y - 1})
	}

	return neighbors
}

func bfs(hmap [][]rune, start, end Pos) int {
	visited := make(map[Pos]bool)
	visited[start] = true
	distance := make(map[Pos]int)
	distance[start] = 0
	queue := []Pos{start}

	for ; len(queue) > 0; queue = queue[1:] {
		iter := queue[0]
		for _, neighbor := range walkableNeighbors(hmap, iter) {
			if visited[neighbor] {
				continue
			}

			visited[neighbor] = true
			distance[neighbor] = distance[iter] + 1
			queue = append(queue, neighbor)

			if neighbor == end {
				break
			}
		}
	}

	return distance[end]
}

func task1(hmap [][]rune, start, end Pos) int {
	return bfs(hmap, start, end)
}

func task2(hmap [][]rune, small []Pos, end Pos) int {
	min := math.MaxInt
	for _, start := range small {
		if hmap[start.x][start.y] == 'a' {
			dist := bfs(hmap, start, end)
			if dist != 0 && dist < min {
				min = dist
			}
		}
	}
	return min
}

func main() {
	f, _ := os.ReadFile("input.txt")
	file := string(f)
	file = strings.TrimSpace(file)

	hmap, start, end, small := parse(file)
	result1 := task1(hmap, start, end)
	result2 := task2(hmap, small, end)

	fmt.Println("Task 1 result is:", result1)
	fmt.Println("Task 2 result is:", result2)
}
