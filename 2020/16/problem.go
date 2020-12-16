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

func (f Field) String() string {
	return fmt.Sprintf("%s: %s or %s", f.Name, f.Range1, f.Range2)
}

func ParseField(line string) (Field, error) {
	parts := strings.Split(line, ": ")
	if len(parts) != 2 {
		return Field{}, fmt.Errorf("invalid field: %s", line)
	}

	name := parts[0]

	ranges := strings.Split(parts[1], " or ")
	if len(ranges) != 2 {
		return Field{}, fmt.Errorf("invalid ranges: %s", line)
	}

	range1, err := ParseRange(ranges[0])
	if err != nil {
		return Field{}, err
	}

	range2, err := ParseRange(ranges[1])
	if err != nil {
		return Field{}, err
	}

	return Field{name, range1, range2}, nil
}

type Ticket []int

func (t Ticket) String() string {
	parts := make([]string, len(t))
	for i, n := range t {
		parts[i] = fmt.Sprintf("%d", n)
	}
	return strings.Join(parts, ",")
}

func ParseTicket(line string) (Ticket, error) {
	parts := strings.Split(line, ",")
	result := make([]int, len(parts))
	for i, part := range parts {
		n, err := strconv.Atoi(part)
		if err != nil {
			return nil, err
		}
		result[i] = n
	}
	return result, nil
}

type Input struct {
	Fields []Field
	Yours  Ticket
	Nearby []Ticket
}

func ReadInput(path string) (Input, error) {
	f, err := os.Open(path)
	if err != nil {
		return Input{}, err
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)

	input := Input{
		Fields: []Field{},
		Yours:  nil,
		Nearby: []Ticket{},
	}

	for scanner.Scan() {
		if scanner.Text() == "" {
			break
		}
		field, err := ParseField(scanner.Text())
		if err != nil {
			return Input{}, err
		}
		input.Fields = append(input.Fields, field)
	}

	if scanner.Scan() {
		if scanner.Text() != "your ticket:" {
			return Input{}, fmt.Errorf("expected your ticket. got %q", scanner.Text())
		}
	}

	if scanner.Scan() {
		input.Yours, err = ParseTicket(scanner.Text())
		if err != nil {
			return Input{}, err
		}
	}

	if scanner.Scan() {
		if scanner.Text() != "" {
			return Input{}, fmt.Errorf("expected blank line. got %q", scanner.Text())
		}
	}

	if scanner.Scan() {
		if scanner.Text() != "nearby tickets:" {
			return Input{}, fmt.Errorf("expected nearby tickets. got %q", scanner.Text())
		}
	}

	for scanner.Scan() {
		t, err := ParseTicket(scanner.Text())
		if err != nil {
			return Input{}, err
		}
		input.Nearby = append(input.Nearby, t)
	}

	if err := scanner.Err(); err != nil {
		return Input{}, err
	}

	return input, nil
}

func (i Input) String() string {
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

func main() {
	input, err := ReadInput("input.txt")
	if err != nil {
		fmt.Printf("%v\n", err)
		return
	}
	fmt.Printf("%s\n", input)
}
