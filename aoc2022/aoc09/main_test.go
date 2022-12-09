package main

import (
	"testing"
)

var input1 string = `R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2`

var input2 string = `R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20`

func TestRope(t *testing.T) {
	got1, want1 := task1(input1), 13
	got2, want2 := task2(input1), 1
	if got1 != want1 {
		t.Errorf("got: %d, want: %d", got1, want1)
	}
	if got2 != want2 {
		t.Errorf("got: %d, want: %d", got2, want2)
	}

	got2, want2 = task2(input2), 36
	if got2 != want2 {
		t.Errorf("got: %d, want: %d", got2, want2)
	}
}
