package example

import (
	"fmt"
	"math/rand"
	"runtime"
	"sync"
	"sync/atomic"
	"testing"
	"time"
)

func ExampleChannels() {
	// make a channel, make(chan val-type)
	messages := make(chan string)
	go func() { messages <- "ping" }()
	msg := <-messages
	fmt.Println(msg)
	// Output:
	// ping
}

func ExampleChannelBuffering() {
	messages := make(chan string, 2)
	messages <- "buffered"
	messages <- "channel"
	fmt.Println(<-messages)
	fmt.Println(<-messages)
	// Output:
	// buffered
	// channel
}

func simpleWorker(done chan bool) {
	fmt.Print("working...")
	time.Sleep(time.Millisecond)
	fmt.Println("done")
	done <- true
}

func ExampleChannelSync() {
	done := make(chan bool, 1)
	go simpleWorker(done)
	<-done
	// Output:
	// working...done
}

func ping(pings chan<- string, msg string) {
	pings <- msg
}

func pong(pings <-chan string, pongs chan<- string) {
	msg := <-pings
	pongs <- msg
}

func ExampleChannelDirection() {
	pings := make(chan string, 1)
	pongs := make(chan string, 1)
	ping(pings, "process message")
	pong(pings, pongs)
	fmt.Println(<-pongs)
	// Output:
	// process message
}

func TestSelect(t *testing.T) {
	c1 := make(chan string)
	c2 := make(chan string)

	go func() {
		time.Sleep(time.Millisecond * 1)
		c1 <- "one"
	}()
	go func() {
		time.Sleep(time.Millisecond * 2)
		c2 <- "two"
	}()

	for i := 0; i < 2; i++ {
		select {
		case msg1 := <-c1:
			fmt.Println("receive:", msg1)
		case msg2 := <-c2:
			fmt.Println("receive:", msg2)
		}
	}
	// Output is out-of-order:
	// receive: one
	// receive: two
}

func sleepOut(t time.Duration, msg string, ch chan<- string) {
	time.Sleep(t)
	ch <- msg
}

func ExampleTimeouts() {
	c1 := make(chan string, 1)
	go sleepOut(time.Millisecond*3, "result c1", c1)
	select {
	case res := <-c1:
		fmt.Println(res)
	case <-time.After(time.Millisecond * 1):
		// timeout < sleep, so output timeout.
		fmt.Println("timeout 1")
	}

	c2 := make(chan string, 1)
	go sleepOut(time.Millisecond*1, "result c2", c2)
	select {
	case res := <-c2:
		fmt.Println(res)
	case <-time.After(time.Millisecond * 3):
		// timeout > sleep, so output result.
		fmt.Println("timeout 3")
	}
	// Output:
	// timeout 1
	// result c2
}

func ExampleChannelNonBlocking() {
	messages := make(chan string)
	signals := make(chan bool)
	select {
	case msg := <-messages:
		fmt.Println("received message", msg)
	default:
		fmt.Println("no message received")
	}

	msg := "hi"
	select {
	case messages <- msg:
		fmt.Println("sent message", msg)
	default:
		fmt.Println("no message sent")
	}

	// multi-way non-blocking select
	select {
	case msg := <-messages:
		fmt.Println("received message", msg)
	case sig := <-signals:
		fmt.Println("received signal", sig)
	default:
		fmt.Println("no activity")
	}
	// Output:
	// no message received
	// no message sent
	// no activity
}

func ExampleChannelClose() {
	jobs := make(chan int, 5)
	done := make(chan bool)

	go func() {
		for {
			if j, more := <-jobs; more {
				fmt.Println("received job", j)
			} else {
				fmt.Println("received all jobs")
				done <- true
				return
			}
		}
	}()

	for j := 1; j <= 3; j++ {
		jobs <- j
		fmt.Println("sent job", j)
	}
	close(jobs)
	fmt.Println("sent all jobs")

	// blocking until a message
	<-done

	// Output:
	// sent job 1
	// sent job 2
	// sent job 3
	// sent all jobs
	// received job 1
	// received job 2
	// received job 3
	// received all jobs
}

func ExampleChannelRange() {
	queue := make(chan string, 2)
	queue <- "one"
	queue <- "two"
	close(queue)

	// if without `close` above, `for` will block and wait for next elem
	for elem := range queue {
		fmt.Println(elem)
	}

	// Output:
	// one
	// two
}

func ExampleTimers() {
	timer1 := time.NewTimer(time.Millisecond * 2)
	// block until it expired
	<-timer1.C
	fmt.Println("timer1 expired")

	timer2 := time.NewTimer(time.Millisecond * 2)
	go func() {
		<-timer2.C
		fmt.Println("timer2 expired")
	}()
	if stop2 := timer2.Stop(); stop2 {
		fmt.Println("timer2 stopped")
	}
	// Output:
	// timer1 expired
	// timer2 stopped
}

func getMs(start time.Time, end time.Time) int64 {
	return end.Sub(start).Nanoseconds() / int64(time.Millisecond)
}

func ExampleTickers() {
	ticker := time.NewTicker(time.Millisecond * 2)
	start := time.Now()
	go func() {
		for t := range ticker.C {
			fmt.Println("Tick after", getMs(start, t), "ms")
		}
	}()
	time.Sleep(time.Millisecond * 9)
	ticker.Stop()
	fmt.Println("Ticker stopped")
	// Output:
	// Tick after 2 ms
	// Tick after 4 ms
	// Tick after 6 ms
	// Tick after 8 ms
	// Ticker stopped
}

func poolWorker(_ int, jobs <-chan int, results chan<- int) {
	for j := range jobs {
		fmt.Println("worker", "<id>", "processing job", "<j>")
		time.Sleep(time.Millisecond * 2)
		results <- j * 2
	}
}

func ExampleWorkerPool() {
	jobs := make(chan int, 100)
	results := make(chan int, 100)

	for w := 1; w < 3; w++ {
		go poolWorker(w, jobs, results)
	}
	for j := 1; j < 4; j++ {
		jobs <- j
	}
	close(jobs)
	for j := 1; j < 4; j++ {
		<-results
	}

	// Output:
	// worker <id> processing job <j>
	// worker <id> processing job <j>
	// worker <id> processing job <j>
}

func TestRateLimit(t *testing.T) {
	count := 5
	requests := make(chan int, count)
	for i := 1; i <= count; i++ {
		requests <- i
	}
	close(requests)

	limiter := time.Tick(time.Millisecond * 10)
	start := time.Now()
	// unused elements in requests temporarily
	for range requests {
		<-limiter
	}

	usedTime := getMs(start, time.Now())
	expectTime := int64(count * 10)
	if (usedTime - expectTime) > 10 {
		t.Error("RateLimit failed. total run time is too long, used:", usedTime)
	}
}

func TestRateLimitBursty(t *testing.T) {
	allCount := 5
	burstyCount := 3
	limiter := make(chan time.Time, 3)
	for i := 0; i < burstyCount; i++ {
		limiter <- time.Now()
	}
	go func() {
		for t := range time.Tick(time.Millisecond * 10) {
			limiter <- t
		}
	}()

	requests := make(chan int, allCount)
	for i := 1; i <= allCount; i++ {
		requests <- i
	}
	close(requests)
	start := time.Now()
	// unused elements in requests temporarily
	for range requests {
		<-limiter
	}

	usedTime := getMs(start, time.Now())
	expectTime := int64((allCount - burstyCount) * 10)
	if (usedTime - expectTime) > 10 {
		t.Error("RateLimit failed. total run time is too long, used:", usedTime)
	}
}

func TestAtomic(t *testing.T) {
	var ops uint64
	var isDone = false
	for i := 0; i < 50; i++ {
		go func() {
			for {
				if isDone {
					break
				}
				atomic.AddUint64(&ops, 1)
				runtime.Gosched()
			}
		}()
	}
	time.Sleep(time.Millisecond * 200)
	isDone = true
	time.Sleep(time.Millisecond * 20)
	opsFinal := atomic.LoadUint64(&ops)
	fmt.Println("ops:", opsFinal)
}

func TestMutexs(t *testing.T) {
	var state = make(map[int]int)
	var mutex = &sync.Mutex{}
	var ops int64
	var isDone = false
	var blackHole = make(chan int)

	go func() {
		for range blackHole {
		}
	}()

	for r := 0; r < 20; r++ {
		go func() {
			for {
				if isDone {
					break
				}
				key := rand.Intn(5)
				mutex.Lock()
				blackHole <- state[key]
				mutex.Unlock()
				atomic.AddInt64(&ops, 1)
				runtime.Gosched()
			}
		}()
	}

	for w := 0; w < 10; w++ {
		go func() {
			for {
				if isDone {
					break
				}
				key := rand.Intn(5)
				val := rand.Intn(100)
				mutex.Lock()
				state[key] = val
				mutex.Unlock()
				atomic.AddInt64(&ops, 1)
				runtime.Gosched()
			}
		}()
	}
	time.Sleep(time.Millisecond * 200)
	isDone = true
	time.Sleep(time.Millisecond * 20)

	opsFinal := atomic.LoadInt64(&ops)
	fmt.Println("ops:", opsFinal)

	mutex.Lock()
	fmt.Println("state:", state)
	mutex.Unlock()
}

type readOp struct {
	key  int
	resp chan int
}
type writeOp struct {
	key  int
	val  int
	resp chan bool
}

func TestGoroutineStateful(t *testing.T) {
	var ops int64
	reads := make(chan *readOp)
	writes := make(chan *writeOp)
	var isDone = false

	go func() {
		var state = make(map[int]int)
		for {
			select {
			case read := <-reads:
				read.resp <- state[read.key]
			case write := <-writes:
				state[write.key] = write.val
				write.resp <- true
			}
		}
	}()

	for r := 0; r < 20; r++ {
		go func() {
			for {
				if isDone {
					break
				}
				read := &readOp{
					key:  rand.Intn(5),
					resp: make(chan int)}
				reads <- read
				<-read.resp
				atomic.AddInt64(&ops, 1)
			}
		}()
	}

	for w := 0; w < 10; w++ {
		go func() {
			for {
				if isDone {
					break
				}
				write := &writeOp{
					key:  rand.Intn(5),
					val:  rand.Intn(100),
					resp: make(chan bool)}
				writes <- write
				<-write.resp
				atomic.AddInt64(&ops, 1)
			}
		}()
	}

	time.Sleep(time.Millisecond * 200)
	isDone = true
	time.Sleep(time.Millisecond * 20)

	opsFinal := atomic.LoadInt64(&ops)
	fmt.Println("ops:", opsFinal)
}
