package main

import (
	"fmt"
	"math"
)

type Rect struct {
	Top    int
	Left   int
	Bottom int
	Right  int
}

func (r Rect) Subtract(x int, y int) Rect {
	return Rect{
		Top:    r.Top - y,
		Left:   r.Left - x,
		Bottom: r.Bottom - y,
		Right:  r.Right - x,
	}
}

/*
initial x speed = 10

sum_to(x) = (x * (x + 1)) / 2

max x distance = sum_to(10) = 55

after 3 steps: sum_to(10) - sum_to(10 - 3) = 34

0: 0
1: 10
2: 19
3: 27
4: 34
5: 40
6: 45
7: 49
8: 52
9: 54
10: 55
*/

func sumTo(x int) int {
	return (x * (x + 1)) / 2
}

func main() {
	r := Rect{
		Top:    -10,
		Left:   20,
		Bottom: -5,
		Right:  30,
	}

	// TODO: If the rectangle is to the left, flip the universe.

	for initialXSpeed := 1; initialXSpeed <= r.Right; initialXSpeed++ {
		fmt.Printf("initial x speed = %d\n", initialXSpeed)

		// The furthest right it could go before reaching 0 x velocity.
		maxX := sumTo(initialXSpeed)
		if maxX < r.Left {
			continue
		}

		// This x speed is neither too slow nor too fast, but it still might not hit.
		for steps := 1; steps <= initialXSpeed; steps++ {
			fmt.Printf("  %d steps\n", steps)
			// TODO: Well, this is wrong.
			x := maxX - sumTo(initialXSpeed-steps)
			fmt.Printf("    x = %d\n", x)
			if x < r.Left {
				continue
			}
			if x > r.Right {
				break
			}

			// So, we can hit the x range with this many steps at this initial speed.

			// Figure out the y aspect...
			// The fall part is maxY = finalY + sumTo(remaining steps).
			// The rise part is maxY = ???
			//
			// If initial y speed is 5...
			// 0 5 9 12 14 15 15 14 12 9 5 0 -6 -13 -21
			//
			// x @ maxY = initial y speed
			//
			// height at x = sumTo(initial y speed) - sumTo(x - initial y speed)
			//
			// h(x) = (ysi*(ysi+1))/2 - ((x-ysi)*((x-ysi)+1))/2
			//
			// 2 * h(x) = (ysi*(ysi+1)) - ((x-ysi)*((x-ysi)+1))
			//
			// 2 * h(x) = ysi^2 + ysi - ((x-ysi)^2 + (x-ysi))
			// 2 * h(x) = ysi^2 + ysi - (x^2 - 2ysi + ysi^2 + x - ysi)
			// 2 * h(x) = ysi^2 + ysi - x^2 + 2ysi - ysi^2 - x + ysi
			// 2 * h(s) = 4ysi - x^2 - ysi^2
			// 0 = -ysi^2 + 4ysi - (2h + x^2)
			//
			// tmp = -4 * (2h + x^2)
			// ysi = (-4 +/- sqrt(c)) / -2
			//
			tmp := float64(-4 * ((2 * r.Top) + (x * x)))
			if tmp < 0 {
				fmt.Println("      no solution!")
				continue
			}
			minInitialYSpeedF := (-4 + math.Sqrt(tmp)) / -2
			maxInitialYSpeedF := (-4 - math.Sqrt(tmp)) / -2
			fmt.Printf("      initial y speed must be between %f and %f\n", minInitialYSpeedF, maxInitialYSpeedF)
		}
	}
}
