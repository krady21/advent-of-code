package main

import "testing"

var input string = `[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]`

func TestTask1(t *testing.T) {
	got := task1(input)
	want := 26397
	if got != want {
		t.Errorf("got: %d, want: %d", got, want)
	}
}

func TestTask2(t *testing.T) {
	got := task2(input)
	want := 288957
	if got != want {
		t.Errorf("got: %d, want: %d", got, want)
	}
}
