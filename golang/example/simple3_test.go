package example

import (
	"bytes"
	"encoding/json"
	"fmt"
	"math/rand"
	"os"
	"reflect"
	"regexp"
	"sort"
	"strconv"
	"strings"
	"testing"
	"time"
)

func ExampleSorting() {
	strs := []string{"c", "a", "b"}
	sort.Strings(strs)
	fmt.Println("Strings:", strs)

	ints := []int{7, 2, 4}
	sort.Ints(ints)
	fmt.Println("Ints:", ints)

	fmt.Println("Sorted:", sort.IntsAreSorted(ints))
	// Output:
	// Strings: [a b c]
	// Ints: [2 4 7]
	// Sorted: true
}

type ByLength []string

func (s ByLength) Len() int {
	return len(s)
}

func (s ByLength) Swap(i, j int) {
	s[i], s[j] = s[j], s[i]
}

func (s ByLength) Less(i, j int) bool {
	return len(s[i]) < len(s[j])
}

func ExampleSortByFunctions() {
	fruits := []string{"peach", "banana", "kiwi"}
	sort.Sort(ByLength(fruits))
	fmt.Println(fruits)
	// Output:
	// [kiwi peach banana]
}

func createFile(s string) *os.File {
	fmt.Println("creating")
	f, err := os.Create(s)
	if err != nil {
		panic(err)
	}
	return f
}

func writeFile(f *os.File) {
	fmt.Println("writing")
	fmt.Fprintln(f, "some data")
}

func closeFile(f *os.File) {
	fmt.Println("closing")
	if err := f.Close(); err != nil {
		panic(err)
	}
}

func ExampleDefer() {
	f := createFile("/tmp/defer.txt")
	defer closeFile(f)
	writeFile(f)
	// Output:
	// creating
	// writing
	// closing
}

func Index(vs []string, t string) int {
	for i, v := range vs {
		if v == t {
			return i
		}
	}
	return -1
}

func IsInclude(vs []string, t string) bool {
	return Index(vs, t) >= 0
}

func Any(vs []string, f func(string) bool) bool {
	for _, v := range vs {
		if f(v) {
			return true
		}
	}
	return false
}

func All(vs []string, f func(string) bool) bool {
	for _, v := range vs {
		if !f(v) {
			return false
		}
	}
	return true
}

func Filter(vs []string, f func(string) bool) []string {
	var vsf []string
	for _, v := range vs {
		if f(v) {
			vsf = append(vsf, v)
		}
	}
	return vsf
}

func Map(vs []string, f func(string) string) []string {
	vsm := make([]string, len(vs))
	for i, v := range vs {
		vsm[i] = f(v)
	}
	return vsm
}

func ExampleCollectionFunctions() {
	var strs = []string{"peach", "apple", "pear", "plum"}
	fmt.Println(Index(strs, "pear"))
	fmt.Println(IsInclude(strs, "black"))
	fmt.Println(Any(strs, func(v string) bool {
		return strings.HasPrefix(v, "p")
	}))
	fmt.Println(All(strs, func(v string) bool {
		return strings.HasSuffix(v, "h")
	}))
	fmt.Println(Filter(strs, func(v string) bool {
		return strings.Contains(v, "e")
	}))
	fmt.Println(Map(strs, strings.ToUpper))

	// Output:
	// 2
	// false
	// true
	// false
	// [peach apple pear]
	// [PEACH APPLE PEAR PLUM]
}

func TestStringsFunctions(t *testing.T) {
	cases := []struct {
		actual string
		expect string
	}{
		{fmt.Sprint(strings.Contains("test", "t")), "true"},
		{fmt.Sprint(strings.Count("test", "t")), "2"},
		{fmt.Sprint(strings.HasPrefix("test", "te")), "true"},
		{fmt.Sprint(strings.HasSuffix("test", "st")), "true"},
		{fmt.Sprint(strings.Index("test", "e")), "1"},
		{strings.Join([]string{"a", "b"}, "-"), "a-b"},
		{strings.Repeat("a", 5), "aaaaa"},
		{strings.Replace("foo", "o", "0", -1), "f00"},
		{strings.Replace("foo", "o", "0", 1), "f0o"},
		{fmt.Sprint(strings.Split("a-b-c-d-e", "-")), "[a b c d e]"},
		{strings.ToLower("TEST"), "test"},
		{strings.ToUpper("test"), "TEST"},
		{fmt.Sprint(len("hello")), "5"},
		{fmt.Sprint("hello"[1]), "101"},
	}

	for i, tc := range cases {
		if tc.actual != tc.expect {
			t.Fatalf("%d: %#v %#v", i, tc.actual, tc.expect)
		}
	}
}

type point struct {
	x, y int
}

func TestStringFormat(t *testing.T) {
	p := point{1, 2}
	cases := []struct {
		actual string
		expect string
	}{
		{fmt.Sprintf("%v", p), "{1 2}"},
		{fmt.Sprintf("%+v", p), "{x:1 y:2}"},
		{fmt.Sprintf("%#v", p), "example.point{x:1, y:2}"},
		{fmt.Sprintf("%T", p), "example.point"},
		{fmt.Sprintf("%t", true), "true"},
		{fmt.Sprintf("%d", 123), "123"},
		{fmt.Sprintf("%b", 14), "1110"},
		{fmt.Sprintf("%c", 33), "!"},
		{fmt.Sprintf("%x", 456), "1c8"},
		{fmt.Sprintf("%f", 78.9), "78.900000"},
		{fmt.Sprintf("%e", 12340000.0), "1.234000e+07"},
		{fmt.Sprintf("%E", 12340000.0), "1.234000E+07"},
		{fmt.Sprintf("%s", "string"), "string"},
		{fmt.Sprintf("%q", "string"), "\"string\""},
		{fmt.Sprintf("%x", "hex this"), "6865782074686973"},
		{fmt.Sprintf("|%6d|%6d|", 12, 345), "|    12|   345|"},
		{fmt.Sprintf("|%6.2f|%6.2f|", 1.2, 3.45), "|  1.20|  3.45|"},
		{fmt.Sprintf("|%-6.2f|%-6.2f|", 1.2, 3.45), "|1.20  |3.45  |"},
		{fmt.Sprintf("|%6s|%6s|", "foo", "bb"), "|   foo|    bb|"},
		{fmt.Sprintf("|%-6s|%-6s|", "foo", "bb"), "|foo   |bb    |"},
	}

	for i, tc := range cases {
		if tc.actual != tc.expect {
			t.Fatalf("%d: %#v %#v", i, tc.actual, tc.expect)
		}
	}
}

func sp(i ...interface{}) string {
	return fmt.Sprint(i...)
}

func TestRegularExpressions(t *testing.T) {
	r, _ := regexp.Compile("p([a-z]+)ch")
	cases := []struct {
		actual string
		expect string
	}{
		{sp(regexp.MatchString("p([a-z]+)ch", "peach")), "true <nil>"},
		{sp(r.MatchString("peach")), "true"},
		{sp(r.FindString("peach punch")), "peach"},
		{sp(r.FindStringIndex("peach punch")), "[0 5]"},
		{sp(r.FindStringSubmatch("peach punch")), "[peach ea]"},
		{sp(r.FindStringSubmatchIndex("peach punch")), "[0 5 1 3]"},
		{sp(r.FindAllString("peach punch pinch", -1)),
			"[peach punch pinch]"},
		{sp(r.FindAllStringSubmatchIndex("peach punch pinch", -1)),
			"[[0 5 1 3] [6 11 7 9] [12 17 13 15]]"},
		{sp(r.FindAllString("peach punch pinch", 1)), "[peach]"},
		{sp(r.Match([]byte("peach"))), "true"},
		{sp(r.ReplaceAllString("a peach", "<fruit>")), "a <fruit>"},
		{string(r.ReplaceAllFunc([]byte("a peach"), bytes.ToUpper)),
			"a PEACH"},
	}

	for i, tc := range cases {
		if tc.actual != tc.expect {
			t.Fatalf("%d: %#v %#v", i, tc.actual, tc.expect)
		}
	}
}

type Response1 struct {
	Page   int
	Fruits []string
}

type Response2 struct {
	// the third column is used for json-tag
	Page   int      `json:"page"`
	Fruits []string `json:"fruits"`
}

func ms(v interface{}) string {
	res, err := json.Marshal(v)
	if err != nil {
		return err.Error()
	}
	return string(res)
}

func TestJsonMarshal(t *testing.T) {
	cases := []struct {
		actual string
		expect string
	}{
		{ms(true), "true"},
		{ms(1), "1"},
		{ms(1.23), "1.23"},
		{ms("gopher"), "\"gopher\""},
		{ms([]string{"apple", "peach", "pear"}),
			"[\"apple\",\"peach\",\"pear\"]"},
		{ms(map[string]int{"apple": 5, "lettuce": 7}),
			"{\"apple\":5,\"lettuce\":7}"},
		{ms(Response1{Page: 1, Fruits: []string{"apple", "peach", "pear"}}),
			"{\"Page\":1,\"Fruits\":[\"apple\",\"peach\",\"pear\"]}"},
		{ms(Response2{Page: 1, Fruits: []string{"apple", "peach", "pear"}}),
			"{\"page\":1,\"fruits\":[\"apple\",\"peach\",\"pear\"]}"},
	}

	for i, tc := range cases {
		if tc.actual != tc.expect {
			t.Fatalf("%d: %#v %#v", i, tc.actual, tc.expect)
		}
	}
}

func ums(t *testing.T, data []byte, real interface{}, want interface{}) {
	if err := json.Unmarshal(data, &real); err != nil {
		t.Fatal(err)
	}
	if eq := reflect.DeepEqual(real, want); !eq {
		t.Fatalf("Not equal: %#v %#v", real, want)
	}
}

func TestJsonUmarshal(t *testing.T) {
	byt := []byte(`{"num":6.13,"strs":["a","b"]}`)
	var bytReal map[string]interface{}
	bytWant := map[string]interface{}{
		"num":  6.13,
		"strs": []interface{}{"a", "b"}}
	ums(t, byt, bytReal, bytWant)
}

func TestTime(t *testing.T) {
	now := time.Date(2016, 10, 30, 15, 5, 20, 651387237, time.UTC)
	then := time.Date(2017, 6, 7, 8, 9, 10, 6521, time.UTC)
	diff := then.Sub(now)
	cases := []struct {
		actual string
		expect string
	}{
		{sp(now), "2016-10-30 15:05:20.651387237 +0000 UTC"},
		{sp(then), "2017-06-07 08:09:10.000006521 +0000 UTC"},
		{sp(diff), "5273h3m49.348619284s"},
		{sp(then.Year()), "2017"},
		{sp(then.Month()), "June"},
		{sp(then.Day()), "7"},
		{sp(then.Hour()), "8"},
		{sp(then.Minute()), "9"},
		{sp(then.Second()), "10"},
		{sp(then.Nanosecond()), "6521"},
		{sp(then.Location()), "UTC"},
		{sp(then.Weekday()), "Wednesday"},
		{sp(then.Before(now)), "false"},
		{sp(then.After(now)), "true"},
		{sp(then.Equal(now)), "false"},
		{sp(diff.Hours()), "5273.063707949801"},
		{sp(diff.Minutes()), "316383.8224769881"},
		{sp(diff.Seconds()), "1.8983029348619282e+07"},
		{sp(diff.Nanoseconds()), "18983029348619284"},
		{sp(then.Add(diff)), "2018-01-13 01:12:59.348625805 +0000 UTC"},
		{sp(then.Add(-diff)), "2016-10-30 15:05:20.651387237 +0000 UTC"},
	}

	for i, tc := range cases {
		if tc.actual != tc.expect {
			t.Fatalf("%d: %#v %#v", i, tc.actual, tc.expect)
		}
	}
}

func ExampleEpoch() {
	now := time.Date(2016, 10, 30, 15, 5, 20, 651387237, time.UTC)
	secs := now.Unix()
	nanos := now.UnixNano()
	mills := nanos / 1000000
	fmt.Println(now)
	fmt.Println(secs)
	fmt.Println(mills)
	fmt.Println(nanos)
	fmt.Println(time.Unix(secs, 0).UTC())
	fmt.Println(time.Unix(0, nanos).UTC())

	// Output:
	// 2016-10-30 15:05:20.651387237 +0000 UTC
	// 1477839920
	// 1477839920651
	// 1477839920651387237
	// 2016-10-30 15:05:20 +0000 UTC
	// 2016-10-30 15:05:20.651387237 +0000 UTC
}

func ExampleTimeFormatParse() {
	t := time.Date(2016, 10, 30, 15, 5, 20, 651387237, time.UTC)
	fmt.Println(t.Format(time.RFC3339))
	t1, _ := time.Parse(time.RFC3339, "2012-11-01T22:08:41+00:00")
	fmt.Println(t1)
	fmt.Println(t.Format("3:04PM"))
	fmt.Println("Mon Jan _2 15:04:05 2006")
	fmt.Println("2006-01-02T15:04:05.999999-07:00")
	t2, _ := time.Parse("3 04 PM", "8 41 PM")
	fmt.Println(t2)
	fmt.Printf("%d-%02d-%02dT%02d:%02d:%02d-00:00\n",
		t.Year(), t.Month(), t.Day(),
		t.Hour(), t.Minute(), t.Second())
	_, err := time.Parse("Mon Jan _2 15:04:05 2006", "8:41PM")
	fmt.Println(err)
	// Output:
	// 2016-10-30T15:05:20Z
	// 2012-11-01 22:08:41 +0000 +0000
	// 3:05PM
	// Mon Jan _2 15:04:05 2006
	// 2006-01-02T15:04:05.999999-07:00
	// 0000-01-01 20:41:00 +0000 UTC
	// 2016-10-30T15:05:20-00:00
	// parsing time "8:41PM" as "Mon Jan _2 15:04:05 2006": cannot parse "8:41PM" as "Mon"
}

func isInIntRange(data int, start int, end int) bool {
	return data >= start && data < end
}

func isInFloatRange(data float64, start float64, end float64) bool {
	return data >= start && data < end
}

func TestRandomNumbers(t *testing.T) {
	if i := rand.Intn(100); !isInIntRange(i, 0, 100) {
		t.Fatalf("Error when rand.Intn(100), %v", i)
	}
	if i := rand.Float64(); !isInFloatRange(i, 0.0, 1.0) {
		t.Fatalf("Error when rand.Float64(), %v", i)
	}
	if i := rand.Float64()*5 + 5; !isInFloatRange(i, 5.0, 10.0) {
		t.Fatalf("Error when rand.Float64()*5+5, %v", i)
	}

	// more random numbers, or use cryto/rand for secret
	s1 := rand.NewSource(time.Now().UnixNano())
	r1 := rand.New(s1)
	if i := r1.Intn(100); !isInIntRange(i, 0, 100) {
		t.Fatalf("Error when r1.Intn(100), %v", i)
	}

	// random seed, produces the same sequence of random numbers
	s2 := rand.NewSource(42)
	r2 := rand.New(s2)
	real := []int{r2.Intn(100), r2.Intn(100), r2.Intn(100), r2.Intn(100)}
	want := []int{5, 87, 68, 50}
	for i := 0; i < len(want); i++ {
		if real[i] != want[i] {
			t.Fatalf("Error when check rand seed, %#v %#v", real, want)
		}
	}
}

func ExampleNumberParse() {
	f, _ := strconv.ParseFloat("1.234", 64)
	fmt.Println(f)
	i, _ := strconv.ParseInt("123", 0, 64)
	fmt.Println(i)
	d, _ := strconv.ParseInt("0x1c88", 0, 64)
	fmt.Println(d)
	u, _ := strconv.ParseUint("789", 0, 64)
	fmt.Println(u)
	k, _ := strconv.Atoi("124")
	fmt.Println(k)
	_, e := strconv.Atoi("what")
	fmt.Println(e)
	// Output:
	// 1.234
	// 123
	// 7304
	// 789
	// 124
	// strconv.Atoi: parsing "what": invalid syntax
}
