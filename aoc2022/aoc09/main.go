package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

type Direction string

const (
	Up    Direction = "U"
	Down  Direction = "D"
	Right Direction = "R"
	Left  Direction = "L"
)

type Pos struct {
	x, y float64
}

type Rope struct {
	Knots         []Pos
	VisitedByTail map[Pos]bool
}

func NewRope(knots int) Rope {
	r := Rope{
		Knots:         make([]Pos, knots),
		VisitedByTail: make(map[Pos]bool, 10000),
	}
	r.VisitedByTail[r.Knots[knots-1]] = true
	return r
}

func (r *Rope) needsToMoveTail(knotIndex int) bool {
	p1 := &r.Knots[knotIndex-1]
	p2 := &r.Knots[knotIndex]
	return math.Abs(p1.x-p2.x) > 1 || math.Abs(p1.y-p2.y) > 1
}

func (r *Rope) moveHead(d Direction) {
	switch d {
	case Up:
		r.Knots[0].x += 1
	case Down:
		r.Knots[0].x -= 1
	case Right:
		r.Knots[0].y += 1
	case Left:
		r.Knots[0].y -= 1
	}
}

func (r *Rope) moveTail(i int) {
	current := &r.Knots[i]
	next := &r.Knots[i-1]
	if next.x > current.x {
		current.x += 1
	}
	if next.x < current.x {
		current.x -= 1
	}
	if next.y > current.y {
		current.y += 1
	}
	if next.y < current.y {
		current.y -= 1
	}
}

func (r *Rope) Move(d Direction) {
	r.moveHead(d)

	for i := 1; i < len(r.Knots); i++ {
		if !r.needsToMoveTail(i) {
			return
		}
		r.moveTail(i)
	}

	r.VisitedByTail[r.Knots[len(r.Knots)-1]] = true
}

func (r *Rope) MoveTimes(d Direction, steps int) {
	for i := 0; i < steps; i++ {
		r.Move(d)
	}
}

func simulate(input string, knots int) int {
	rope := NewRope(knots)
	for _, line := range strings.Split(input, "\n") {
		split := strings.Split(line, " ")
		direction := Direction(split[0])
		steps, _ := strconv.Atoi(split[1])
		rope.MoveTimes(direction, steps)
	}

	return len(rope.VisitedByTail)
}

func task1(input string) int {
	return simulate(input, 2)
}

func task2(input string) int {
	return simulate(input, 10)
}

func main() {
	f, _ := os.ReadFile("input.txt")
	file := string(f)
	file = strings.TrimSpace(file)

	result1 := task1(file)
	result2 := task2(file)

	fmt.Println("Task 1 result is:", result1)
	fmt.Println("Task 2 result is:", result2)
}
