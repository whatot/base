package example

import (
	"bufio"
	"crypto/sha1"
	"encoding/base64"
	"fmt"
	"io"
	"io/ioutil"
	"net"
	"net/url"
	"os"
)

func ExampleUrlParse() {
	s := "postgres://user:pass@host.com:5432/path?k=v#f"
	u, err := url.Parse(s)
	if err != nil {
		panic(err)
	}
	fmt.Println(u.Scheme)
	fmt.Println(u.User.Username())
	p, _ := u.User.Password()
	fmt.Println(p)
	host, port, _ := net.SplitHostPort(u.Host)
	fmt.Println(host)
	fmt.Println(port)
	fmt.Println(u.Path)
	fmt.Println(u.Fragment) // after the #
	fmt.Println(u.RawQuery)
	m, _ := url.ParseQuery(u.RawQuery)
	fmt.Println(m)
	fmt.Println(m["k"][0])
	// Output:
	// postgres
	// user
	// pass
	// host.com
	// 5432
	// /path
	// f
	// k=v
	// map[k:[v]]
	// v
}

func ExampleSha1() {
	s := "does not know it"
	h := sha1.New()
	_, _ = h.Write([]byte(s))
	bs := h.Sum(nil)
	fmt.Println(s)
	fmt.Printf("%x\n", bs)
	// Output:
	// does not know it
	// b6cadb9c219bb3bb3471c89bad7002a787e712fd
}

func ExampleBase64() {
	// StdEncoding and URLEncoding, refer to (trailing + vs -)
	data := "abc123!?$*&()'-=@~"
	sEnc := base64.StdEncoding.EncodeToString([]byte(data))
	fmt.Println(sEnc)
	sDec, _ := base64.StdEncoding.DecodeString(sEnc)
	fmt.Println(string(sDec))
	uEnc := base64.URLEncoding.EncodeToString([]byte(data))
	fmt.Println(uEnc)
	uDec, _ := base64.URLEncoding.DecodeString(uEnc)
	fmt.Println(string(uDec))
	// Output:
	// YWJjMTIzIT8kKiYoKSctPUB+
	// abc123!?$*&()'-=@~
	// YWJjMTIzIT8kKiYoKSctPUB-
	// abc123!?$*&()'-=@~
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func ExampleReadWriteFile() {
	fName := "/tmp/changing-data.txt"
	check(ioutil.WriteFile(fName, []byte("hello\n"), 0644))
	data, err := ioutil.ReadFile(fName)
	check(err)
	fmt.Print(string(data))
	check(os.Truncate(fName, 0))
	check(os.Remove(fName))

	f, err := os.Create(fName)
	check(err)

	// write some thing and sync
	w, err := f.Write([]byte{115, 111, 109, 101, 10})
	check(err)
	fmt.Println("write", w, "bytes")
	w, err = f.WriteString("some string\n")
	check(err)
	fmt.Println("write", w, "bytes")
	check(f.Sync())

	bw := bufio.NewWriter(f)
	w, err = bw.WriteString("buffered string")
	check(err)
	fmt.Println("buffered write", w, "bytes")
	check(bw.Flush())

	// rewind
	_, err = f.Seek(0, 0)
	check(err)

	b := make([]byte, 5)
	r1, err := f.Read(b)
	check(err)
	fmt.Println("read", r1, "bytes")

	p, err := f.Seek(int64(r1), 0)
	check(err)
	b2 := make([]byte, 12)
	r2, err := f.Read(b2)
	check(err)
	fmt.Println("read", r2, "bytes @", p, string(b2))

	b3 := make([]byte, 15)
	r3, err := io.ReadAtLeast(f, b3, 10)
	check(err)
	fmt.Println("read", r3, "bytes @", p, string(b3))

	// rewind
	_, err = f.Seek(0, 0)
	check(err)

	r4 := bufio.NewReader(f)
	b4, err := r4.Peek(10)
	check(err)
	fmt.Println("10 bytes:", string(b4))

	check(f.Close())

	// Output:
	// hello
	// write 5 bytes
	// write 12 bytes
	// buffered write 15 bytes
	// read 5 bytes
	// read 12 bytes @ 5 some string

	// read 15 bytes @ 5 buffered string
	// 10 bytes: some
	// some
}
