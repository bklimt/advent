package main

import (
	"fmt"
)

type position struct {
	X int
	Y int
}

func main() {
	p := ReadProgram()
	in := make(chan int)
	out := make(chan int)
	wantInput := make(chan bool)
	c := NewComputer(p, in, out, wantInput)
	go func() {
		c.Run()
	}()

	// Supply the input programming.
	go func() {
		// NOT C J
		// AND D J
		// NOT A T
		// OR T J
		// s := "NOT C J\nAND D J\nNOT A T\nOR T J\nRUN\n"

		// OR D T
		// AND C T
		// NOT T T
		// s := "OR D T\nAND C T\nNOT T T\nRUN\n"

		// NOT D J
		// NOT C T
		// OR T J
		s := "NOT D J\nNOT C T\nOR T J\nRUN\n"

		for _, c := range s {
			<-wantInput
			in <- int(c)
		}
		close(in)
	}()

	screen := make(map[position]int)
	cursor := position{}
	size := position{}

	for {
		c, ok := <-out
		if !ok {
			break
		}

		if c > 127 {
			fmt.Printf("\nRESULT: %d\n", c)
			break
		}

		fmt.Printf("%c", c)
		screen[cursor] = c
		if c == 10 {
			cursor.X = 0
			cursor.Y++
		} else {
			cursor.X++
		}
		if cursor.X > size.X {
			size.X = cursor.X
		}
		if cursor.Y > size.Y {
			size.Y = cursor.Y
		}
	}

	fmt.Println()
	fmt.Println()
}

// 1:
// NOT C J
// AND D J
// NOT A T
// OR T J
// WALK
// 19357180

// 2:
// OR D T
// AND C T
// NOT T T
//
//   1XXX2XXX3XXX4
// #####.##.##.#.###
//
// ABCDEFGHI
// #####.##. NO
// ####.##.# NO
// ###.##.## YES
// ##.##.#.# YES
// #.#.###?? YES
//
// E^F | !C
// !D | !C
// !(D&C)
