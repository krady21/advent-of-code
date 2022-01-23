package main

import "testing"

var input string = `2199943210
3987894921
9856789892
8767896789
9899965678
`

func TestTask1(t *testing.T) {
	got := task1(input)
	want := 15
	if got != want {
		t.Errorf("got: %d, want: %d", got, want)
	}
}

func TestTask2(t *testing.T) {
	got := task2(input)
	want := 1134
	if got != want {
		t.Errorf("got: %d, want: %d", got, want)
	}
}
