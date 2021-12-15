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
	m := map[string]int{}
	for _, c := range s {
		v, ok := m[string(c)]
		if !ok {
			v = 0
		}
		v++
		m[string(c)] = v
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

	fmt.Printf("counts = %v\n", m)
	fmt.Printf("min = %d, max = %d\n", min, max)
	fmt.Printf("result = %d\n", max-min)
}

func CountBigraphs(s string) map[string]int {
	m := map[string]int{}
	for i := 0; i < len(s)-1; i++ {
		m[s[i:i+2]] = m[s[i:i+2]] + 1
	}
	return m
}

func (input *Input) Both(times int) error {
	start := input.Template
	s := start

	m := CountBigraphs(start)

	var err error
	fmt.Println(s)
	fmt.Printf("%v\n", m)
	for i := 0; i < times; i++ {
		m2 := map[string]int{}
		for k, v := range m {
			c, ok := input.Rules[k]
			if !ok {
				return fmt.Errorf("invalid key: %s", k)
			}
			nk1 := fmt.Sprintf("%c%s", k[0], c)
			nk2 := fmt.Sprintf("%s%c", c, k[1])
			m2[nk1] = m2[nk1] + v
			m2[nk2] = m2[nk2] + v
		}
		m = m2

		s, err = input.Process(s)
		if err != nil {
			return fmt.Errorf("unable to process string: %s", err)
		}
		fmt.Println(s)

		method1 := CountBigraphs(s)
		fmt.Printf("Method 1 = %v\n", method1)
		fmt.Printf("Method 2 = %v\n", m)

		for k, v := range method1 {
			if m[k] != v {
				fmt.Printf("!!! MISMATCH %s: %d != %d !!!\n", k, m[k], v)
			}
		}
	}
	return nil
}

func (input *Input) Approach1(times int) error {
	s := input.Template
	var err error
	fmt.Println(s)
	for i := 0; i < times; i++ {
		fmt.Printf("i = %d\n", i)
		s, err = input.Process(s)
		if err != nil {
			return fmt.Errorf("unable to process string: %s", err)
		}
		fmt.Println(s)
	}
	CountLetters(s)
	return nil
}

func (input *Input) Approach2(times int) error {
	start := input.Template

	m := map[string]int{}
	for i := 0; i < len(start)-1; i++ {
		m[start[i:i+2]] = m[start[i:i+2]] + 1
	}

	fmt.Printf("%v\n", m)
	for i := 0; i < times; i++ {
		m2 := map[string]int{}
		for k, v := range m {
			c, ok := input.Rules[k]
			if !ok {
				return fmt.Errorf("invalid key: %s", k)
			}
			nk1 := fmt.Sprintf("%c%s", k[0], c)
			nk2 := fmt.Sprintf("%s%c", c, k[1])
			m2[nk1] = m2[nk1] + v
			m2[nk2] = m2[nk2] + v
		}
		m = m2
		fmt.Printf("%v\n", m)
	}

	counts := make(map[string]int)
	counts[string(start[len(start)-1])] = 1
	for k, v := range m {
		counts[k[0:1]] = counts[k[0:1]] + v
	}
	fmt.Printf("counts = %v\n", counts)

	min := -1
	max := 0
	for _, v := range counts {
		if v < min || min < 0 {
			min = v
		}
		if v > max {
			max = v
		}
	}

	fmt.Printf("min = %d, max = %d\n", min, max)
	fmt.Printf("result = %d\n", max-min)
	return nil
}

func main() {
	rules, err := ReadInput()
	if err != nil {
		fmt.Printf("unable to read input: %s\n", err)
		return
	}

	rules.Approach2(40)
}
