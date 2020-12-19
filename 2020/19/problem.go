package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

const debug = false

type LiteralRule struct {
	Character byte
}

func (r *LiteralRule) Match(s string, rules *RuleSet, indent int) (bool, int) {
	if debug {
		fmt.Printf("%sTrying %s with %s\n", strings.Repeat("  ", indent), r, s)
	}
	if len(s) == 0 {
		if debug {
			fmt.Printf("%sReturning false: empty string\n", strings.Repeat("  ", indent+1))
		}
		return false, 0
	}
	if s[0] == r.Character {
		if debug {
			fmt.Printf("%sReturning true\n", strings.Repeat("  ", indent+1))
		}
		return true, 1
	}
	if debug {
		fmt.Printf("%sReturning false: no match\n", strings.Repeat("  ", indent+1))
	}
	return false, 0
}

func (r *LiteralRule) String() string {
	return fmt.Sprintf("%q", r.Character)
}

type SequenceRule struct {
	SubRules []int
}

func (r *SequenceRule) Match(s string, rules *RuleSet, indent int) (bool, int) {
	if debug {
		fmt.Printf("%sTrying %s with %s\n", strings.Repeat("  ", indent), r, s)
	}
	eaten := 0
	for _, rule := range r.SubRules {
		matched, n := rules.Rules[rule].Match(s[eaten:], rules, indent+1)
		if !matched {
			if debug {
				fmt.Printf("%sReturning false\n", strings.Repeat("  ", indent+1))
			}
			return false, 0
		}
		eaten = eaten + n
	}
	if debug {
		fmt.Printf("%sReturning true\n", strings.Repeat("  ", indent+1))
	}
	return true, eaten
}

func (r *SequenceRule) String() string {
	ss := make([]string, len(r.SubRules))
	for i, n := range r.SubRules {
		ss[i] = fmt.Sprintf("%d", n)
	}
	return strings.Join(ss, " ")
}

type DisjunctiveRule struct {
	SubRules []*SequenceRule
}

func (r *DisjunctiveRule) Match(s string, rules *RuleSet, indent int) (bool, int) {
	if debug {
		fmt.Printf("%sTrying %s with %s\n", strings.Repeat("  ", indent), r, s)
	}
	for _, rule := range r.SubRules {
		matched, n := rule.Match(s, rules, indent+1)
		if matched {
			if debug {
				fmt.Printf("%sReturning true\n", strings.Repeat("  ", indent+1))
			}
			return true, n
		}
	}
	if debug {
		fmt.Printf("%sReturning false\n", strings.Repeat("  ", indent+1))
	}
	return false, 0
}

func (r *DisjunctiveRule) String() string {
	ss := make([]string, len(r.SubRules))
	for i, n := range r.SubRules {
		ss[i] = fmt.Sprintf("%s", n)
	}
	return strings.Join(ss, " | ")
}

type Rule interface {
	Match(s string, rules *RuleSet, indent int) (bool, int)
}

type RuleSet struct {
	Rules map[int]Rule
}

func (rs *RuleSet) Match(s string) bool {
	matched, n := rs.Rules[0].Match(s, rs, 0)
	if !matched {
		return false
	}
	if n != len(s) {
		return false
	}
	return true
}

func (rs *RuleSet) String() string {
	ss := []string{}
	for i, r := range rs.Rules {
		ss = append(ss, fmt.Sprintf("%d: %s", i, r))
	}
	return strings.Join(ss, "\n")
}

func ParseSequence(s string) (*SequenceRule, error) {
	parts := strings.Split(s, " ")
	ns := make([]int, len(parts))
	for i, part := range parts {
		n, err := strconv.Atoi(part)
		if err != nil {
			return nil, fmt.Errorf("invalid sequence element %q in %q", part, s)
		}
		ns[i] = n
	}
	return &SequenceRule{ns}, nil
}

func ParseRule(s string) (int, Rule, error) {
	parts := strings.Split(s, ": ")
	if len(parts) != 2 {
		return 0, nil, fmt.Errorf("invalid rule missing colon: %q", s)
	}

	n, err := strconv.Atoi(parts[0])
	if err != nil {
		return 0, nil, fmt.Errorf("invalid rule lhs is not an int: %v for %q in %q", err, parts[0], s)
	}

	rhs := parts[1]

	if rhs[0] == '"' {
		if len(rhs) != 3 {
			return 0, nil, fmt.Errorf("invalid rule for character: %q in %q", rhs, s)
		}
		return n, &LiteralRule{rhs[1]}, nil
	}

	rhsParts := strings.Split(rhs, " | ")
	if len(rhsParts) == 1 {
		seq, err := ParseSequence(rhsParts[0])
		if err != nil {
			return 0, nil, fmt.Errorf("invalid sequence %q in %q: %s", rhsParts[0], s, err)
		}
		return n, seq, nil
	}

	rules := make([]*SequenceRule, len(rhsParts))
	for i, part := range rhsParts {
		rule, err := ParseSequence(part)
		if err != nil {
			return 0, nil, fmt.Errorf("invalid rule sequence %q in %q", part, s)
		}
		rules[i] = rule
	}
	return n, &DisjunctiveRule{rules}, nil
}

func ProcessFile(path string) error {
	f, err := os.Open(path)
	if err != nil {
		return err
	}
	defer f.Close()
	scanner := bufio.NewScanner(f)

	rules := &RuleSet{Rules: make(map[int]Rule)}
	for scanner.Scan() {
		if scanner.Text() == "" {
			break
		}
		n, rule, err := ParseRule(scanner.Text())
		if err != nil {
			return err
		}
		rules.Rules[n] = rule
	}
	fmt.Printf("Rules:\n%s", rules)

	count := 0
	fmt.Printf("\n\nMessages:\n")
	for scanner.Scan() {
		message := scanner.Text()
		matches := rules.Match(message)
		fmt.Printf("%s: %v\n", message, matches)
		if debug {
			fmt.Println()
		}
		if matches {
			count = count + 1
		}
	}
	fmt.Printf("\nCount: %d", count)

	return nil
}

func main() {
	if err := ProcessFile("input.txt"); err != nil {
		fmt.Printf("%v\n", err)
	}
}
