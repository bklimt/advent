package main

import (
	"fmt"
	"os"
)

type Message struct {
	From int
	To   int
	X    int
	Y    int
}

type ComputerState struct {
	In                chan int
	Queue             []Message
	ConsecutiveInputs int
}

func main() {
	p := ReadProgram()

	cs := make([]*Computer, 50, 50)
	ss := make([]*ComputerState, 50, 50)

	anyOut := make(chan Message)
	anyWantInput := make(chan int)

	nat := Message{From: 255, To: 0}
	prevY := 0

	// Make 50 computers.
	for i := 0; i < 50; i++ {
		ss[i] = &ComputerState{
			In:                make(chan int, 1000),
			Queue:             []Message{},
			ConsecutiveInputs: 0,
		}

		out := make(chan int, 1000)
		wantInput := make(chan bool)
		c := NewComputer(p, ss[i].In, out, wantInput)
		go func() {
			c.Run()
		}()

		// Tell it its IP address.
		<-wantInput
		ss[i].In <- i

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
				anyOut <- Message{j, addr, x, y}
			}
		}()
	}

	for {
		select {
		case msg, ok := <-anyOut:
			if !ok {
				fmt.Printf("anyOut is closed\n")
				continue
			}
			fmt.Printf("%3d > %3d [%d, %d]\n", msg.From, msg.To, msg.X, msg.Y)
			ss[msg.From].ConsecutiveInputs = 0
			if msg.To == 255 {
				nat.X = msg.X
				nat.Y = msg.Y
			} else if msg.To >= 50 {
				fmt.Printf("Invalid address: %d for message{X=%d, Y=%d}\n", msg.To, msg.X, msg.Y)
				os.Exit(0)
			} else {
				ss[msg.To].Queue = append(ss[msg.To].Queue, msg)
			}

		case want, ok := <-anyWantInput:
			if !ok {
				fmt.Printf("anyWantInput is closed\n")
				continue
			}
			// fmt.Printf("%3d <\n", want)
			// fmt.Printf("%d wants input\n", want)
			if len(ss[want].Queue) == 0 {
				ss[want].In <- -1
				ss[want].ConsecutiveInputs++
			} else {
				msg := ss[want].Queue[0]
				ss[want].Queue = ss[want].Queue[1:]
				// TODO: What's broken here is we don't eat up the extra "wantInput" sends.
				ss[want].In <- msg.X
				ss[want].In <- msg.Y
				ss[want].ConsecutiveInputs = 0
				fmt.Printf("%3d < %3d [%d, %d]\n", msg.To, msg.From, msg.X, msg.Y)
			}
		}

		// Check for idle state.
		idle := true
		for i := 0; i < 50; i++ {
			if ss[i].ConsecutiveInputs < 100 {
				idle = false
				break
			}
			if len(ss[i].Queue) != 0 {
				fmt.Println("WARNING: invalid input state.")
			}
		}
		if idle {
			fmt.Printf("Idle: sending [%d, %d]\n", nat.X, nat.Y)
			for i := 0; i < 50; i++ {
				ss[i].ConsecutiveInputs = 0
			}
			ss[0].Queue = []Message{nat}
			if nat.Y == prevY {
				fmt.Printf("Result: %d\n", nat.Y)
				os.Exit(0)
			}
			prevY = nat.Y
		}
	}
}

// 1: 20160
// 2: 13241 is too high.
//    13161 is too low.
//    13217 is not right.
