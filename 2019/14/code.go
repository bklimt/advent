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
	Str        string
}

func (eq *Equation) String() string {
	/*
		parts := make([]string, len(eq.Antecedent))
		for i, elem := range eq.Antecedent {
			parts[i] = elem.String()
		}
		ant := strings.Join(parts, ", ")
		return fmt.Sprintf("%s => %s", ant, eq.Consequent)
	*/
	return eq.Str
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
		Str:        s,
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
	for s, _ := range inv.Items {
		//keys = append(keys, fmt.Sprintf("%s:%d", s, n))
		keys = append(keys, s)
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

func (inv *Inventory) ApplyEquation(eq *Equation) *Inventory {
	invAmt, ok := inv.Items[eq.Consequent.Element]
	if !ok {
		return nil
	}

	factor := int(math.Ceil(float64(invAmt) / float64(eq.Consequent.Amount)))
	newItems := make(map[string]int, len(inv.Items))
	for elem, amount := range inv.Items {
		newItems[elem] = amount
	}
	for _, elem := range eq.Antecedent {
		newItems[elem.Element] += factor * elem.Amount
	}
	delete(newItems, eq.Consequent.Element)
	return &Inventory{Items: newItems}
}

func TestApplyEquation() {
	inv := &Inventory{map[string]int{"FUEL": 3, "A": 1}}
	rule := &Equation{
		Consequent: &Element{"FUEL", 1},
		Antecedent: []*Element{&Element{"A", 2}, &Element{"B", 3}},
	}

	newInv := inv.ApplyEquation(rule)
	fmt.Printf("Result: %v", newInv)
}

type SearchState struct {
	Inventory     *Inventory
	EquationsUsed map[string]int
}

func (state *SearchState) MaybeApplyEquation(eq *Equation, limit int) *SearchState {
	// If we've already used this equation too much, then don't try it again.
	if state.EquationsUsed[eq.String()] >= limit {
		return nil
	}
	// Try the equation.
	newInv := state.Inventory.ApplyEquation(eq)
	if newInv == nil {
		return nil
	}
	// Build the new state.
	newUsed := make(map[string]int, len(state.EquationsUsed)+1)
	for s, n := range state.EquationsUsed {
		newUsed[s] = n
	}
	newUsed[eq.String()] = newUsed[eq.String()] + 1
	return &SearchState{Inventory: newInv, EquationsUsed: newUsed}
}

func SearchBFS(equations []*Equation, equationLimit int) {
	states := []*SearchState{
		&SearchState{Inventory: &Inventory{Items: map[string]int{"FUEL": 1}}},
	}
	testedInventories := make(map[string]bool, 10000000)
	for i := 0; true; i++ {
		fmt.Printf("Pass %d: %d inventories\n", i, len(states))

		// Loop over the rules and expand the possible cases.
		nextStates := []*SearchState{}
		for si, oldState := range states {
			for _, eq := range equations {
				fmt.Printf("%d/%d\r", si, len(states))
				newState := oldState.MaybeApplyEquation(eq, equationLimit)
				if newState != nil {
					s := newState.Inventory.String()
					if !testedInventories[s] {
						// Test whether we're done here.
						amt, ok := newState.Inventory.OreOnlyAmount()
						if ok {
							fmt.Printf("ORE: %d\n", amt)
							return
						}

						testedInventories[s] = true
						nextStates = append(nextStates, newState)
					}
				}
			}
		}
		states = nextStates
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
	SearchBFS(equations, 1)
}

/*

Input | Elems | Passes | Ore     | Time (s)
------|-------|--------|---------|----------
    1 |     7 |      5 |      31 |      0.3
    2 |     8 |      6 |     165 |      0.8
    3 |    10 |      8 |   13312 |      0.3
		4 |    13 |     11 |  180697 |      1.2
		5 |    18 |     16 | 2210736 |      6.7
    - |    64 |      ? |       ? |        ?
*/
