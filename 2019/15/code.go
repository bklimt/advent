package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type SpaceTile int

const (
	Unknown SpaceTile = iota
	Empty
	Wall
	Goal
)

type Direction int

const (
	North Direction = iota + 1
	South
	West
	East
)

type Response int

const (
	HitWall Response = iota
	Moved
	FoundGoal
)

type Position struct {
	X int
	Y int
}

func (pos Position) Add(dir Direction) Position {
	dx, dy := dir.Delta()
	return Position{X: pos.X + dx, Y: pos.Y + dy}
}

type Space struct {
	Map      map[Position]SpaceTile
	GoalDist map[Position]int
	Min      Position
	Max      Position
	Robot    Position
	Goal     Position
	In       <-chan int
	Out      chan<- int
	OutReady <-chan bool
}

func NewSpace(in <-chan int, out chan<- int, outReady <-chan bool) *Space {
	return &Space{
		Map:      make(map[Position]SpaceTile),
		GoalDist: make(map[Position]int),
		Min:      Position{X: 0, Y: 0},
		Max:      Position{X: 0, Y: 0},
		Robot:    Position{X: 0, Y: 0},
		Goal:     Position{X: 0, Y: 0},
		In:       in,
		Out:      out,
		OutReady: outReady,
	}
}

func (space *Space) Draw() {
	fmt.Println(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>")
	for row := space.Min.Y; row <= space.Max.Y; row++ {
		for col := space.Min.X; col <= space.Max.X; col++ {
			if space.Robot.X == col && space.Robot.Y == row {
				fmt.Print("D")
				continue
			}

			tile := space.Map[Position{X: col, Y: row}]
			switch tile {
			case Unknown:
				fmt.Print(" ")
			case Empty:
				fmt.Print(".")
			case Wall:
				fmt.Print("#")
			case Goal:
				fmt.Print("@")
			}
		}
		fmt.Println()
	}
	fmt.Println("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<")
}

func (space *Space) LetterForDistance(pos Position) int {
	dist, ok := space.GoalDist[pos]
	if !ok {
		return '?'
	}
	return dist/20 + 'A'

}

func (space *Space) CycleDist() bool {
	updated := false
	fmt.Println(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>")
	for row := space.Min.Y; row <= space.Max.Y; row++ {
		for col := space.Min.X; col <= space.Max.X; col++ {
			pos := Position{X: col, Y: row}
			tile := space.Map[pos]
			switch tile {
			case Unknown:
				fmt.Print(" ")
			case Empty:
				if space.UpdateGoalDists(pos) {
					updated = true
				}
				fmt.Printf("%c", space.LetterForDistance(pos))
			case Wall:
				fmt.Print("#")
			case Goal:
				fmt.Print("@")
			}
		}
		fmt.Println()
	}
	fmt.Println("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<")
	return updated
}

func (dir Direction) Opposite() Direction {
	switch dir {
	case North:
		return South
	case South:
		return North
	case West:
		return East
	case East:
		return West
	}
	panic("unknown direction")
}

func (dir Direction) Delta() (int, int) {
	switch dir {
	case North:
		return 0, -1
	case South:
		return 0, 1
	case West:
		return -1, 0
	case East:
		return 1, 0
	}
	panic("unknown direction")
}

func (space *Space) UpdateGoalDists(pos Position) bool {
	if _, ok := space.GoalDist[pos]; ok {
		return false
	}

	found := false
	min := 0

	positions := []Position{pos.Add(North), pos.Add(South), pos.Add(East), pos.Add(West)}
	for _, p := range positions {
		d, ok := space.GoalDist[p]
		if ok && (!found || d+1 < min) {
			min = d + 1
			found = true
		}
	}

	if !found {
		return false
	}

	space.GoalDist[pos] = min
	return true
}

func (space *Space) TryDirection(dir Direction) {
	newPos := space.Robot.Add(dir)

	if newPos.X < space.Min.X {
		space.Min.X = newPos.X
	}
	if newPos.X > space.Max.X {
		space.Max.X = newPos.X
	}
	if newPos.Y < space.Min.Y {
		space.Min.Y = newPos.Y
	}
	if newPos.Y > space.Max.Y {
		space.Max.Y = newPos.Y
	}

	if space.Map[newPos] != Unknown {
		return
	}

	<-space.OutReady
	space.Out <- int(dir)
	response := Response(<-space.In)
	if response == HitWall {
		space.Map[newPos] = Wall
		space.Draw()
	} else {
		// Move to the new position and try from there.
		oldPos := space.Robot
		space.Robot = newPos
		if response == FoundGoal {
			space.Map[newPos] = Goal
			space.Goal = newPos
			space.GoalDist[newPos] = 0
		} else {
			space.Map[newPos] = Empty
		}
		space.Draw()
		space.Explore()
		// Move back.
		<-space.OutReady
		space.Out <- int(dir.Opposite())
		<-space.In
		space.Robot = oldPos
	}
}

func (space *Space) Explore() {
	space.TryDirection(South)
	space.TryDirection(East)
	space.TryDirection(North)
	space.TryDirection(West)
}

func (space *Space) Run() {
	space.Map[space.Robot] = Empty
	space.Explore()
}

func (space *Space) GetMaxGoalDistance() int {
	max := 0
	for row := space.Min.Y; row <= space.Max.Y; row++ {
		for col := space.Min.X; col <= space.Max.X; col++ {
			pos := Position{X: col, Y: row}
			if space.Map[pos] == Empty {
				dist, ok := space.GoalDist[pos]
				if !ok {
					panic("unmeasured distance")
				}
				if dist > max {
					max = dist
				}
			}
		}
	}
	return max
}

func (space *Space) GetGoalDistance() int {
	return space.GoalDist[Position{X: 0, Y: 0}]
}

type Computer struct {
	In        <-chan int
	Out       chan<- int
	WantInput chan<- bool
	Running   bool
	Memory    map[int]int
	Ip        int
	Modes     int
	Base      int
}

func NewComputer(memory []int, in <-chan int, out chan<- int, wantInput chan<- bool) *Computer {
	mem := make(map[int]int)
	for i, x := range memory {
		mem[i] = x
	}
	return &Computer{
		Memory:    mem,
		In:        in,
		Out:       out,
		WantInput: wantInput,
		Running:   true,
		Ip:        0,
		Modes:     0,
		Base:      0,
	}
}

func (comp *Computer) quit() {
	comp.Running = false
}

func (comp *Computer) add(d1, d2, a3 int) {
	d3 := d1 + d2
	comp.Memory[a3] = d3
}

func (comp *Computer) multiply(d1, d2, a3 int) {
	d3 := d1 * d2
	comp.Memory[a3] = d3
}

func (comp *Computer) input(a1 int) {
	comp.WantInput <- true
	d1 := <-comp.In
	comp.Memory[a1] = d1
}

func (comp *Computer) output(d1 int) {
	comp.Out <- d1
}

func (comp *Computer) jumpIf(d1, d2 int) {
	if d1 != 0 {
		comp.Ip = d2
	}
}

func (comp *Computer) jumpIfNot(d1, d2 int) {
	if d1 == 0 {
		comp.Ip = d2
	}
}

func (comp *Computer) lessThan(d1, d2, a3 int) {
	if d1 < d2 {
		comp.Memory[a3] = 1
	} else {
		comp.Memory[a3] = 0
	}
}

func (comp *Computer) equals(d1, d2, a3 int) {
	if d1 == d2 {
		comp.Memory[a3] = 1
	} else {
		comp.Memory[a3] = 0
	}
}

func (comp *Computer) fetch() int {
	d1 := comp.Memory[comp.Ip]
	comp.Ip++
	return d1
}

func (comp *Computer) fetchOp() int {
	op := comp.fetch()
	opcode := op % 100
	op = op / 100
	comp.Modes = op
	return opcode
}

func (comp *Computer) fetchData() int {
	mode := comp.Modes % 10
	comp.Modes = comp.Modes / 10
	d1 := comp.fetch()
	if mode == 0 {
		return comp.Memory[d1]
	}
	if mode == 1 {
		return d1
	}
	if mode == 2 {
		return comp.Memory[d1+comp.Base]
	}
	panic("unknown data mode")
}

func (comp *Computer) fetchAddr() int {
	mode := comp.Modes % 10
	comp.Modes = comp.Modes / 10
	d1 := comp.fetch()
	if mode == 0 {
		return d1
	}
	if mode == 2 {
		return d1 + comp.Base
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
		comp.Base = comp.Base + a
	default:
		fmt.Printf("unknown opcode: %d\n", opcode)
		comp.quit()
	}
}

func (comp *Computer) Run() {
	comp.Running = true
	comp.Ip = 0
	for comp.Running {
		comp.process()
	}
	close(comp.Out)
	close(comp.WantInput)
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
	wantInput := make(chan bool)

	c := NewComputer(p, in, out, wantInput)
	space := NewSpace(out, in, wantInput)
	go func() {
		c.Run()
	}()
	space.Run()
	close(in)

	for space.CycleDist() {
	}

	dist := space.GetGoalDistance()
	max := space.GetMaxGoalDistance()
	fmt.Printf("goal distance: %d\nmax distance: %d\n", dist, max)
}

// 1: 296
// 2: 302
