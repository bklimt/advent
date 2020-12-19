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

type Rule8 struct {
	Rule42 []string
}

func (r *Rule8) String() string {
	return "[Rule 8]"
}

// Eh, there's mulitple ways for it to match now.
func (r *Rule8) Match(s string, rules *RuleSet, indent int) (bool, int) {
	if debug {
		fmt.Printf("%sTrying %s with %s\n", strings.Repeat("  ", indent), r, s)
	}
	if debug {
		fmt.Printf("%sReturning false\n", strings.Repeat("  ", indent+1))
	}
	return false, 0
}

// Eh, this will be 10 * len(rule42)...
func (r *Rule8) Generate(rules *RuleSet) []string {
	return []string{"**** Rule 8 *****"}
}

type Rule11 struct {
	Rule31 []string
	Rule42 []string
}

func (r *Rule11) String() string {
	return "[Rule 8]"
}

// Eh, there's mulitple ways for it to match now.
func (r *Rule11) Match(s string, rules *RuleSet, indent int) (bool, int) {
	if debug {
		fmt.Printf("%sTrying %s with %s\n", strings.Repeat("  ", indent), r, s)
	}
	if debug {
		fmt.Printf("%sReturning false\n", strings.Repeat("  ", indent+1))
	}
	return false, 0
}

// Eh, this will be ... big ...
func (r *Rule11) Generate(rules *RuleSet) []string {
	return []string{"**** Rule 11 *****"}
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
	// Part 1: 195
	fmt.Printf("\nCount: %d", count)

	fmt.Printf("\n\nRule 42 strings:\n")
	rule42s := rules.Rules[42].Generate(rules)
	sort.Strings(rule42s)
	for _, s := range rule42s {
		fmt.Printf("%s\n", s)
	}

	fmt.Printf("\n\nRule 31 strings:\n")
	rule31s := rules.Rules[31].Generate(rules)
	sort.Strings(rule31s)
	for _, s := range rule31s {
		fmt.Printf("%s\n", s)
	}

	rules.Rules[8] = &Rule8{Rule42: rule42s}
	rules.Rules[11] = &Rule11{Rule31: rule31s, Rule42: rule42s}

	fmt.Printf("\n\nRule 0 strings:\n")
	rule0s := rules.Rules[0].Generate(rules)
	sort.Strings(rule0s)
	for _, s := range rule0s {
		fmt.Printf("%s\n", s)
	}

	return nil
}

func main() {
	if err := ProcessFile("input.txt"); err != nil {
		fmt.Printf("%v\n", err)
	}
}

//
// 0 -> 8 11
// 8 and 11 both generate chunks of sizes that are multiples of 8
// So, yeah...
//
// 42
//   20 86
//     20
//       a
//     86
//       104 20 (bbaaab|bbaabb|bbabaa|bbabab)
//         104
//           91 70 (bbaaab|bbaabb|bbabaa|bbabab)
//             91
//               b
//             70
//               91 24 (baaab|baabb|babaa|babab)
//                 91
//                   b
//                 24
//                   20 83 (aaab|aabb|abaa|abab)
//                     20
//                       a
//                     83 (aab|abb|baa|bab)
//                       1 20 (baa)
//                         1 (ba)
//                           91 20 (ba)
//                             91
//                               b
//                             20
//                               a
//                         20
//                           a
//                       60 91 (aab|abb|bab)
//                         60 (aa|ab|ba)
//                           91 20 (ba)
//                           20 106 (aa|ab)
//                             20
//                               a
//                             106 (b|a)
//                               91
//                                 b
//                               20
//                                 a
//                         91
//                           b
//                   91 64
//                     91
//                     64
//               20 34
//           20 108
//         20
//           a
//       9 91
//   91 56
//     91
//       b
//     56
//       91 121 | 20 61
// 31
//   91 69
//   20 47
