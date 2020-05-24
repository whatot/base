package main

import (
	"fmt"
	"io"
	"math"
	"os"
	"strings"
)

type Vertex struct {
	X, Y float64
}

// 接收者的类型定义和方法声明必须在同一包内；不能为内建类型声明方法
func (v Vertex) Abs() float64 {
	return math.Sqrt(v.X*v.X + v.Y*v.Y)
}

// use pointer to update origin data
func (v *Vertex) Scale(f float64) {
	v.X = v.X * f
	v.Y = v.Y * f
}

func (v Vertex) String() string {
	return fmt.Sprintf("Vertex(X:%f, Y:%f)", v.X, v.Y)
}

type I interface {
	M()
}

type T struct {
	S string
}

func (t *T) M() {
	if t == nil {
		fmt.Println("<nil>")
		return
	}
	fmt.Println(t.S)
}

type F float64

func (f F) M() {
	fmt.Println(f)
}

func describe(i interface{}) {
	fmt.Printf("(%v, %T)\n", i, i)
}

func basic_func() {
	v := Vertex{X: 3, Y: 4}
	fmt.Println(v.Abs())

	// pointer redirected => (&v).Scale(10)
	v.Scale(10)
	fmt.Println(v.Abs())
}

func interface_about() {
	var i I = &T{"hello"}
	describe(i)
	i.M()

	// need check nil
	var t *T
	i = t
	describe(i)
	i.M()

	i = F(math.Pi)
	describe(i)
	i.M()

}

func dynamic_interface() {
	var i interface{} = "string"

	s, ok := i.(string)
	fmt.Println("value:", s, "is_ok?", ok)

	// panic if only f
	f, ok := i.(float64)
	fmt.Println("value:", f, "is_ok?", ok)
}

func do(i interface{}) {
	switch v := i.(type) {
	case int:
		fmt.Printf("Twice %v is %v\n", v, v*2)
	case string:
		fmt.Printf("%q is %v bytes long\n", v, len(v))
	default:
		fmt.Printf("I don't know about type %T!\n", v)
	}
}

func type_switch() {
	do(42)
	do("hello")
	do(false)
}

type IPAddr [4]byte

func (i IPAddr) String() string {
	return fmt.Sprintf("%d.%d.%d.%d", i[0], i[1], i[2], i[3])
}

func stringer() {
	v := Vertex{X: 3, Y: 4}
	fmt.Println(v)

	hosts := map[string]IPAddr{
		"google":   {8, 8, 8, 8},
		"loopback": {127, 0, 0, 1},
	}
	for name, ip := range hosts {
		fmt.Println(name, ip)
	}
}

type ErrNegativeSqrt float64

func (e ErrNegativeSqrt) Error() string {
	return fmt.Sprint("cannot Sqrt negative number:", float64(e))
}

func Sqrt(x float64) (float64, error) {
	if x < 0 {
		return 0, ErrNegativeSqrt(x)
	}
	return math.Sqrt(x), nil
}

func error_about() {
	fmt.Println(Sqrt(2))
	fmt.Println(Sqrt(-2))
}

func reader() {
	r := strings.NewReader("Hello, Reader!")

	b := make([]byte, 8)
	for {
		n, err := r.Read(b)
		fmt.Printf("n=%d, err=%v, b=%v, b[:n]=%q\n", n, err, b, b[:n])
		if err == io.EOF {
			break
		}
	}
}

type rot13Reader struct {
	r io.Reader
}

func rot13(b byte) byte {
	switch {
	case 'A' <= b && b <= 'M':
		b = b + 13
	case 'M' < b && b <= 'Z':
		b = b - 13
	case 'a' <= b && b <= 'm':
		b = b + 13
	case 'm' < b && b <= 'z':
		b = b - 13
	}
	return b
}

func (nr rot13Reader) Read(b []byte) (int, error) {
	n, e := nr.r.Read(b)
	for i := 0; i < n; i++ {
		b[i] = rot13(b[i])
	}
	return n, e
}

func rot13_about() {
	s := strings.NewReader("Lbh penpxrq gur pbqr!")
	r := rot13Reader{s}
	// You cracked the code!
	io.Copy(os.Stdout, r)
}

func main() {
	basic_func()
	interface_about()
	dynamic_interface()
	type_switch()
	stringer()
	error_about()
	reader()
	rot13_about()
}
