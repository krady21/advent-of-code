package main

import "testing"

var input string = `5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526`

func TestTask1(t *testing.T) {
	got := task1(input, 10)
	want := 204
	if got != want {
		t.Errorf("got: %d, want: %d", got, want)
	}

	got = task1(input, 100)
	want = 1656
	if got != want {
		t.Errorf("got: %d, want: %d", got, want)
	}

}

func TestTask2(t *testing.T) {
	got := task2(input)
	want := 195
	if got != want {
		t.Errorf("got: %d, want: %d", got, want)
	}
}
