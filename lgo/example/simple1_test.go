package example

import (
	"errors"
	"fmt"
	"math"
	"sort"
	"time"
)

func ExampleHelloWorld() {
	fmt.Println("Hello World!")
	// Output: Hello World!
}

func ExampleValues() {
	fmt.Println("go" + "lang")
	fmt.Println("1+1 =", 1+1)
	fmt.Println("7.0/3.0 =", 7.0/3.0)
	fmt.Println(true && false)
	fmt.Println(true || false)
	fmt.Println(!true)
	// Output:
	// golang
	// 1+1 = 2
	// 7.0/3.0 = 2.3333333333333335
	// false
	// true
	// false
}

func ExampleVariables() {
	var a = "initial"
	fmt.Println(a)
	var b, c = 1, 2
	fmt.Println(b, c)
	var d = true
	fmt.Println(d)
	var e int
	fmt.Println(e)
	f := "short"
	fmt.Println(f)
	// Output:
	// initial
	// 1 2
	// true
	// 0
	// short
}

const s string = "constant"

func ExampleConstants() {
	const n = 50000000
	const d = 3e20 / n
	fmt.Println(s)
	fmt.Println(d)
	fmt.Println(int64(d))
	fmt.Println(math.Sin(n))
	// Output:
	// constant
	// 6e+12
	// 6000000000000
	// 0.8256467432733234
}

func ExampleFor() {
	j := 7
	for j <= 8 {
		fmt.Println(j)
		j = j + 1
	}

	for i := 1; i <= 2; i++ {
		fmt.Println(i)
	}

	for {
		fmt.Println("loop")
		break
	}
	// Output:
	// 7
	// 8
	// 1
	// 2
	// loop
}

func ExampleIfElse() {
	if 7%2 == 0 {
		fmt.Println("7 is even")
	} else {
		fmt.Println("7 is odd")
	}

	if num := 9; num < 0 {
		fmt.Println(num, "is negative")
	} else if num < 10 {
		fmt.Println(num, "has one digit")
	} else {
		fmt.Println(num, "has multiple digits")
	}
	// Output:
	// 7 is odd
	// 9 has one digit
}

func ExampleSwitch() {
	i := 2
	fmt.Print("Write ", i, " as ")
	switch i {
	case 1:
		fmt.Println("one")
	case 2:
		fmt.Println("two")
	case 3:
		fmt.Println("three")
	}

	switch time.Unix(1650000000, 11).Weekday() {
	case time.Saturday, time.Sunday:
		fmt.Println("It's the weekend")
	default:
		fmt.Println("It's a weekday")
	}

	t, _ := time.ParseDuration("20h")
	switch {
	case t.Hours() < 12:
		fmt.Println("It's before noon")
	default:
		fmt.Println("It's after noon")
	}

	// Output:
	// Write 2 as two
	// It's a weekday
	// It's after noon
}

func ExampleArrays() {
	var a [5]int
	fmt.Println("emp:", a)
	fmt.Println("len:", len(a))
	b := [5]int{1, 2, 3, 4, 5}
	fmt.Println("dcl:", b)

	var twod [2][3]int
	for i := 0; i < 2; i++ {
		for j := 0; j < 3; j++ {
			twod[i][j] = i + j
		}
	}
	fmt.Println("2d:", twod)

	// Output:
	// emp: [0 0 0 0 0]
	// len: 5
	// dcl: [1 2 3 4 5]
	// 2d: [[0 1 2] [1 2 3]]
}

func ExampleSlices() {
	s := make([]string, 3)
	fmt.Println("empty slices:", s)

	s[0], s[1], s[2] = "a", "b", "c"
	fmt.Println("after set:", s)
	fmt.Println("get element:", s[2])
	fmt.Println("slices len:", len(s))

	s = append(s, "d")
	s = append(s, "e", "f")
	fmt.Println("after append:", s)

	c := make([]string, len(s))
	copy(c, s)
	fmt.Println("copied slices:", c)

	fmt.Println("part [2:5] of slices:", s[2:5])
	fmt.Println("part [:5] of slices:", s[:5])
	fmt.Println("part [2:] of slices:", s[2:])

	t := []string{"g", "h", "i"}
	fmt.Println("declare slices:", t)

	twod := make([][]int, 3)
	for i := 0; i < 3; i++ {
		innerLen := i + 1
		twod[i] = make([]int, innerLen)
		for j := 0; j < innerLen; j++ {
			twod[i][j] = i + j
		}
	}
	fmt.Println("twod:", twod)

	// Output:
	// empty slices: [  ]
	// after set: [a b c]
	// get element: c
	// slices len: 3
	// after append: [a b c d e f]
	// copied slices: [a b c d e f]
	// part [2:5] of slices: [c d e]
	// part [:5] of slices: [a b c d e]
	// part [2:] of slices: [c d e f]
	// declare slices: [g h i]
	// twod: [[0] [1 2] [2 3 4]]
}

func ExampleMaps() {
	// make(map[key-type]val-type)
	m := make(map[string]int)
	m["k1"] = 7
	m["k2"] = 47
	fmt.Println("get element:", m["k1"])

	delete(m, "k2")
	fmt.Println("map:", m, "len:", len(m))
	_, isExist := m["k2"]
	fmt.Println("k2 exist:", isExist)

	n := map[string]int{"foo": 1, "bar": 2}
	delete(n, "bar")
	fmt.Println("new map:", n, "len:", len(n))

	// Output:
	// get element: 7
	// map: map[k1:7] len: 1
	// k2 exist: false
	// new map: map[foo:1] len: 1
}

func ExampleRange() {
	nums := []int{2, 3, 4}
	sum := 0
	for _, num := range nums {
		sum += num
	}
	fmt.Println("sum:", sum)

	for i, num := range nums {
		if num == 3 {
			fmt.Println("index of 3:", i)
		}
	}

	// print sorted map
	kvs := map[string]string{"a": "apple", "b": "banana"}
	keys := make([]string, 0, len(kvs))
	for k := range kvs {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	for _, k := range keys {
		fmt.Println(k, "->", kvs[k])
	}

	for i, c := range "go" {
		fmt.Println(i, c)
	}

	// Output:
	// sum: 9
	// index of 3: 1
	// a -> apple
	// b -> banana
	// 0 103
	// 1 111
}

func plus(a int, b int) int {
	return a + b
}

func plusPlus(a, b, c int) int {
	return a + b + c
}

func vals() (int, int) {
	return 3, 7
}

func sum(nums ...int) int {
	total := 0
	for _, num := range nums {
		total += num
	}
	return total
}

func ExampleFunctions() {
	fmt.Println("1+2 =", plus(1, 2))
	fmt.Println("1+2+3 =", plusPlus(1, 2, 3))
	a, b := vals()
	fmt.Println("vals():", a, b)
	fmt.Println("sum(1,2):", sum(1, 2))
	fmt.Println("sum(1,2,3):", sum(1, 2, 3))
	nums := []int{1, 2, 3, 4, 5}
	fmt.Println("sum of slice:", sum(nums...))
	// Output:
	// 1+2 = 3
	// 1+2+3 = 6
	// vals(): 3 7
	// sum(1,2): 3
	// sum(1,2,3): 6
	// sum of slice: 15
}

func intSeq() func() int {
	i := 0
	return func() int {
		i++
		return i
	}
}

func ExampleClosures() {
	nextInt := intSeq()
	fmt.Println(nextInt())
	fmt.Println(nextInt())
	fmt.Println(nextInt())

	newInt := intSeq()
	fmt.Println(newInt())

	// Output:
	// 1
	// 2
	// 3
	// 1
}

func fact(n int) int {
	if n == 0 {
		return 1
	}
	return n * fact(n-1)
}

func ExampleRecursion() {
	fmt.Println("fact(8):", fact(8))
	// Output:
	// fact(8): 40320
}

func zeroval(ival int) {
	ival = 0
	fmt.Println("inside zeroval:", ival)
}

func zeroptr(iptr *int) {
	*iptr = 0
	fmt.Println("inside zeroptr:", *iptr)
}

func ExamplePointers() {
	i := 5
	fmt.Println("init val:", i)
	zeroval(i)
	fmt.Println("zeroval:", i)
	zeroptr(&i)
	fmt.Println("zeroptr:", i)
	// Output:
	// init val: 5
	// inside zeroval: 0
	// zeroval: 5
	// inside zeroptr: 0
	// zeroptr: 0
}

type person struct {
	name string
	age  int
}

func ExampleStructs() {
	fmt.Println(person{"Bob", 20})
	fmt.Println(person{name: "Alice", age: 30})
	fmt.Println(person{name: "Fred"})
	fmt.Println(&person{name: "Ann", age: 40})
	s := person{name: "Sean", age: 50}
	fmt.Println(s.name)
	sp := &s
	fmt.Println(sp.age)
	sp.age = 51
	fmt.Println(sp.age)
	// Output:
	// {Bob 20}
	// {Alice 30}
	// {Fred 0}
	// &{Ann 40}
	// Sean
	// 50
	// 51
}

type rect struct {
	width, height float64
}

func (r rect) area() float64 {
	return r.width * r.height
}

func (r rect) perim() float64 {
	return 2*r.width + 2*r.height
}

func ExampleStructMethods() {
	r := rect{width: 10, height: 8}
	fmt.Println("area:", r.area())
	fmt.Println("perim:", r.perim())
	rp := &r
	fmt.Println("area:", rp.area())
	fmt.Println("perim:", rp.perim())
	// Output:
	// area: 80
	// perim: 36
	// area: 80
	// perim: 36
}

type geometry interface {
	area() float64
	perim() float64
}

type circle struct {
	radius float64
}

func (c circle) area() float64 {
	return math.Pi * c.radius * c.radius
}

func (c circle) perim() float64 {
	return 2 * math.Pi * c.radius
}

func measure(g geometry) {
	fmt.Println(g)
	fmt.Println("area:", g.area(), "perim:", g.perim())
}

func ExampleInterfaces() {
	r := rect{width: 8, height: 6}
	c := circle{radius: 10}
	measure(r)
	measure(c)
	// Output:
	// {8 6}
	// area: 48 perim: 28
	// {10}
	// area: 314.1592653589793 perim: 62.83185307179586
}

func errorF1(arg int) (int, error) {
	if arg == 42 {
		return -1, errors.New("can't work with 42")
	}
	return arg + 5, nil
}

type argError struct {
	arg  int
	prob string
}

func (e *argError) Error() string {
	return fmt.Sprint(e.arg, " - ", e.prob)
}

func errorF2(arg int) (int, error) {
	if arg == 42 {
		return -1, &argError{arg, "can't work with it"}
	}
	return arg + 5, nil
}

func ExampleError() {
	for _, i := range []int{7, 42} {
		if r, e := errorF1(i); e != nil {
			fmt.Println("errorF1 failed:", e)
		} else {
			fmt.Println("errorF1 worked:", r)
		}
	}

	for _, i := range []int{7, 42} {
		if r, e := errorF2(i); e != nil {
			fmt.Println("errorF2 failed:", e)
		} else {
			fmt.Println("errorF2 worked:", r)
		}
	}

	_, e := errorF2(42)
	if ae, ok := e.(*argError); ok {
		fmt.Println(ae.arg)
		fmt.Println(ae.prob)
	}

	// Output:
	// errorF1 worked: 12
	// errorF1 failed: can't work with 42
	// errorF2 worked: 12
	// errorF2 failed: 42 - can't work with it
	// 42
	// can't work with it
}
