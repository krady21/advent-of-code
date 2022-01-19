package main

import (
	"testing"
)

var input string = `0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2`

func TestTask1(t *testing.T) {
	lines := parse(input)
	got := task1(lines)
	want := 5
	if got != want {
		t.Errorf("Error. Got: %d, received: %d.", got, want)
	}
}

func TestTask2(t *testing.T) {
	lines := parse(input)
	got := task2(lines)
	want := 12
	if got != want {
		t.Errorf("Error. Got: %d, received: %d.", got, want)
	}
}
