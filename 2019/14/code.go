package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

type Element struct {
	Element string
	Amount  int
}

func (elem *Element) String() string {
	return fmt.Sprintf("%d %s", elem.Amount, elem.Element)
}

type Equation struct {
	Consequent *Element
	Antecedent []*Element
}

func (eq *Equation) String() string {
	parts := make([]string, len(eq.Antecedent))
	for i, elem := range eq.Antecedent {
		parts[i] = elem.String()
	}
	ant := strings.Join(parts, ", ")
	return fmt.Sprintf("%s => %s", ant, eq.Consequent)
}

func ParseItem(s string) *Element {
	parts := strings.Split(s, " ")
	if len(parts) != 2 {
		fmt.Printf("Error parsing item %q\n", s)
		panic("invalid item")
	}
	n, err := strconv.Atoi(parts[0])
	if err != nil {
		panic(err)
	}
	return &Element{Element: parts[1], Amount: n}
}

func ParseList(s string) []*Element {
	parts := strings.Split(s, ", ")
	result := make([]*Element, len(parts))
	for i, part := range parts {
		result[i] = ParseItem(part)
	}
	return result
}

func ParseEquation(s string) *Equation {
	parts := strings.Split(s, " => ")
	if len(parts) != 2 {
		fmt.Printf("Error parsing equation %q\n", s)
		panic("invalid equation")
	}
	return &Equation{
		Consequent: ParseItem(parts[1]),
		Antecedent: ParseList(parts[0]),
	}
}

func ParseEquations(s string) []*Equation {
	parts := strings.Split(s, "\n")
	result := make([]*Equation, len(parts))
	for i, part := range parts {
		result[i] = ParseEquation(part)
	}
	return result
}

func EquationsToString(eqs []*Equation) string {
	parts := make([]string, len(eqs))
	for i, eq := range eqs {
		parts[i] = eq.String()
	}
	return strings.Join(parts, "\n")
}

func ParseFile(path string) []*Equation {
	b, err := ioutil.ReadFile(path)
	if err != nil {
		panic(err)
	}
	s := strings.TrimSpace(string(b))
	return ParseEquations(s)
}

type Inventory struct {
	Items map[string]int
}

func (inv *Inventory) String() string {
	keys := make([]string, 0, len(inv.Items))
	for s, n := range inv.Items {
		keys = append(keys, fmt.Sprintf("%s:%d", s, n))
	}
	sort.Strings(keys)
	return strings.Join(keys, ",")
}

func (inv *Inventory) OreOnlyAmount() (int, bool) {
	if len(inv.Items) != 1 {
		return 0, false
	}
	amt, ok := inv.Items["ORE"]
	return amt, ok
}

func (inv *Inventory) ApplyRule(rule *Equation) *Inventory {
	invAmt, ok := inv.Items[rule.Consequent.Element]
	if !ok {
		return nil
	}

	factor := int(math.Ceil(float64(invAmt) / float64(rule.Consequent.Amount)))
	newItems := make(map[string]int)
	for elem, amount := range inv.Items {
		newItems[elem] = amount
	}
	for _, elem := range rule.Antecedent {
		newItems[elem.Element] += factor * elem.Amount
	}
	delete(newItems, rule.Consequent.Element)
	return &Inventory{Items: newItems}
}

func TestApplyRule() {
	inv := &Inventory{map[string]int{"FUEL": 3, "A": 1}}
	rule := &Equation{
		Consequent: &Element{"FUEL", 1},
		Antecedent: []*Element{&Element{"A", 2}, &Element{"B", 3}},
	}

	newInv := inv.ApplyRule(rule)
	fmt.Printf("Result: %v", newInv)
}

func SearchBF(rules []*Equation) {
	i := 0
	invs := []*Inventory{&Inventory{Items: map[string]int{"FUEL": 1}}}
	testedInventories := make(map[string]bool)
	for {
		fmt.Printf("Pass %d: %d inventories\n", i, len(invs))
		i++

		// Check whether any inventory is complete.
		for _, inv := range invs {
			amt, ok := inv.OreOnlyAmount()
			if ok {
				fmt.Printf("ORE: %d\n", amt)
				return
			}
		}

		// Loop over the rules and expand the possible cases.
		next := []*Inventory{}
		for _, rule := range rules {
			for _, oldInv := range invs {
				newInv := oldInv.ApplyRule(rule)
				if newInv != nil {
					s := newInv.String()
					if !testedInventories[s] {
						testedInventories[s] = true
						next = append(next, newInv)
					}
				}
			}
		}
		invs = next
	}
}

func CountElements(eqs []*Equation) int {
	e := make(map[string]bool)
	for _, eq := range eqs {
		e[eq.Consequent.Element] = true
		for _, elem := range eq.Antecedent {
			e[elem.Element] = true
		}
	}
	return len(e)
}

func main() {
	equations := ParseFile(os.Args[1])
	fmt.Println("Equations:")
	fmt.Printf("%v\n\n", EquationsToString(equations))
	fmt.Printf("Elements: %d\n\n", CountElements(equations))

	//TestApplyRule()
	SearchBF(equations)
}

/*

Input | Elems | Passes | Ore     | Time (s)
------|-------|--------|---------|----------
    1 |     7 |      6 |      31 |      0.3
    2 |     8 |      7 |     165 |      0.8
    3 |    10 |      9 |   13312 |      0.3
		4 |    13 |     12 |  180697 |      8.5
		5 |    18 |        |         |

*/
