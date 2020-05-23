package main

import (
	"fmt"
	"runtime"
	"time"
)

func main() {
	sum := 0
	for i := 0; i < 11; i++ {
		sum += i
	}
	fmt.Println("test for ", sum)

	in := float64(110)
	s := in / 2
	for i := 0; i < 5; i++ {
		s = s - (s*s-in)/(2*s)
		fmt.Println(i, s)
	}

	switch os := runtime.GOOS; os {
	case "linux":
		fmt.Println("linux")
	default:
		fmt.Println("os: ", os)
	}

	switch {
	case time.Now().Hour() < 12:
		fmt.Println("Good morning")
	case time.Now().Hour() < 17:
		fmt.Println("Good afternoon")
	default:
		fmt.Println("Good evening")
	}

	// LIFO Last-In, First-Out
	defer fmt.Println("world")
	defer fmt.Println("1")
	defer fmt.Println("2")
	defer fmt.Println("3")
	fmt.Println("hello")
}
