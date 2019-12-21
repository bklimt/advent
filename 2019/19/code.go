package main

import (
	"fmt"
)

type Position struct {
	X int
	Y int
}

func main() {
	startRow := 1100
	startCol := 625
	endRow := 1290
	endCol := 825

	space := make(map[Position]bool)

	p := ReadProgram()
	total := 0
	for row := startRow; row < endRow; row++ {
		rowTotal := 0
		fmt.Printf("%02d ", row)
		prefix := true
		leadingDots := 0
		for col := startCol; col < endCol; col++ {
			in := make(chan int)
			out := make(chan int)
			wantInput := make(chan bool)
			c := NewComputer(p, in, out, wantInput)
			go func() {
				c.Run()
			}()

			<-wantInput
			in <- col
			<-wantInput
			in <- row
			on := <-out
			if on == 1 {
				total++
				rowTotal++
				fmt.Printf("#")
				space[Position{X: col, Y: row}] = true
				prefix = false
			} else {
				fmt.Printf(".")
				space[Position{X: col, Y: row}] = false
				if prefix {
					leadingDots++
				}
			}

			close(in)
		}
		fmt.Printf(" total = %03d, leading=%03d\n", rowTotal, leadingDots)
	}
	fmt.Printf("total = %d\n\n", total)

	for row := startRow; row < endRow; row++ {
		for col := startCol; col < endCol; col++ {
			if !space[Position{X: col, Y: row}] {
				continue
			}
			w := 0
			h := 0
			// How far right can I go?
			for c := col; c < endCol; c++ {
				if !space[Position{X: c, Y: row}] {
					break
				}
				w++
			}
			// How far down can I go?
			for r := row; r < endRow; r++ {
				if !space[Position{X: col, Y: r}] {
					break
				}
				h++
			}
			sq := w
			if h < sq {
				sq = h
			}
			if sq >= 100 {
				fmt.Printf("[%d, %d] sq = %d\n", col, row, sq)
			}
		}
	}
}

// 1: 169
// 2: 7001134
