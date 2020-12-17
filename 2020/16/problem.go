package main

import (
	"bufio"
	"bytes"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Range struct {
	low  int
	high int
}

func (r Range) Contains(n int) bool {
	return n >= r.low && n <= r.high
}

func (r Range) String() string {
	return fmt.Sprintf("[%d, %d]", r.low, r.high)
}

func ParseRange(line string) (Range, error) {
	parts := strings.Split(line, "-")
	if len(parts) != 2 {
		return Range{}, fmt.Errorf("invalid range: %s", line)
	}

	low, err := strconv.Atoi(parts[0])
	if err != nil {
		return Range{}, err
	}

	high, err := strconv.Atoi(parts[1])
	if err != nil {
		return Range{}, err
	}

	return Range{low, high}, nil
}

type Field struct {
	Name   string
	Range1 Range
	Range2 Range
}

func (f *Field) Contains(n int) bool {
	return f.Range1.Contains(n) || f.Range2.Contains(n)
}

func (f *Field) String() string {
	return fmt.Sprintf("%s: %s or %s", f.Name, f.Range1, f.Range2)
}

func ParseField(line string) (*Field, error) {
	parts := strings.Split(line, ": ")
	if len(parts) != 2 {
		return nil, fmt.Errorf("invalid field: %s", line)
	}

	name := parts[0]

	ranges := strings.Split(parts[1], " or ")
	if len(ranges) != 2 {
		return nil, fmt.Errorf("invalid ranges: %s", line)
	}

	range1, err := ParseRange(ranges[0])
	if err != nil {
		return nil, err
	}

	range2, err := ParseRange(ranges[1])
	if err != nil {
		return nil, err
	}

	return &Field{name, range1, range2}, nil
}

type Ticket struct {
	Values []int
	Valid  bool
}

func (t *Ticket) String() string {
	parts := make([]string, len(t.Values))
	for i, n := range t.Values {
		parts[i] = fmt.Sprintf("%d", n)
	}
	values := strings.Join(parts, ",")
	valid := ""
	if !t.Valid {
		valid = "*"
	}
	return fmt.Sprintf("%s%s", valid, values)
}

func ParseTicket(line string) (*Ticket, error) {
	parts := strings.Split(line, ",")
	result := make([]int, len(parts))
	for i, part := range parts {
		n, err := strconv.Atoi(part)
		if err != nil {
			return nil, err
		}
		result[i] = n
	}
	return &Ticket{Values: result}, nil
}

type Input struct {
	Fields []*Field
	Yours  *Ticket
	Nearby []*Ticket
}

func (input *Input) Part1() int {
	ans := 0
	for _, t := range input.Nearby {
		t.Valid = true
		for _, n := range t.Values {
			found := false
			for _, f := range input.Fields {
				if f.Contains(n) {
					found = true
				}
			}
			if !found {
				ans = ans + n
				t.Valid = false
			}
		}
	}
	return ans
}

func ReadInput(path string) (*Input, error) {
	f, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)

	input := &Input{
		Fields: nil,
		Yours:  nil,
		Nearby: nil,
	}

	for scanner.Scan() {
		if scanner.Text() == "" {
			break
		}
		field, err := ParseField(scanner.Text())
		if err != nil {
			return nil, err
		}
		input.Fields = append(input.Fields, field)
	}

	if scanner.Scan() {
		if scanner.Text() != "your ticket:" {
			return nil, fmt.Errorf("expected your ticket. got %q", scanner.Text())
		}
	}

	if scanner.Scan() {
		input.Yours, err = ParseTicket(scanner.Text())
		if err != nil {
			return nil, err
		}
	}

	if scanner.Scan() {
		if scanner.Text() != "" {
			return nil, fmt.Errorf("expected blank line. got %q", scanner.Text())
		}
	}

	if scanner.Scan() {
		if scanner.Text() != "nearby tickets:" {
			return nil, fmt.Errorf("expected nearby tickets. got %q", scanner.Text())
		}
	}

	for scanner.Scan() {
		t, err := ParseTicket(scanner.Text())
		if err != nil {
			return nil, err
		}
		input.Nearby = append(input.Nearby, t)
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return input, nil
}

func (i *Input) String() string {
	buf := &bytes.Buffer{}
	for _, f := range i.Fields {
		buf.WriteString(fmt.Sprintf("%s\n", f))
	}
	buf.WriteString(fmt.Sprintf("\nYours:\n%s\n\nNearby:\n", i.Yours))
	for _, t := range i.Nearby {
		buf.WriteString(fmt.Sprintf("%s\n", t))
	}
	return buf.String()
}

type Position struct {
	Possibilities map[string]bool
	Solved        bool
}

func CreatePositions(fs []*Field) []*Position {
	ps := []*Position{}
	for range fs {
		p := &Position{Possibilities: map[string]bool{}}
		for _, f := range fs {
			p.Possibilities[f.Name] = true
		}
		ps = append(ps, p)
	}
	return ps
}

func FilterImpossible(t *Ticket, fs []*Field, pos []*Position) {
	if !t.Valid {
		return
	}
	for i, n := range t.Values {
		for _, f := range fs {
			if !f.Contains(n) {
				delete(pos[i].Possibilities, f.Name)
			}
		}
	}
}

func FilterSingletons(pos []*Position) bool {
	filtered := false

	for i, p1 := range pos {
		if p1.Solved {
			continue
		}
		if len(p1.Possibilities) != 1 {
			continue
		}

		name := ""
		for s, _ := range p1.Possibilities {
			name = s
		}

		p1.Solved = true

		for j, p2 := range pos {
			if i == j {
				continue
			}

			if _, ok := p2.Possibilities[name]; ok {
				delete(p2.Possibilities, name)
				filtered = true
			}
		}
	}

	return filtered
}

func (input *Input) Part2() int {
	pos := CreatePositions(input.Fields)
	for _, t := range input.Nearby {
		FilterImpossible(t, input.Fields, pos)
	}

	fmt.Println()
	for FilterSingletons(pos) {
		for _, p := range pos {
			for name, _ := range p.Possibilities {
				fmt.Printf("%s, ", name)
			}
			fmt.Println()
		}
		fmt.Println()
	}

	// There's no guarantee that there's a unique solution at this point,
	// but it works for the given input.

	// Compute the answer
	ans := 1
	for i, p := range pos {
		name := ""
		for s, _ := range p.Possibilities {
			name = s
		}

		if strings.HasPrefix(name, "departure") {
			ans = ans * input.Yours.Values[i]
		}
	}
	return ans
}

func main() {
	input, err := ReadInput("input.txt")
	if err != nil {
		fmt.Printf("%v\n", err)
		return
	}

	fmt.Printf("%d\n", input.Part1())

	fmt.Printf("input: %s\n", input)

	fmt.Printf("%d\n", input.Part2())
}
