package main

import (
	"fmt"
)

type Position struct {
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

	screen := make(map[Position]int)
	cursor := Position{}
	size := Position{}

	for {
		c, ok := <-out
		if !ok {
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
	close(in)

	fmt.Println()
	fmt.Println()

	xs := make([]Position, 0)
	sum := 0
	for y := 0; y <= size.Y; y++ {
		for x := 0; x <= size.X; x++ {
			c, ok := screen[Position{X: x, Y: y}]
			if !ok {
				fmt.Print(" ")
			} else {
				fmt.Printf("%c", c)
			}

			if c == 35 {
				if screen[Position{X: x - 1, Y: y}] == 35 &&
					screen[Position{X: x + 1, Y: y}] == 35 &&
					screen[Position{X: x, Y: y - 1}] == 35 &&
					screen[Position{X: x, Y: y + 1}] == 35 {
					xs = append(xs, Position{X: x, Y: y})
					sum = sum + x*y
				}
			}
		}
	}
	fmt.Println()

	for _, x := range xs {
		fmt.Printf("%v\n", x)
	}
	fmt.Printf("\n%d\n", sum)
}

// 1: 3292
