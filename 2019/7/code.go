package main

import (
	"fmt"
  "io/ioutil"
  "strconv"
	"strings"
)

type Computer struct {
	in      <-chan int
	out     chan<- int
	running bool
	memory  []int
	ip      int
	modes   int
}

func NewComputer(memory []int, in <-chan int, out chan<- int) *Computer {
	mem := make([]int, len(memory))
	copy(mem, memory)
	return &Computer{
		memory:  mem,
		in:      in,
		out:     out,
		running: true,
		ip:      0,
		modes:   0,
	}
}

func (comp *Computer) quit() {
	comp.running = false
}

func (comp *Computer) add(d1, d2, a3 int) {
	d3 := d1 + d2
	comp.memory[a3] = d3
}

func (comp *Computer) multiply(d1, d2, a3 int) {
	d3 := d1 * d2
	comp.memory[a3] = d3
}

func (comp *Computer) input(a1 int) {
	d1 := <-comp.in
	comp.memory[a1] = d1
}

func (comp *Computer) output(d1 int) {
	comp.out <- d1
}

func (comp *Computer) jumpIf(d1, d2 int) {
	if d1 != 0 {
		comp.ip = d2
	}
}

func (comp *Computer) jumpIfNot(d1, d2 int) {
	if d1 == 0 {
		comp.ip = d2
	}
}

func (comp *Computer) lessThan(d1, d2, a3 int) {
	if d1 < d2 {
		comp.memory[a3] = 1
	} else {
		comp.memory[a3] = 0
	}
}

func (comp *Computer) equals(d1, d2, a3 int) {
	if d1 == d2 {
		comp.memory[a3] = 1
	} else {
		comp.memory[a3] = 0
	}
}

func (comp *Computer) fetch() int {
	d1 := comp.memory[comp.ip]
	comp.ip++
	return d1
}

func (comp *Computer) fetchOp() int {
	op := comp.fetch()
	opcode := op % 100
	op = op / 100
	comp.modes = op
	return opcode
}

func (comp *Computer) fetchData() int {
	mode := comp.modes % 10
	comp.modes = comp.modes / 10
	d1 := comp.fetch()
	if mode == 0 {
		return comp.memory[d1]
	}
	return d1
}

func (comp *Computer) fetchAddr() int {
	comp.modes = comp.modes / 10
	d1 := comp.fetch()
	return d1
}

func (comp *Computer) process() {
	switch opcode := comp.fetchOp(); opcode {
	case 99:
		comp.quit()
	case 1:
		a := comp.fetchData()
		b := comp.fetchData()
		c := comp.fetchAddr()
		comp.add(a, b, c)
	case 2:
		a := comp.fetchData()
		b := comp.fetchData()
		c := comp.fetchAddr()
		comp.multiply(a, b, c)
	case 3:
		a := comp.fetchAddr()
		comp.input(a)
	case 4:
		a := comp.fetchData()
		comp.output(a)
	case 5:
		a := comp.fetchData()
		b := comp.fetchData()
		comp.jumpIf(a, b)
	case 6:
		a := comp.fetchData()
		b := comp.fetchData()
		comp.jumpIfNot(a, b)
	case 7:
		a := comp.fetchData()
		b := comp.fetchData()
		c := comp.fetchAddr()
		comp.lessThan(a, b, c)
	case 8:
		a := comp.fetchData()
		b := comp.fetchData()
		c := comp.fetchAddr()
		comp.equals(a, b, c)
	default:
		fmt.Printf("unknown opcode: %d\n", opcode)
		comp.quit()
	}
}

func (comp *Computer) Run() int {
	comp.running = true
	comp.ip = 0
	for comp.running {
		comp.process()
	}
	return comp.memory[0]
}

func Thrust(p []int, s1, s2, s3, s4, s5 int) int {
	out1 := make(chan int, 2)
	out2 := make(chan int, 2)
	out3 := make(chan int, 2)
	out4 := make(chan int, 2)
	out5 := make(chan int, 2)

	t1 := NewComputer(p, out5, out1)
	t2 := NewComputer(p, out1, out2)
	t3 := NewComputer(p, out2, out3)
	t4 := NewComputer(p, out3, out4)
	t5 := NewComputer(p, out4, out5)

	out5 <- s1
	out1 <- s2
	out2 <- s3
	out3 <- s4
	out4 <- s5
  out5 <- 0

  done := make(chan bool)
	go func() {
    t1.Run()
    done <- true
  }()
  go func() {
    t2.Run()
    done <- true
  }()
	go func() {
    t3.Run()
    done <- true
  }()
	go func() {
    t4.Run()
    done <- true
  }()
	go func() {
    t5.Run()
    done <- true
  }()
  <-done
  <-done
  <-done
  <-done
  <-done

  return <-out5
}

func ReadProgram() []int {
	b, err := ioutil.ReadFile("program.txt")
	if err != nil {
		panic(err)
	}
	s := strings.TrimSpace(string(b))
	parts := strings.Split(s, ",")
	p := make([]int, len(parts))
	for i, part := range parts {
		p[i], err = strconv.Atoi(part)
		if err != nil {
			panic(err)
		}
	}
	return p
}

func main() {
	p := ReadProgram()

	maxX := 0
	maxS := ""

	for i := 5; i < 10; i++ {
		for j := 5; j < 10; j++ {
			if j == i {
				continue
			}
			for k := 5; k < 10; k++ {
				if k == i || k == j {
					continue
				}
				for m := 5; m < 10; m++ {
					if m == i || m == j || m == k {
						continue
					}
					for n := 5; n < 10; n++ {
						if n == i || n == j || n == k || n == m {
							continue
						}

						x := Thrust(p, i, j, k, m, n)
						s := fmt.Sprintf("%d%d%d%d%d", i, j, k, m, n)
						fmt.Printf("%d: %s\n", x, s)
						if x > maxX {
							maxX = x
							maxS = s
						}
					}
				}
			}
		}
	}
	fmt.Printf("%d: %s\n", maxX, maxS)
}

// 1: 11828: 40231
// 2: 1714298: 86957
