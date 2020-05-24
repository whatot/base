package main

import (
	"fmt"
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

func main() {
	go say("world")
	say("hello")

	sum_with_channel()
	fibonacci_with_one_chan()
	fibonacci_with_select_chan()
	tick_boom()
}
