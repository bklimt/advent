package main

import (
	"fmt"
	"os"
)

type Message struct {
	From int
	Addr int
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

	nat := Message{}
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
			ss[msg.From].ConsecutiveInputs = 0
			if msg.Addr == 255 {
				nat.X = msg.X
				nat.Y = msg.Y
			} else if msg.Addr >= 50 {
				fmt.Printf("Invalid address: %d for message{X=%d, Y=%d}\n", msg.Addr, msg.X, msg.Y)
				os.Exit(0)
			} else {
				ss[msg.Addr].Queue = append(ss[msg.Addr].Queue, msg)
			}

		case want, ok := <-anyWantInput:
			if !ok {
				fmt.Printf("anyWantInput is closed\n")
				continue
			}
			// fmt.Printf("%d wants input\n", want)
			if len(ss[want].Queue) == 0 {
				ss[want].In <- -1
				ss[want].ConsecutiveInputs++
			} else {
				msg := ss[want].Queue[0]
				ss[want].Queue = ss[want].Queue[1:]
				ss[want].In <- msg.X
				ss[want].In <- msg.Y
				ss[want].ConsecutiveInputs = 0
			}
		}

		// Check for idle state.
		idle := true
		for i := 0; i < 50; i++ {
			if ss[i].ConsecutiveInputs < 50 {
				idle = false
			}
		}
		if idle {
			fmt.Println("idle")
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
