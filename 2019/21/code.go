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

		// NOT B J
		// NOT C T
		// OR T J
		// AND D J
		// s := "NOT B J\nNOT C T\nOR T J\nAND D J\nRUN\n"

		// OR A T
		// AND B T
		// AND C T
		// NOT T J
		// AND D J
		// s := "OR A T\nAND B T\nAND C T\nNOT T J\nAND D J\nRUN\n"

		// OR D J
		// AND H J
		// s := "OR D J\nAND H J\nRUN\n"

		// OR A T
		// AND B T
		// AND C T
		// NOT T J
		// AND H J
		// AND D J
		// s := "OR A T\nAND B T\nAND C T\nNOT T J\nAND H J\nAND D J\nRUN\n"

		//   OR A T
		//   AND B T
		//   AND C T
		//   NOT T J
		//   NOT A T
		//   AND A T
		//   OR E T
		//   OR H T
		//   AND T J
		//   AND D J
		s := `OR A T
AND B T
AND C T
NOT T J
NOT A T
AND A T
OR E T
OR H T
AND T J
AND D J
RUN
`

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
//
// WWJ   J   J   ???
// #####.##.##.#.###
//
//  ABCDEFGHI
// #####.##.# NO
// ####.##.## NO
// ###.##.### YES
// ##.##.#.## YES
// #.#.###??? YES
//
// !B | !C
// NOT B J
// NOT C T
// OR T J
//
// #####...#########
//
//  ABCDEFGHI
// #####...## ?
// ####...### NO
// ###...#### NO
// ##...##### NO
// #...#####? YES
//
// NOT B J
// NOT C T
// OR T J
// AND D J
//
// (!B | !C) ^ D
//
// #####.#.#########
//
//  ABCDEFGHI
// #####.#.##
// ####.#.###
// ###.#.####
// ##.#.#####
// #.#.######
// #.########
//
// (!A | !B | !C) ^ D
// !(A^B^C) ^ D
//
// OR A T
// AND B T
// AND C T
// NOT T J
// AND D J
//
// #####.#.#...#####
//
//  ABCDEFGHI
// #####.#.#.
// ####.#.#..
// ###.#.#...
// ##.#.#...#
// #.#.#...##
// #.#...####
//
// D & H ? Is it rally that simple? No.
//
// X J   J   ??
// #####.#.#.##.####
//
//  ABCDEFGHI
// #####.#.#.
// ####.#.#.#
// ###.#.#.##
// ##.#.#.##.
// #.#.#.##.#
// #.#.##.###
// #.##.####?
//
// D & H & !(A ^ B ^ C)
//
// OR A T
// AND B T
// AND C T
// NOT T J
// AND H J
// AND D J
//
// #####...###..####
//
// (E | H) & D & !(A ^ B ^ C)
//
// J = !(A^B^C)
//   OR A T
//   AND B T
//   AND C T
//   NOT T J
// T = 0
//   NOT A T
//   AND A T
// T |= E
//   OR E T
// T |= H
//   OR H T
// J &= T
//   AND T J
// J &= D
//   AND D J
//
// 1139793906
//
