fn main() {
    c4_bounds();
    println!();
    c4_empty_bounds();
    println!();
    c5_multiple_bounds();
    println!();
    c6_where_bounds();
    println!();
    c7_associated_items_1();
    println!();
    c7_associated_items_2();
}

use std::fmt::Debug;
use std::fmt::Display;

trait HasArea {
    fn area(&self) -> f64;
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}
#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

// The generic `T` must implement `Debug`. Regardless
// of the type, this will work properly.
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn c4_bounds() {
    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    // print_debug(&_triangle);
    // println!("Area: {}", area(&_triangle));
    // Error: Does not implement either `Debug` or `HasArea`
}

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// These functions are only valid for types which implement these
// traits. The fact that the traits are empty is irrelevant.
fn red<T: Red>(_: &T) -> &'static str {
    "red"
}
fn blue<T: Blue>(_: &T) -> &'static str {
    "blue"
}

fn c4_empty_bounds() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    // `red()` won't work on a blue jay nor vice versa
    // because of the bounds.
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    // println!("A turkey is {}", red(&_turkey));
    // Error: the trait bound `Turkey: Red` is not satisfied
}


fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: {:?}", t);
    println!("Display: {}", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: {:?}", t);
    println!("u: {:?}", u);
}

fn c5_multiple_bounds() {
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    compare_types(&array, &vec);

    // compare_prints(&array);
    // Error: std::fmt::Display is not implemented...
}

trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T
    where Option<T>: Debug
{
    // We want `Option<T>: Debug` as our bound because that is what's
    // being printed. Doing otherwise would be using the wrong bound.
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn c6_where_bounds() {
    let vec = vec![1, 2, 3];
    vec.print_in_option();
}

fn c7_associated_items_1() {
    struct Container(i32, i32);
    trait Contains<A, B> {
        fn contains(&self, &A, &B) -> bool; // explicitly requires A amd B
        fn first(&self) -> i32; // doesn't explicitly requires A and B
        fn last(&self) -> i32; // doesn't explicitly requires A and B
    }

    impl Contains<i32, i32> for Container {
        fn contains(&self, num1: &i32, num2: &i32) -> bool {
            (&self.0 == num1) && (&self.1 == num2)
        }
        fn first(&self) -> i32 {
            self.0
        }
        fn last(&self) -> i32 {
            self.1
        }
    }

    // `C` contains `A` and `B`. In light of that, having to express `A` and
    // `B` again is a nuisance.
    fn difference<A, B, C>(container: &C) -> i32
        where C: Contains<A, B>
    {
        container.last() - container.first()
    }

    let number_1 = 3;
    let number_2 = 8;
    let container = Container(number_1, number_2);
    println!("Does container contains {} and {} : {}",
             &number_1,
             &number_2,
             container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    println!("The difference is : {}", difference(&container));
}

// Associated types
fn c7_associated_items_2() {
    struct Container(i32, i32);
    trait Contains {
        // Define generic types here which methods will be able to utilize.
        type A;
        type B;

        fn contains(&self, &Self::A, &Self::B) -> bool;
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }

    impl Contains for Container {
        // Specify what types `A` and `B` are. If the `input` type
        // is `Container(i32, i32)`, the `output` types are determined
        // as `i32` and `i32`.
        type A = i32;
        type B = i32;

        fn contains(&self, num1: &i32, num2: &i32) -> bool {
            (&self.0 == num1) && (&self.1 == num2)
        }
        fn first(&self) -> i32 {
            self.0
        }
        fn last(&self) -> i32 {
            self.1
        }
    }

    // `C` contains `A` and `B`. In light of that, having to express `A` and
    // `B` again is a nuisance.
    fn difference<C: Contains>(container: &C) -> i32 {
        container.last() - container.first()
    }

    let number_1 = 3;
    let number_2 = 8;
    let container = Container(number_1, number_2);
    println!("Does container contains {} and {} : {}",
             &number_1,
             &number_2,
             container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    println!("The difference is : {}", difference(&container));
}
