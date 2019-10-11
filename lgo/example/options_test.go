package example

// http://dave.cheney.net/2014/10/17/functional-options-for-friendly-apis

import (
	"fmt"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
)

type Server struct {
	x       int
	y       int
	timeout time.Duration
	retry   bool
}

func NewServer() *Server {
	return &Server{}
}

func (s *Server) WithXY(x int, y int) *Server {
	s.x = x
	s.y = y
	return s
}

func (s *Server) WithTimeOut(t time.Duration) *Server {
	s.timeout = t
	return s
}

func (s *Server) WithRetry(b bool) *Server {
	s.retry = b
	return s
}

func (s *Server) Summary() string {
	return fmt.Sprint(s.x, s.y, s.timeout, s.retry)
}

func TestOptionChaining(t *testing.T) {
	// It is not suitable for error-handle.
	s := NewServer().WithXY(10, 100).WithTimeOut(time.Second).WithRetry(true)

	expect := "10 100 1s true"
	assert.Equal(t, expect, s.Summary())
}

func ExampleOptionChaining() {
	s := NewServer().
		WithXY(10, 100).
		WithTimeOut(time.Second).
		WithRetry(true)
	fmt.Println(s)
	// Output:
	// &{10 100 1000000000 true}
}

func NewServerPlus(options ...func(*Server) error) (*Server, error) {
	s := Server{}

	for _, option := range options {
		err := option(&s)
		// handle err, cleanup
		if err != nil {
			fmt.Println(err)
		}
	}
	return &s, nil
}

func HaveRetry(s *Server) error {
	s.retry = true
	return nil
}

func SetXY(x int, y int) func(*Server) error {
	return func(s *Server) error {
		s.x = x
		s.y = y
		return nil
	}
}

func SetTimeout(t time.Duration) func(*Server) error {
	return func(s *Server) error {
		s.timeout = t
		return nil
	}
}

func TestOptionClosure(t *testing.T) {
	// the nicer way to deal with options
	s, _ := NewServerPlus(HaveRetry, SetXY(1, 2), SetTimeout(time.Second))

	expect := "1 2 1s true"
	assert.Equal(t, expect, s.Summary())
}
