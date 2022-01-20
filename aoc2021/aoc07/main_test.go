package main

import "testing"

var input string = "16,1,2,0,4,2,7,1,2,14"

func TestTask1(t *testing.T) {
	got := task1(input)
	want := 37
	if got != want {
		t.Errorf("Error: got %d, want %d\n", got, want)
	}
}

func TestTask2(t *testing.T) {
	got := task2(input)
	want := 168
	if got != want {
		t.Errorf("Error: got %d, want %d\n", got, want)
	}
}
