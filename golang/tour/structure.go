package main

import (
	"fmt"
	"math"
	"strings"
)

type Vertex struct {
	X int
	Y int
}

func main() {
	i := 42
	p := &i
	fmt.Println(*p)
	*p = 11
	fmt.Println(i)

	j := 2701
	p = &j
	*p = *p / 37
	fmt.Println(j)

	fmt.Println(Vertex{1, 2})
	v := &Vertex{1, 2}
	v.X = 3
	fmt.Println(*v)
	fmt.Println(Vertex{})
	fmt.Println(Vertex{X: 1})

	var a [2]string
	a[0] = "hello"
	a[1] = "world"
	fmt.Println(a[0], a[1], a)

	primes := [6]int{2, 3, 5, 7, 11, 13}
	fmt.Println(primes)
	// [1,4)
	var s []int = primes[1:4]
	fmt.Println(s)

	// create slice directly
	s2 := []struct {
		i int
		b bool
	}{
		{1, true},
		{2, false},
		{3, true},
		{4, false},
	}
	fmt.Printf("len=%d cap=%d %v\n", len(s2), cap(s2), s2)
	fmt.Println(s2[0:])
	fmt.Println(s2[:])

	// slice operation
	b := make([]int, 0, 5)
	fmt.Printf("make len=%d cap=%d %v\n", len(b), cap(b), b)
	b = append(b, 3, 3, 3)
	fmt.Printf("append len=%d cap=%d %v\n", len(b), cap(b), b)
	c := make([]int, len(b)+1)
	copy(c, b)
	fmt.Printf("copy len=%d cap=%d %v\n", len(c), cap(c), c)

	// range
	for i, v := range []int{1, 2, 4, 8, 16} {
		fmt.Printf("2**%d = %d\n", i, v)
	}

	// slice index or value
	pow := make([]int, 5)
	for i := range pow {
		pow[i] = 1 << uint(i) // == 2**i
	}
	fmt.Print("pow")
	for _, value := range pow {
		fmt.Printf(" %d", value)
	}
	fmt.Println()

	// map
	m := make(map[string]int)
	m["first"] = 110
	fmt.Println(m)
	fmt.Println(m["second"]) // 0

	m2 := map[string]int{
		"first":  110,
		"second": 120,
	}
	delete(m2, "first")
	fmt.Println(m2)

	// detect whether the key exists
	elem, ok := m2["third"]
	fmt.Println("elem:", elem, "present?", ok)

	fmt.Println(WordCount("A man a plan a canal panama"))

	// call func with func
	hypot := func(x, y float64) float64 {
		return math.Sqrt(x*x + y*y)
	}
	fmt.Println(compute(3, 4, hypot))
	fmt.Println(compute(3, 4, math.Pow))

	pos, neg := adder(), adder()
	for i := 2; i < 6; i++ {
		fmt.Println(pos(i*i), neg(-5*i))
	}

	f := fibonacci()
	fmt.Print("fibonacci")
	for i := 0; i < 15; i++ {
		fmt.Printf(" %d", f())
	}
	fmt.Println()
}

func WordCount(s string) map[string]int {
	sf := strings.Fields(s)
	m := make(map[string]int)
	for _, f := range sf {
		_, ok := m[f]
		if ok {
			m[f] = m[f] + 1
		} else {
			m[f] = 1
		}
	}
	return m
}

func compute(a, b float64, fn func(float64, float64) float64) float64 {
	return fn(a, b)
}

// Go 函数可以是一个闭包。闭包是一个函数值，它引用了其函数体之外的变量。
// 该函数可以访问并赋予其引用的变量的值，换句话说，该函数被这些变量“绑定”在一起。
// 例如，函数 adder 返回一个闭包。每个闭包都被绑定在其各自的 sum 变量上。
func adder() func(int) int {
	sum := 0
	return func(c int) int {
		sum += c
		return sum
	}
}

func fibonacci() func() int {
	a, b := 0, 1
	return func() (t int) {
		t = a
		a, b = b, a+b
		return t
	}
}
