package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
	"sync"
)

type Board struct {
	m [5][5]int
}

func removeEmpty(list []string) []string {
	var copy []string
	for _, s := range list {
		if s != "" && s != "\n" {
			copy = append(copy, s)
		}
	}
	return copy
}

func parse(contents string) ([]int, []Board) {
	input := strings.Split(contents, "\n\n")
	numbers := strings.Split(input[0], ",")

	// parse the chosen numbers
	chosen := make([]int, len(numbers))
	for _, n := range numbers {
		num, err := strconv.Atoi(strings.TrimFunc(n, func(r rune) bool {
			return r == ' ' || r == '\n'
		}))
		if err == nil {
			chosen = append(chosen, num)
		}
	}

	// parse the bingo boards
	var boards []Board
	for _, b := range input[1:] {
		var matrix [5][5]int

		lines := strings.Split(b, "\n")
		lines = removeEmpty(lines)
		for i, line := range lines {
			elements := strings.Split(line, " ")
			elements = removeEmpty(elements)
			for j, element := range elements {
				matrix[i][j], _ = strconv.Atoi(strings.TrimFunc(element, func(r rune) bool {
					return r == ' ' || r == '\n'
				}))
			}
		}
		boards = append(boards, Board{m: matrix})
	}

	return chosen, boards
}

func contains(arr []int, el int) int {
	for i, a := range arr {
		if a == el {
			return i
		}
	}
	return -1
}

func maxArr(arr []int) int {
	var max int
	for _, a := range arr {
		if a > max {
			max = a
		}
	}
	return max
}

func checkBoard(board *Board, chosen []int, ch chan<- int) {
	lastIndex := len(chosen) - 1
	min := lastIndex

	for i := 0; i < 5; i++ {
		var indexesL, indexesC []int
		for j := 0; j < 5; j++ {
			if ret := contains(chosen, board.m[i][j]); ret >= 0 {
				indexesL = append(indexesL, ret)
			}

			if ret := contains(chosen, board.m[j][i]); ret >= 0 {
				indexesC = append(indexesC, ret)
			}
		}

		// if the line has a faster bingo
		maxL := maxArr(indexesL)
		if len(indexesL) == 5 && maxL < min {
			min = maxL
		}

		// if the column has a faster bingo
		maxC := maxArr(indexesC)
		if len(indexesC) == 5 && maxC < min {
			min = maxC
		}
	}
	ch <- min
}

func checkBoard2(board *Board, chosen []int, ch chan<- int) {
	lastIndex := len(chosen) - 1
	max := lastIndex

	for i := 0; i < 5; i++ {
		var indexesL, indexesC []int
		for j := 0; j < 5; j++ {
			if ret := contains(chosen, board.m[i][j]); ret >= 0 {
				indexesL = append(indexesL, ret)
			}

			if ret := contains(chosen, board.m[j][i]); ret >= 0 {
				indexesC = append(indexesC, ret)
			}
		}

		// if the line has a faster bingo
		maxL := maxArr(indexesL)
		if len(indexesL) == 5 && maxL > max {
			max = maxL
		}

		// if the column has a faster bingo
		maxC := maxArr(indexesC)
		if len(indexesC) == 5 && maxC > max {
			max = maxC
		}
	}
	ch <- max
}

func calculateScore(board *Board, chosen []int) int {
	var sum int
	for i := 0; i < 5; i++ {
		for j := 0; j < 5; j++ {
			if contains(chosen, board.m[i][j]) == -1 {
				sum += board.m[i][j]
			}
		}
	}
	return sum
}

func task1(input string) (int, int) {
	chosen, boards := parse(string(input))

	ch := make(chan int)
	var mu sync.Mutex

	min := len(chosen) - 1
	minBoard := 0
	for i, board := range boards {
		go checkBoard(&board, chosen, ch)

		mu.Lock()
		if ret := <-ch; ret < min {
			minBoard, min = i, ret
		}
		mu.Unlock()
	}

	return chosen[min], calculateScore(&boards[minBoard], chosen[:min+1])
}

func task2(input string) (int, int) {
	chosen, boards := parse(string(input))

	ch := make(chan int)
	var mu sync.Mutex

	max := 0
	minBoard := 0
	for i, board := range boards {
		go checkBoard(&board, chosen, ch)

		mu.Lock()
		if ret := <-ch; ret > max {
			minBoard, max = i, ret
		}
		mu.Unlock()
	}

	return chosen[max], calculateScore(&boards[minBoard], chosen[:max+1])
}

func main() {
	content, err := os.ReadFile("input.txt")
	if err != nil {
		panic("Could not open input.txt")
	}
	input := string(content)

	t1v, t1s := task1(input)
	t2v, t2s := task2(input)

	fmt.Println("Task 1 result is:", t1v*t1s)
	fmt.Println("Task 2 result is:", t2v*t2s)
}
