package main

import (
	"fmt"
	"os"
	"strings"
)

func dedup(s string) string {
	dict := make(map[rune]bool, len(s))
	var sb strings.Builder
	for _, ch := range s {
		if _, ok := dict[ch]; !ok {
			sb.WriteRune(ch)
		}
		dict[ch] = true
	}
	return sb.String()
}

func task1(input string) int {
	return firstDistinctSubstr(input, 4)
}

func task2(input string) int {
	return firstDistinctSubstr(input, 14)
}

func firstDistinctSubstr(input string, charCount int) int {
	for i := 0; i < len(input)-charCount; i++ {
		newS := dedup(input[i : i+charCount])
		if len(newS) == charCount {
			return i + charCount
		}
	}
	panic("we should not get here")

}

func main() {
	f, _ := os.ReadFile("input.txt")
	file := string(f)
	file = strings.TrimSpace(file)

	fmt.Println("Task 1 result is: ", task1(file))
	fmt.Println("Task 2 result is: ", task2(file))
}
