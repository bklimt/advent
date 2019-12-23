package main

import (
	"fmt"
	"os"
)

type Message struct {
	Addr int
	X    int
	Y    int
}

func main() {
	p := ReadProgram()

	cs := make([]*Computer, 50, 50)
	anyOut := make(chan Message)
	anyWantInput := make(chan int)
	allIn := make([]chan int, 50, 50)

	qs := make([][]Message, 50, 50)

	// Make 50 computers.
	for i := 0; i < 50; i++ {
		qs[i] = []Message{}

		allIn[i] = make(chan int, 1000)
		out := make(chan int, 1000)
		wantInput := make(chan bool)
		c := NewComputer(p, allIn[i], out, wantInput)
		go func() {
			c.Run()
		}()

		// Tell it its IP address.
		<-wantInput
		allIn[i] <- i

		// Put it in the array.
		cs[i] = c

		// Connect its input.
		j := i
		go func() {
			for {
				_, ok := <-wantInput
				if !ok {
					break
				}
				anyWantInput <- j
			}
		}()

		// Connect its output.
		go func() {
			for {
				addr, ok := <-out
				if !ok {
					break
				}
				x := <-out
				y := <-out
				if addr >= 50 {
					fmt.Printf("Invalid address: %d for message{X=%d, Y=%d}\n", addr, x, y)
					os.Exit(0)
				}
				anyOut <- Message{addr, x, y}
			}
		}()
	}

	for {
		select {
		case msg := <-anyOut:
			qs[msg.Addr] = append(qs[msg.Addr], msg)
		case want, ok := <-anyWantInput:
			if !ok {
				fmt.Printf("anyWantInput is closed\n")
				continue
			}
			fmt.Printf("%d wants input\n", want)
			if len(qs[want]) == 0 {
				allIn[want] <- -1
			} else {
				msg := qs[want][0]
				qs[want] = qs[want][1:]
				allIn[want] <- msg.X
				allIn[want] <- msg.Y
			}
		}
	}
}

// 1: 20160
