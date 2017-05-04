fn main() {
    print_structures();
    println!();
    print_enums();
}


// A unit struct
struct Nil;

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
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("area of rectangle: {}", _rectangle.area());
    println!("area of new square rectangle: {}",
             square(Point { x: 1.1, y: 2.2 }, 3.3).area());
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
