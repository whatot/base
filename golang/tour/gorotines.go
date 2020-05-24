package main

import (
	"fmt"
	"math/rand"
	"time"
)

func say(s string) {
	for i := 0; i < 3; i++ {
		time.Sleep(100 * time.Millisecond)
		fmt.Println(s)
	}
}

func sum(s []int, c chan int) {
	sum := 0
	for i := 0; i < len(s); i++ {
		sum += s[i]
	}
	c <- sum
}

func sum_with_channel() {
	s := []int{7, 2, 7, 8, 9, 10, 2, 4, 10}
	c := make(chan int)
	go sum(s[:len(s)/2], c)
	go sum(s[len(s)/2:], c)
	x, y := <-c, <-c
	fmt.Println(x, y, x+y)
}

func fibonacci_v1(n int, c chan int) {
	x, y := 0, 1
	for i := 0; i < n; i++ {
		c <- x
		x, y = y, x+y
	}
	// only sender can close channel
	// send data to a closed channel will panic
	close(c)
}

func fibonacci_with_one_chan() {
	c := make(chan int, 20)
	go fibonacci_v1(cap(c), c)
	fmt.Printf("fibonacci1(%d) =>", cap(c))
	for i := range c {
		fmt.Printf(" %d", i)
	}
	fmt.Println()
}

func fibonacci_v2(c, quit chan int) {
	x, y := 0, 1
	for {
		select {
		case c <- x:
			x, y = y, x+y
		case <-quit:
			fmt.Println(" quit")
			return
		}
	}
}

func fibonacci_with_select_chan() {
	c := make(chan int)
	quit := make(chan int)

	limit := 15
	fmt.Printf("fibonacci2(%d) =>", limit)
	go func() {
		for i := 0; i < limit; i++ {
			fmt.Printf(" %d", <-c)
		}
		quit <- 0
	}()
	fibonacci_v2(c, quit)
}

func tick_boom() {
	tick := time.Tick(50 * time.Millisecond)
	boom := time.After(500 * time.Millisecond)

	for {
		select {
		case <-tick:
			fmt.Printf("tick.")
		case <-boom:
			fmt.Println("boom.")
			return
		default:
			fmt.Printf(".")
			time.Sleep(30 * time.Millisecond)
		}
	}
}

type Tree struct {
	Left  *Tree
	Value int
	Right *Tree
}

func New(k int) *Tree {
	var t *Tree
	for _, v := range rand.Perm(15) {
		t = insert(t, (1+v)*k)
	}
	return t
}

func insert(t *Tree, v int) *Tree {
	if t == nil {
		return &Tree{nil, v, nil}
	}
	if v < t.Value {
		t.Left = insert(t.Left, v)
	} else {
		t.Right = insert(t.Right, v)
	}
	return t
}

func (t *Tree) String() string {
	if t == nil {
		return "()"
	}
	s := ""
	if t.Left != nil {
		s += t.Left.String() + " "
	}
	s += fmt.Sprint(t.Value)
	if t.Right != nil {
		s += " " + t.Right.String()
	}
	return "(" + s + ")"
}

// walk tree, send value into ch
func Walk(t *Tree, ch chan int) {
	walkRecurse(t, ch)
	// use close to indicate walk finished
	close(ch)
}

func walkRecurse(t *Tree, ch chan int) {
	if t == nil {
		return
	}
	if t.Left != nil {
		walkRecurse(t.Left, ch)
	}
	ch <- t.Value
	if t.Right != nil {
		walkRecurse(t.Right, ch)
	}
}

// check two tree have same values
func Same(t1, t2 *Tree) bool {
	c1 := make(chan int, 10)
	go Walk(t1, c1)

	c2 := make(chan int, 10)
	go Walk(t2, c2)

	for {
		v1, ok1 := <-c1
		v2, ok2 := <-c2

		// fmt.Printf("v1:%d, ok1:%v, v2:%d, ok2:%v\n", v1, ok1, v2, ok2)
		// all read normally
		if ok1 && ok2 {
			if v1 != v2 {
				return false
			}
		}

		// all closed
		if !ok1 && !ok2 {
			return true
		}

		// anyone closed first
		if (ok1 && !ok2) || (!ok1 && ok2) {
			return false
		}
	}
}

func check_same_tree() {
	fmt.Println(New(1))
	fmt.Println(Same(New(1), New(2)))
	fmt.Println(Same(New(1), New(1)))
}

func main() {
	go say("world")
	say("hello")

	sum_with_channel()
	fibonacci_with_one_chan()
	fibonacci_with_select_chan()
	tick_boom()
	check_same_tree()
}
