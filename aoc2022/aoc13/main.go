package main

import (
	"encoding/json"
	"fmt"
	"os"
	"sort"
	"strings"
)

func parse(input string) []any {
	lines := strings.Split(input, "\n")
	packets := []any{}
	for _, line := range lines {
		if strings.TrimSpace(line) == "" {
			continue
		}

		var packet any
		json.Unmarshal([]byte(line), &packet)
		packets = append(packets, packet)
	}
	return packets
}

func compare(left, right any) int {
	var l, r []any
	var lsingle, rsingle bool

	switch left.(type) {
	case float64:
		l, lsingle = []any{left}, true
	case []any, []float64:
		l = left.([]any)
	}

	switch right.(type) {
	case float64:
		r, rsingle = []any{right}, true
	case []any, []float64:
		r = right.([]any)
	}

	if rsingle && lsingle {
		return int(l[0].(float64) - r[0].(float64))
	}

	for i := 0; i < len(l) && i < len(r); i++ {
		c := compare(l[i], r[i])
		if c != 0 {
			return c
		}
	}
	return len(l) - len(r)
}

func task1(packets []any) int {
	var count int
	for i := 0; i < len(packets); i += 2 {
		if compare(packets[i], packets[i+1]) <= 0 {
			count += i/2 + 1
		}
	}
	return count
}

func task2(packets []any) int {
	sort.Slice(packets, func(i, j int) bool {
		return compare(packets[i], packets[j]) < 0
	})

	decoderKey := 1
	for i := 0; i < len(packets); i++ {
		if fmt.Sprint(packets[i]) == "[[2]]" || fmt.Sprint(packets[i]) == "[[6]]" {
			decoderKey *= i + 1
		}
	}

	return decoderKey
}

func main() {
	f, _ := os.ReadFile("input.txt")
	file := string(f)
	file = strings.TrimSpace(file)

	packets := parse(file)
	result1 := task1(packets)

	packets2 := parse(file + "\n\n[[2]]\n[[6]]")
	result2 := task2(packets2)

	fmt.Println("Task 1 result is:", result1)
	fmt.Println("Task 2 result is:", result2)
}
