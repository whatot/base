package main

import (
	"fmt"
	"math"
	"math/rand"
)

const PPI = 3.14

func add(a int, b int) int {
	return a + b
}

// merged same type params
func add3(a, b, c int) int {
	return a + b + c
}

// return more result
func swap(a, b int) (int, int) {
	return b, a
}

// naming return value
func split(sum int) (x, y int) {
	x = sum * 4 / 9
	y = sum - x
	return
}

func main() {
	fmt.Println("golang number is ", rand.Intn(100))
	fmt.Println("test sqrt ", math.Sqrt(7))
	fmt.Println("test pi", math.Pi)
	fmt.Println("test phi", math.Phi)
	fmt.Println("test const ppi", PPI)
	fmt.Println("test add ", add(6, 7))
	fmt.Println("test add3 ", add3(3, 4, 5))

	a, b := swap(1, 9)
	fmt.Println("test swap ", a, b)

	x, y := split(110)
	fmt.Println("test split ", x, y)

	var i int
	var j int = 2
	k := float64(j)
	fmt.Println("test var ", i, j/2.0, k/2.0)

}
