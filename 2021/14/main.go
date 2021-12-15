package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

type Input struct {
	Template string
	Rules    map[string]string
}

func ReadInput() (*Input, error) {
	b, err := ioutil.ReadFile("input.txt")
	if err != nil {
		return nil, err
	}

	lines := strings.Split(strings.TrimSpace(string(b)), "\n")
	if len(lines) < 3 {
		return nil, fmt.Errorf("invalid lines in input: %d", len(lines))
	}
	if lines[1] != "" {
		return nil, fmt.Errorf("invalid second line: %s", lines[1])
	}

	template := lines[0]
	rules := make(map[string]string)

	for _, line := range lines[2:] {
		parts := strings.Split(line, " -> ")
		if len(parts) != 2 {
			return nil, fmt.Errorf("invalid line: %s", line)
		}

		rules[parts[0]] = parts[1]
	}

	return &Input{template, rules}, nil
}

func (input *Input) Process(in string) (string, error) {
	out := ""
	for i := 0; i < len(in)-1; i++ {
		k := in[i : i+2]
		c1 := k[0]
		v, ok := input.Rules[k]
		if !ok {
			return "", fmt.Errorf("invalid lhs: %s", k)
		}
		out = fmt.Sprintf("%s%c%s", out, c1, v)
	}
	out = fmt.Sprintf("%s%c", out, in[len(in)-1])
	return out, nil
}

func CountLetters(s string) {
	m := map[rune]int{}
	for _, c := range s {
		v, ok := m[c]
		if !ok {
			v = 0
		}
		v++
		m[c] = v
	}

	min := len(s)
	max := 0
	for _, v := range m {
		if v < min {
			min = v
		}
		if v > max {
			max = v
		}
	}

	fmt.Printf("min = %d, max = %d\n", min, max)
	fmt.Printf("part1 = %d", max-min)
}

func main() {
	rules, err := ReadInput()
	if err != nil {
		fmt.Printf("unable to read input: %s\n", err)
		return
	}

	s := rules.Template
	for i := 0; i < 10; i++ {
		s, err = rules.Process(s)
		if err != nil {
			fmt.Printf("unable to process string: %s", err)
			return
		}
		//fmt.Println(s)
	}

	CountLetters(s)
}
