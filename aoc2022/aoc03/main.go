package main

import (
	"fmt"
	"os"
	"strings"
)

func uniq(first string, others ...string) rune {
	for _, ch := range first {
		var foundInAll bool
		for _, toLookIn := range others {
			if strings.ContainsRune(toLookIn, ch) {
				foundInAll = true
			} else {
				foundInAll = false
				break
			}
		}
		if foundInAll {
			return ch
		}
	}
	panic("should not get here")
}

func main() {
	f, _ := os.ReadFile("input.txt")
	file := string(f)

	lines := strings.Split(strings.TrimSpace(file), "\n")
	prio := make(map[rune]int, 52)
	for c, i := 'a', 1; c <= 'z'; c++ {
		prio[c] = i
		i++
	}

	for c, i := 'A', 27; c <= 'Z'; c++ {
		prio[c] = i
		i++
	}

	var sum1, sum2 int
	for i := 0; i < len(lines); i++ {
		sum1 += prio[uniq(lines[i][:len(lines[i])/2], lines[i][len(lines[i])/2:])]
		if i%3 == 0 {
			sum2 += prio[uniq(lines[i], lines[i+1], lines[i+2])]
		}
	}

	fmt.Println("Task 1 result is: ", sum1)
	fmt.Println("Task 2 result is: ", sum2)
}
