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

//
// Parsing
//

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
		// keys = append(keys, fmt.Sprintf("%s:%d", s, n))
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

//
// Backward Inference Logic
//

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

//
// Backward Inference Breadth-First Search
//

var elemScore map[string]int = make(map[string]int)

type SearchState struct {
	Inventory     *Inventory
	EquationsUsed map[string]int
	Depth         int
	Path          []int
}

func (state *SearchState) Score() float64 {
	// num := 0
	// min := 50
	// prod := 1.0
	count := 0
	invSum := 0.0
	for elem, _ := range state.Inventory.Items {
		// prod = prod * float64(elemScore[elem]+1)
		count++
		invSum = invSum + (1.0 / float64(elemScore[elem]+1))
		/*
			num = num + elemScore[elem]
			if elemScore[elem] < min {
				min = elemScore[elem]
			}
		*/
	}
	//avg := float64(num) / float64(count)
	//return float64(min) + (avg / 10.0) + (float64(len(state.Inventory.Items)) / 100.0)
	//return prod
	return float64(count) / invSum
}

type byScore []*SearchState

func (s byScore) Len() int {
	return len(s)
}
func (s byScore) Swap(i, j int) {
	s[i], s[j] = s[j], s[i]
}
func (s byScore) Less(i, j int) bool {
	return s[i].Score() < s[j].Score()
}

func FancySearch(equations []*Equation, equationLimit int) {
	queue := []*SearchState{&SearchState{
		Inventory: &Inventory{Items: map[string]int{"FUEL": 1}},
		Path:      make([]int, 0, len(equations)*equationLimit),
	}}

	stateSeen := make(map[string]bool)

	for len(queue) > 0 {
		// Find all the new states.
		state := queue[0]
		queue = queue[1:]
		fmt.Printf("%f: %s\n", state.Score(), state.Inventory.String())
		for _, eq := range equations {
			if newState := state.MaybeApplyEquation(eq, equationLimit); newState != nil {
				amt, ok := newState.Inventory.OreOnlyAmount()
				if ok {
					fmt.Printf("\nORE: %d\n", amt)
					return
				}

				// Check if this state has already been visited.
				invStr := newState.Inventory.String()
				if stateSeen[invStr] {
					// fmt.Printf("Skipping %s\n", invStr)
					continue
				}
				stateSeen[invStr] = true
				queue = append(queue, newState)
			}
		}
		// Sort all the states.
		sort.Sort(byScore(queue))
	}
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
	return &SearchState{Inventory: newInv, EquationsUsed: newUsed, Depth: state.Depth + 1}
}

func SearchBFS(equations []*Equation, equationLimit int) {
	states := []*SearchState{
		&SearchState{Inventory: &Inventory{Items: map[string]int{"FUEL": 1}}},
	}
	testedInventories := make(map[string]bool, 10000000)
	for i := 0; true; i++ {
		fmt.Printf("BFS pass %d: %d inventories\n", i, len(states))

		// Loop over the rules and expand the possible cases.
		nextStates := []*SearchState{}
		for si, oldState := range states {
			fmt.Printf("bfs: %d/%d         \r", si, len(states))

			/*
				if len(states) > 100 {
					if SearchDFS(equations, equationLimit, testedInventories, oldState) {
						return
					}
					continue
				}
			*/

			for _, eq := range equations {
				newState := oldState.MaybeApplyEquation(eq, equationLimit)
				if newState != nil {
					s := newState.Inventory.String()
					if !testedInventories[s] {
						// Test whether we're done here.
						amt, ok := newState.Inventory.OreOnlyAmount()
						if ok {
							fmt.Printf("\nORE: %d\n", amt)
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

func SearchDFS(equations []*Equation, equationLimit int, state *SearchState) bool {
	if state == nil {
		state = &SearchState{
			Inventory: &Inventory{Items: map[string]int{"FUEL": 1}},
			Path:      make([]int, 0, len(equations)*equationLimit),
		}
	}
	if len(state.Path) == 0 {
		state.Path = append(state.Path, 0)
	}
	//if state.Path[len(state.Path)-1] == 0 {
	fmt.Printf("dfs: [%d/%d] ", state.Depth, len(equations)*equationLimit)
	for _, n := range state.Path {
		fmt.Printf("%d / ", n)
	}
	fmt.Printf("                                    \r")
	//}
	// Loop over the rules and expand the possible cases.
	for i, eq := range equations {
		newState := state.MaybeApplyEquation(eq, equationLimit)
		if newState != nil {
			newState.Path = make([]int, len(state.Path), len(equations)*equationLimit)
			copy(newState.Path, state.Path)
			newState.Path = append(newState.Path, i)

			//s := newState.Inventory.String()
			// if !testedInventories[s] {
			// Test whether we're done here.
			amt, ok := newState.Inventory.OreOnlyAmount()
			if ok {
				fmt.Printf("\nORE: %d\n", amt)
				return true
			}
			//testedInventories[s] = true

			if SearchDFS(equations, equationLimit, newState) {
				return true
			}
			//}
		}
	}
	return false
}

//
// Forward Inference Equation Simplification
//
func SimplifyEquations(eqs []*Equation) {
	byElement := make(map[string][]*Equation)
	// elemScore = make(map[string]int)
	eqScore := make(map[string]int)

	// Initialize all the arrays.
	for _, eq := range eqs {
		byElement[eq.Consequent.Element] = []*Equation{}
		for _, elem := range eq.Antecedent {
			byElement[elem.Element] = []*Equation{}
		}
	}

	// Populate the arrays.
	for _, eq := range eqs {
		eqScore[eq.String()] = -1

		byElement[eq.Consequent.Element] = append(byElement[eq.Consequent.Element], eq)
		for _, elem := range eq.Antecedent {
			byElement[elem.Element] = append(byElement[elem.Element], eq)
		}
	}

	// Compute element scores.
	elemScore["ORE"] = 0
	updated := true
	fmt.Printf("Computing element scores")
	for updated {
		fmt.Printf(".")
		updated = false
		for _, eq := range eqs {
			if _, ok := elemScore[eq.Consequent.Element]; ok {
				continue
			}

			valid := true
			maxScore := -1
			for _, elem := range eq.Antecedent {
				if score, ok := elemScore[elem.Element]; ok {
					if score > maxScore {
						maxScore = score
					}
				} else {
					valid = false
				}
			}
			if valid {
				elemScore[eq.Consequent.Element] = maxScore + 1
				eqScore[eq.String()] = maxScore + 1
				updated = true
			}
		}
	}
	fmt.Printf("\n")

	// Get the unique list of elements for sorting.
	elems := make([]string, 0, len(byElement))
	for elem, _ := range byElement {
		elems = append(elems, elem)
	}
	sort.Strings(elems)

	// Print the maps.
	fmt.Printf("\nElement scores:\n")
	for _, elem := range elems {
		if score, ok := elemScore[elem]; ok {
			fmt.Printf("%d: ", score)
		} else {
			fmt.Println("?: ")
		}
		fmt.Printf("%s\n", elem)
		/*
			eqs := byElement[elem]
			for _, eq := range eqs {
				fmt.Printf("  %s\n", eq.String())
			}
		*/
	}

	fmt.Printf("\nEquation scores:\n")
	for eq, score := range eqScore {
		fmt.Printf("%d: %s\n", score, eq)
	}
}

//
// Metrics
//

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

	// TestApplyRule()
	// SearchBFS(equations, 1)
	// SearchDFS(equations, 1, nil)

	SimplifyEquations(equations)
	fmt.Println()
	FancySearch(equations, 1)
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
