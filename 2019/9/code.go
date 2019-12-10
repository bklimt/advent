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
	memory  map[int]int
	ip      int
	modes   int
	base    int
}

func NewComputer(memory []int, in <-chan int, out chan<- int) *Computer {
	mem := make(map[int]int)
	for i, x := range memory {
		mem[i] = x
	}
	return &Computer{
		memory:  mem,
		in:      in,
		out:     out,
		running: true,
		ip:      0,
		modes:   0,
		base:    0,
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
	if mode == 1 {
		return d1
	}
	if mode == 2 {
		return comp.memory[d1+comp.base]
	}
	panic("unknown mode")
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
	case 9:
		comp.base = comp.base + comp.fetchData()
	default:
		fmt.Printf("unknown opcode: %d\n", opcode)
		comp.quit()
	}
}

func (comp *Computer) Run() {
	comp.running = true
	comp.ip = 0
	for comp.running {
		comp.process()
	}
	close(comp.out)
}

func test1() {
	// p := []int{109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99}
	// p := []int{1102, 34915192, 34915192, 7, 4, 7, 99, 0}
	p := []int{104, 1125899906842624, 99}
	in := make(chan int)
	out := make(chan int)
	c := NewComputer(p, in, out)
	go c.Run()
	output := []string{}
	for x := range out {
		output = append(output, fmt.Sprintf("%d", x))
	}
	fmt.Printf("[%s]\n", strings.Join(output, ","))
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
	in := make(chan int)
	out := make(chan int)
	c := NewComputer(p, in, out)
	go c.Run()
	in <- 1
	output := []string{}
	for x := range out {
		output = append(output, fmt.Sprintf("%d", x))
	}
	fmt.Printf("[%s]\n", strings.Join(output, ","))
}
