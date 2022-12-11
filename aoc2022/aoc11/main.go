package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Monke struct {
	Items []uint64

	// shows how your worry level changes as the monke inspects that item
	Operation func(level uint64) uint64
	// shows how the monkey uses your worry level to decide where to throw an item
	Test func(level uint64) int

	Divisible uint64
}

func getModulo(monkes []Monke) uint64 {
	var modulo uint64 = 1
	for _, m := range monkes {
		modulo = modulo * m.Divisible
	}
	return modulo
}

func parse(input string) []Monke {
	list := strings.Split(input, "\n\n")
	monkes := make([]Monke, 0, len(list))
	for _, paragraph := range list {
		monke := Monke{}
		lines := strings.Split(paragraph, "\n")

		// starting items
		strItems := strings.Split(strings.Split(lines[1], ": ")[1], ", ")
		for _, strItem := range strItems {
			item, _ := strconv.Atoi(strItem)
			monke.Items = append(monke.Items, uint64(item))
		}

		// operation
		op := strings.Split(lines[2], " ")
		v, convErr := strconv.Atoi(op[len(op)-1])
		value := uint64(v)
		opType := op[len(op)-2]

		monke.Operation = func(initial uint64) uint64 {
			if convErr != nil { // old * old case
				value = initial
			}

			switch opType {
			case "*":
				return initial * value
			case "+":
				return initial + value
			default:
				panic("invalid optype")
			}
		}

		// test
		testLine := strings.Split(lines[3], " ")
		d, _ := strconv.Atoi(testLine[len(testLine)-1])
		div := uint64(d)
		monke.Divisible = div

		trueLine := strings.Split(lines[4], " ")
		trueMonke, _ := strconv.Atoi(trueLine[len(trueLine)-1])

		falseLine := strings.Split(lines[5], " ")
		falseMonke, _ := strconv.Atoi(falseLine[len(falseLine)-1])

		monke.Test = func(level uint64) int {
			if level%div == 0 {
				return trueMonke
			}
			return falseMonke
		}

		monkes = append(monkes, monke)
	}
	return monkes
}

func simulate(monkes []Monke, rounds int, decreaseWorry func(uint64) uint64) []uint64 {
	inspections := make([]uint64, len(monkes))
	for i := 0; i < rounds; i++ {
		for j, monke := range monkes {
			inspections[j] += uint64(len(monke.Items))
			for _, item := range monke.Items {
				newItem := monke.Operation(item)
				newItem = decreaseWorry(newItem)
				newMonkeIdx := monke.Test(newItem)

				monkes[newMonkeIdx].Items = append(monkes[newMonkeIdx].Items, newItem)
			}
			monkes[j].Items = monkes[j].Items[:0] // remove all current items.
		}
	}
	return inspections
}

func monkeBusiness(inspections []uint64) uint64 {
	var max1, max2 uint64
	for _, value := range inspections {
		if value > max1 {
			max2 = max1
			max1 = value
		} else if value > max2 {
			max2 = value
		}
	}
	return max1 * max2
}

func task1(monkes []Monke) uint64 {
	inspections := simulate(monkes, 20, func(old uint64) uint64 {
		return old / 3
	})
	return monkeBusiness(inspections)
}

func task2(monkes []Monke) uint64 {
	modulo := getModulo(monkes)
	inspections := simulate(monkes, 10000, func(old uint64) uint64 {
		return old % modulo
	})
	return monkeBusiness(inspections)
}

func main() {
	f, _ := os.ReadFile("input.txt")
	file := string(f)
	file = strings.TrimSpace(file)

	monkes := parse(file)
	result1 := task1(monkes)
	// I forgot to reparse the file and i was using the modified slice :((((
	monkes = parse(file)
	result2 := task2(monkes)

	fmt.Println("Task 1 result is:", result1)
	fmt.Println("Task 2 result is:", result2)
}
