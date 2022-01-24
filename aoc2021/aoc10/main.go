package main

import (
	"fmt"
	"os"
	"sort"
	"strings"
)

var t1 = map[string]int{
	")": 3,
	"]": 57,
	"}": 1197,
	">": 25137,
}

var t2 = map[string]int{
	")": 1,
	"]": 2,
	"}": 3,
	">": 4,
}

var match = map[string]string{
	"(": ")",
	"[": "]",
	"{": "}",
	"<": ">",
}

func parse(input string) [][]string {
	var nav [][]string
	for _, line := range strings.Split(strings.TrimSpace(input), "\n") {
		var chars []string
		chars = append(chars, strings.Split(strings.TrimSpace(line), "")...)
		nav = append(nav, chars)
	}
	return nav
}

func matches(s1, s2 string) bool {
	if (s1 == "(" && s2 == ")") ||
		(s1 == "[" && s2 == "]") ||
		(s1 == "{" && s2 == "}") ||
		(s1 == "<" && s2 == ">") {
		return true
	}
	return false
}

// checks if a line is corrupted
// returns the score if corrupted or 0 if not
func checkCorrupted(line []string) int {
	stack := make([]string, len(line))
	index := -1
	for _, ch := range line {
		switch ch {
		case ")", "]", "}", ">":
			if matches(stack[index], ch) {
				stack[index] = ""
				index--
			} else {
				return t1[ch]
			}
		case "(", "[", "{", "<":
			index++
			stack[index] = ch
		}
	}
	return 0
}

// returns the completion string for incomplete lines
func completionStr(line []string) []string {
	stack := make([]string, len(line))
	index := -1
	for _, ch := range line {
		switch ch {
		case ")", "]", "}", ">":
			if matches(stack[index], ch) {
				index--
			}
		case "(", "[", "{", "<":
			index++
			stack[index] = ch
		}
	}

	var compl []string
	for i := index; i >= 0; i-- {
		compl = append(compl, match[stack[i]])
	}
	return compl
}

func task1(input string) int {
	nav := parse(input)

	var score int
	for _, line := range nav {
		score += checkCorrupted(line)
	}
	return score
}

func task2(input string) int {
	nav := parse(input)

	// remove corrupted lines
	var nav2 [][]string
	for _, line := range nav {
		if corr := checkCorrupted(line); corr == 0 {
			nav2 = append(nav2, line)
		}
	}

	// find completion string for incomplete lines
	var scores []int
	for _, line := range nav2 {
		var score int
		for _, ch := range completionStr(line) {
			score = score*5 + t2[ch]
		}
		scores = append(scores, score)
	}

	sort.Ints(scores)
	return scores[len(scores)/2]
}

func main() {
	contents, _ := os.ReadFile("input.txt")
	input := string(contents)

	fmt.Println("Task 1 result is:", task1(input))
	fmt.Println("Task 2 result is:", task2(input))
}
