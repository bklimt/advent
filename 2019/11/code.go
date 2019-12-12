package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type Direction int

const (
	Up Direction = iota
	Right
	Down
	Left
)

func (dir Direction) Clockwise() Direction {
	switch dir {
	case Left:
		return Up
	case Right:
		return Down
	case Up:
		return Right
	case Down:
		return Left
	default:
		panic("unknown direction")
	}
}

func (dir Direction) CounterClockwise() Direction {
	switch dir {
	case Left:
		return Down
	case Right:
		return Up
	case Up:
		return Left
	case Down:
		return Right
	default:
		panic("unknown direction")
	}
}

type Position struct {
	X int
	Y int
}

type Map map[Position]int

type Robot struct {
	Map Map
	Pos Position
	Dir Direction
	In  <-chan int
	Out chan<- int
}

func NewRobot(in <-chan int, out chan<- int) *Robot {
	return &Robot{
		Map: make(map[Position]int),
		Pos: Position{0, 0},
		Dir: Up,
		In:  in,
		Out: out,
	}
}

func (robot *Robot) Step() bool {
	robot.Out <- robot.Map[robot.Pos]
	color, ok := <-robot.In
	if !ok {
		return false
	}
	turn := <-robot.In
	robot.Map[robot.Pos] = color
	if turn == 0 {
		robot.Dir = robot.Dir.CounterClockwise()
	} else {
		robot.Dir = robot.Dir.Clockwise()
	}
	switch robot.Dir {
	case Left:
		robot.Pos.X--
	case Right:
		robot.Pos.X++
	case Up:
		robot.Pos.Y--
	case Down:
		robot.Pos.Y++
	}
	return true
}

func (robot *Robot) Run() {
	for robot.Step() {
	}
}

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
	panic("unknown data mode")
}

func (comp *Computer) fetchAddr() int {
	mode := comp.modes % 10
	comp.modes = comp.modes / 10
	d1 := comp.fetch()
	if mode == 0 {
		return d1
	}
	if mode == 2 {
		return d1 + comp.base
	}
	panic("unknown addr mode")
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
		a := comp.fetchData()
		comp.base = comp.base + a
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
	robot := NewRobot(out, in)
	go robot.Run()
	c.Run()

	for p, n := range robot.Map {
		fmt.Printf("map[%d,%d] = %d\n", p.X, p.Y, n)
	}
	fmt.Println(len(robot.Map))
}

// 1785
