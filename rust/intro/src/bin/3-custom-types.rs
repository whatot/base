fn main() {
    print_structures();
    println!();
    print_enums();
    println!();
    print_c_enums();
    println!();
    print_linked_list();
}

// A unit struct
struct OneUnit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Struts can be reused as fields of other struct
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f32 {
        (self.p1.x - self.p2.x).abs() * (self.p1.y - self.p2.y).abs()
    }
}

fn square(p: Point, len: f32) -> Rectangle {
    Rectangle {
        p1: Point { x: p.x, y: p.y }, // lower left corner
        p2: Point {
            // higher right corner
            x: p.x + len,
            y: p.y + len,
        },
    }
}

fn print_structures() {
    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the struct
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Destructure the point using a `let` binding
    let Point { x: new_x, y: new_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: new_x, y: new_y },
        p2: point,
    };

    // Instantiate a unit struct
    let _nil = OneUnit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("area of rectangle: {}", _rectangle.area());
    println!(
        "area of new square rectangle: {}",
        square(Point { x: 1.1, y: 2.2 }, 3.3).area()
    );
}

#[allow(dead_code)]
enum Person {
    Engineer,
    Scientist,
    Height(i32),
    Weight(i32),
    Info { name: String, height: i32 },
}

fn inspect(p: Person) {
    match p {
        Person::Engineer => println!("Is an engineer!"),
        Person::Scientist => println!("Is a scientist!"),
        Person::Height(i) => println!("Has a height of {}", i),
        Person::Weight(i) => println!("Has a weight of {}", i),
        Person::Info { name, height } => {
            println!("{} has {} tall!", name, height);
        }
    }
}

fn print_enums() {
    let person = Person::Height(18);
    let amira = Person::Weight(10);
    // `to_owned()` creates an owned `String` from a string slice.
    let dave = Person::Info {
        name: "Dave".to_owned(),
        height: 72,
    };
    let rebecca = Person::Scientist;
    let rohan = Person::Engineer;

    inspect(person);
    inspect(amira);
    inspect(dave);
    inspect(rebecca);
    inspect(rohan);
}

#[allow(dead_code)]
enum Number {
    Zero,
    One,
    Two,
}

#[allow(dead_code)]
enum Color {
    Red = 0x00ff_0000,
    Green = 0x0000_ff00,
    Blue = 0x0000_00ff,
}

fn print_c_enums() {
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

use crate::List::*;
enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // End Case: An empty list has zero length
            Nil => 0,
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            }
            Nil => "Nil".to_string(),
        }
    }
}

fn print_linked_list() {
    // Create an empty linked list
    let mut list = List::new();

    // Append some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

// constants
#[allow(dead_code)]
static LANGUAGE: &str = "rust";
#[allow(dead_code)]
const THRESHOLD: i32 = 10;
