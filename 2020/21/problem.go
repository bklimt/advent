package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type Recipe struct {
	Ingredients map[int]bool
	Allergens   map[int]bool
}

type Input struct {
	AllergenIdToName   []string
	AllergenNameToId   map[string]int
	IngredientIdToName []string
	IngredientNameToId map[string]int
	Recipes            []*Recipe
}

func (input *Input) Filter() (bool, bool, error) {
	changed := false
	finished := true
	for allergenId, allergenName := range input.AllergenIdToName {
		fmt.Printf("Considering allergen %d %q\n", allergenId, allergenName)
		found := false
		intersection := make(map[int]bool)
		for _, recipe := range input.Recipes {
			if _, ok := recipe.Allergens[allergenId]; !ok {
				continue
			}
			if !found {
				// Populate the list.
				found = true
				for k, v := range recipe.Ingredients {
					intersection[k] = v
				}
			} else {
				// Filter out anything not in here.
				for ingredientId, _ := range intersection {
					if _, ok := recipe.Ingredients[ingredientId]; !ok {
						delete(intersection, ingredientId)
					}
				}
			}
		}
		if !found {
			// return false, false, fmt.Errorf("allergen %d %q not found in any recipe", allergenId, allergenName)
			fmt.Printf("  was not found in any recipes\n")
			continue
		}
		finished = false
		if len(intersection) != 1 {
			fmt.Printf("  had a non-trivial intersection\n")
			continue
		}

		changed = true
		ingredientId := 0
		for k, _ := range intersection {
			ingredientId = k
		}
		ingredientName := input.IngredientIdToName[ingredientId]
		fmt.Printf("  was only found in 1 ingredient: %d %q\n", ingredientId, ingredientName)

		for _, recipe := range input.Recipes {
			delete(recipe.Allergens, allergenId)
			delete(recipe.Ingredients, ingredientId)
		}
	}
	return finished, changed, nil
}

func (input *Input) Part1() int {
	ans := 0
	for _, recipe := range input.Recipes {
		ans = ans + len(recipe.Ingredients)
	}
	return ans
}

func (input *Input) ParseRecipe(line string) error {
	parts := strings.Split(line, " (contains ")
	if len(parts) != 2 {
		return fmt.Errorf("did not find contains string in %q", line)
	}
	parts[1] = parts[1][:len(parts[1])-1]

	ingredientNames := strings.Split(parts[0], " ")
	allergenNames := strings.Split(parts[1], ", ")
	recipe := &Recipe{
		Ingredients: make(map[int]bool),
		Allergens:   make(map[int]bool),
	}

	for _, ingredientName := range ingredientNames {
		id, ok := input.IngredientNameToId[ingredientName]
		if ok {
			recipe.Ingredients[id] = true
		} else {
			id = len(input.IngredientIdToName)
			input.IngredientIdToName = append(input.IngredientIdToName, ingredientName)
			input.IngredientNameToId[ingredientName] = id
			recipe.Ingredients[id] = true
		}
	}

	for _, allergenName := range allergenNames {
		id, ok := input.AllergenNameToId[allergenName]
		if ok {
			recipe.Allergens[id] = true
		} else {
			id = len(input.AllergenIdToName)
			input.AllergenIdToName = append(input.AllergenIdToName, allergenName)
			input.AllergenNameToId[allergenName] = id
			recipe.Allergens[id] = true
		}
	}

	input.Recipes = append(input.Recipes, recipe)

	return nil
}

func ReadInput(path string) (*Input, error) {
	f, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer f.Close()
	scanner := bufio.NewScanner(f)

	input := &Input{
		AllergenIdToName:   []string{},
		AllergenNameToId:   make(map[string]int),
		IngredientIdToName: []string{},
		IngredientNameToId: make(map[string]int),
	}

	for scanner.Scan() {
		if scanner.Text() != "" {
			if err := input.ParseRecipe(scanner.Text()); err != nil {
				return nil, err
			}
		}
	}

	return input, nil
}

func main() {
	input, err := ReadInput("input.txt")
	if err != nil {
		fmt.Printf("unable to read input: %s\n", err)
		return
	}

	pass := 0
	changed := true
	finished := false
	for changed {
		pass++
		fmt.Printf("\nPass %d:\n", pass)
		finished, changed, err = input.Filter()
		if err != nil {
			fmt.Printf("unable to read input: %s\n", err)
			return
		}
	}
	if !finished {
		fmt.Printf("did not find a solution")
		return
	}

	fmt.Printf("\nPart 1: %d\n", input.Part1())
}
