package main

import (
	"strings"
	"testing"
)

var input []string = strings.Split(`00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010`, "\n")

func TestTask1(t *testing.T) {
	got := task1(input)
	want := 198
	if got != want {
		t.Errorf("Error. Got: %d, received: %d.", got, want)
	}
}
func TestTask2(t *testing.T) {
	got := task2(input)
	want := 230
	if got != want {
		t.Errorf("Error. Got: %d, received: %d.", got, want)
	}
}
