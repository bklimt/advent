package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
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

func (r *LiteralRule) Generate(rules *RuleSet) []string {
	return []string{fmt.Sprintf("%c", r.Character)}
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

func (r *SequenceRule) Generate(rules *RuleSet) []string {
	current := []string{""}
	for _, i := range r.SubRules {
		rule := rules.Rules[i]
		subs := rule.Generate(rules)
		next := []string{}
		for _, s1 := range current {
			for _, s2 := range subs {
				next = append(next, fmt.Sprintf("%s%s", s1, s2))
			}
		}
		current = next
	}
	return current
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

func (r *DisjunctiveRule) Generate(rules *RuleSet) []string {
	result := []string{}
	for _, rule := range r.SubRules {
		added := rule.Generate(rules)
		result = append(result, added...)
	}
	return result
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
	Generate(rules *RuleSet) []string
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

type Part2Matcher struct {
	PartLen int
	Rule42  []string
	Rule31  []string
}

func Contains(ss []string, s string) bool {
	i := sort.SearchStrings(ss, s)
	return i < len(ss) && ss[i] == s
}

func (m *Part2Matcher) Match(s string) bool {
	if len(s) < m.PartLen*3 {
		return false
	}
	if len(s)%m.PartLen != 0 {
		return false
	}
	n := len(s) / m.PartLen
	parts := []string{}
	for i := 0; i < n; i++ {
		parts = append(parts, s[i*m.PartLen:(i+1)*m.PartLen])
	}

	var rule42 int
	for rule42 = 0; rule42 < len(parts) && Contains(m.Rule42, parts[rule42]); rule42++ {
	}
	// rule42 now points one past the last element that matched rule42.
	if rule42 < 2 {
		return false
	}

	var rule31 int
	for rule31 = len(parts); rule31 > 0 && Contains(m.Rule31, parts[rule31-1]); rule31-- {
	}
	// rule31 now points to the first element that matched rule31.
	if rule31 == len(parts) {
		return false
	}

	// Make sure they at least overlap.
	if rule42 < rule31 {
		return false
	}

	// Make sure there's at least as many rule42s and rule31s.
	if rule42 <= len(parts)-rule31 {
		return false
	}

	fmt.Printf("  %s matches at %d\n", strings.Join(parts, " | "), rule31)
	return true
}

func NewPart2Matcher(rules *RuleSet) *Part2Matcher {
	partLen := 0

	fmt.Printf("\n\nRule 42 strings:\n")
	rule42s := rules.Rules[42].Generate(rules)
	sort.Strings(rule42s)
	for _, s := range rule42s {
		fmt.Printf("%s\n", s)
		if partLen == 0 {
			partLen = len(s)
		} else if partLen != len(s) {
			panic("what?")
		}
	}

	fmt.Printf("\n\nRule 31 strings:\n")
	rule31s := rules.Rules[31].Generate(rules)
	sort.Strings(rule31s)
	for _, s := range rule31s {
		fmt.Printf("%s\n", s)
		if partLen == 0 {
			partLen = len(s)
		} else if partLen != len(s) {
			panic("what?")
		}
	}

	return &Part2Matcher{Rule42: rule42s, Rule31: rule31s, PartLen: partLen}
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

	part2 := NewPart2Matcher(rules)

	count1 := 0
	count2 := 0
	fmt.Printf("\n\nMessages:\n")
	for scanner.Scan() {
		message := scanner.Text()
		matches1 := rules.Match(message)
		matches2 := part2.Match(message)
		fmt.Printf("%s: %v, %v\n", message, matches1, matches2)
		if debug {
			fmt.Println()
		}
		if matches1 {
			count1 = count1 + 1
		}
		if matches2 {
			count2 = count2 + 1
		}
	}
	// Part 1: 195
	fmt.Printf("\nCount 1: %d\n", count1)
	// Part 2: <318. It's not 312.
	fmt.Printf("Count 2: %d\n", count2)

	return nil
}

func main() {
	if err := ProcessFile("input.txt"); err != nil {
		fmt.Printf("%v\n", err)
	}
}

// 0 -> 8 11
// 8 and 11 both generate chunks of sizes that are multiples of 8
// So, yeah...
//
// A{1, 11-2N} A{N} B{N} for N=[0, 5]
//
// A
// A A
// A A A
// A A A A
// A A A A A
// A A A A A A
// A A A A A A A
// A A A A A A A A
// A A A A A A A A A
// A A A A A A A A A A
// A A A A A A A A A A A
// A A B
// A A A B B
// A A A A B B B
// A A A A A B B B B
// A A A A A A B B B B B
// (plus any number of As)
