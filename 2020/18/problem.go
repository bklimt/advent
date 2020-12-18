package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
	"unicode"
)

func ProcessTerm(r *strings.Reader) (int, error) {
	c, _, err := r.ReadRune()
	if err != nil {
		return 0, err
	}
	if c == '(' {
		return ProcessExpression(r)
	}
	if unicode.IsDigit(c) {
		return strconv.Atoi(fmt.Sprintf("%c", c))
	}
	return 0, fmt.Errorf("expected term; got %q", c)
}

func ProcessExpression(r *strings.Reader) (int, error) {
	n, err := ProcessTerm(r)
	if err != nil {
		return 0, err
	}

	for true {
		c, _, err := r.ReadRune()

		if err == io.EOF {
			return n, nil
		}
		if err != nil {
			return 0, err
		}
		if c == ')' {
			return n, nil
		}
		if c == ' ' {
			// Okay, there was a space, so now we need to read an operator.
			op, _, err := r.ReadRune()
			if err != nil {
				return 0, err
			}
			multiply := false
			if op == '*' {
				multiply = true
			} else if op != '+' {
				return 0, fmt.Errorf("expected +|*; got %q", c)
			}
			space, _, err := r.ReadRune()
			if err != nil {
				return 0, err
			}
			if space != ' ' {
				return 0, fmt.Errorf("expected space; got %q", c)
			}

			// Now read the next term.
			m, err := ProcessTerm(r)
			if err != nil {
				return 0, err
			}

			if multiply {
				n = n * m
			} else {
				n = n + m
			}

			continue
		}
		return 0, fmt.Errorf("expected space or end-of-expression; got %q", c)
	}

	return 0, fmt.Errorf("unreachable code")
}

func ProcessFile(path string) error {
	f, err := os.Open(path)
	if err != nil {
		return err
	}
	defer f.Close()

	total := 0
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		r := strings.NewReader(scanner.Text())
		n, err := ProcessExpression(r)
		if err != nil {
			return err
		}
		fmt.Printf("%s = %d\n", scanner.Text(), n)
		total = total + n
	}
	fmt.Printf("Total: %d", total)
	return nil
}

func main() {
	if err := ProcessFile("input.txt"); err != nil {
		fmt.Printf("%v\n", err)
	}
}
