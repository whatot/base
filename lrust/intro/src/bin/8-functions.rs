fn main() {
    c0_fizzbuzz();
    println!();
    c1_methods();
    println!();
    // http://huonw.github.io/blog/2015/05/finding-closure-in-rust/
    c2_closures();
    println!();
    c2_closures_capture();
    println!();
    c2_closures_as_input_parameter();
    println!();
    c2_closures_type_anonymity();
    println!();
    c2_closures_input_function();
    println!();
    c2_closures_as_output_parameter();
    println!();
    c2_closures_std_iterator_any();
    println!();
    c2_closures_std_iterator_find();
    println!();
    c3_higher_order_functions()
}

fn c0_fizzbuzz() {
    fizzbuzz_to(8);
}

fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n)
    }
}

fn fizzbuzz(n: u32) {
    fn is_divisible_by(l: u32, r: u32) -> bool {
        if r == 0 {
            return false;
        }
        l % r == 0
    }

    if is_divisible_by(n, 6) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 2) {
        println!("fizz");
    } else if is_divisible_by(n, 3) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is an instance method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;
        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    fn destroy(self) {
        // Destructure `self`
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
        // `first` and `second` go out of scope and get freed
    }
}

fn c1_methods() {
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 2.0),
    };
    square.translate(2.0, 3.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
}

fn c2_closures() {
    // Increment via closures and functions.
    fn function(i: i32) -> i32 {
        i + 1
    }

    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;

    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    let one = || 1;
    println!("closure return one: {}", one());
}

fn c2_closures_capture() {
    let color = "green";

    // A closure to print `color` which immediately borrows (`&`)
    // `color` and stores the borrow and closure in the `print`
    // variable. It will remain borrowed until `print` goes out of
    // scope. `println!` only requires `by reference` so it doesn't
    // impose anything more restrictive.
    let print_color = || println!("`color`: {}", color);

    // Call the closure using the borrow
    print_color();
    print_color();

    let mut count = 0;
    // A closure to increment `count` could take either `&mut count`
    // or `count` but `&mut count` is less restrictive so it takes
    // that. Immediately borrows `count`.
    //
    // A `mut` is required on `inc` because a `&mut` is stored inside.
    // Thus, calling the closure mutates the closure which requires
    // a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    inc();

    // A non-copy type.
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    use std::mem;
    let consume = || {
        println!("`movable:` {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable so this can only be called once.
    consume();
    // consume(); // will error
}

// Fn capture immutable variable, &T
// FnMut capture mutable variables, &mut T
// FnOnce capture variables by value, T
fn c2_closures_as_input_parameter() {
    fn apply<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }

    let greeting = "hello";
    // A non-copy type. `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("The I screamed {}!", farewell);
        println!("Now I can sleep. zzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        use std::mem;
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    fn apply_to_3<F>(f: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(3)
    }

    println!("3 doubled: {}", apply_to_3(double));
}

fn c2_closures_type_anonymity() {
    // `F` must implement `Fn` for a closure which takes no inputs
    // and returns nothing - exactly what is required for `print`.
    fn apply_anonymity<F>(f: F)
    where
        F: Fn(),
    {
        f();
    }

    let x = 7;

    // Capture `x` into an anonymous type and implement
    // `Fn` for it. Store it in `print`.
    let print = || println!("{}", x);

    apply_anonymity(print);
}

fn c2_closures_input_function() {
    // Define a function which takes a generic `F` argument
    // bounded by `Fn`, and calls it
    fn call_me<F: Fn()>(f: F) {
        f();
    }

    // Define a wrapper function satisfying the `Fn` bound
    fn function() {
        println!("I'm a function!");
    }

    // Define a closure satisfying the `Fn` bound
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}

// `move` keyword must be used, which signals that all captures occur by value.
// This is required because any captures by reference would be dropped
// as soon as the function exited, leaving invalid references in the closure.
fn c2_closures_as_output_parameter() {
    fn create_fn() -> Box<dyn Fn()> {
        let text = "Fn".to_owned();

        Box::new(move || println!("This is a: {}", text))
    }

    fn create_fnmut() -> Box<dyn FnMut()> {
        let text = "FnMut".to_owned();

        Box::new(move || println!("This is a: {}", text))
    }

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();

    fn_plain();
    fn_mut();
}

// fn any<F>(&mut self, f: F) -> bool where
//    F: FnMut(Self::Item) -> bool {}
fn c2_closures_std_iterator_any() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`. Destructure to `i32`.
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // `into_iter()` for vecs yields `i32`. No destructuring required.
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`.
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // `into_iter()` for arrays unusually yields `&i32`.
    println!("2 in array2: {}", array2.iter().any(|&x| x == 2));
}

// fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
//    P: FnMut(&Self::Item) -> bool {}
fn c2_closures_std_iterator_find() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`.
    let mut iter = vec1.iter();
    // `into_iter()` for vecs yields `i32`.
    let mut into_iter = vec2.into_iter();

    // A reference to what is yielded is `&&i32`. Destructure to `i32`.
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    // A reference to what is yielded is `&i32`. Destructure to `i32`.
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`
    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    // `into_iter()` for arrays unusually yields `&i32`
    println!("Find 2 in array2: {:?}", array2.iter().find(|&&x| x == 2));
}

fn c3_higher_order_functions() {
    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000u32;

    // Imperative approach
    // Declare accumulator variable
    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    // Functional approach
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n| n < upper)
        .filter(|&n| is_odd(n))
        .sum();
    println!("functional style: {}", sum_of_squared_odd_numbers);
}
