fn main() {
    c1_if_else();
    println!();
    c2_loop();
    println!();
    c2_nested_loop();
    println!();
    c3_while();
    println!();
    c4_for_range();
    println!();
    c5_match();
    println!();
    c5_match_destructure_tuple();
    println!();
    c5_match_destructure_enum();
    println!();
    c5_match_destructure_pointer_ref();
    println!();
    c5_destructure_struct();
    println!();
    c5_match_guard();
    println!();
    c5_match_binding();
    println!();
    c6_if_let();
    println!();
    c7_while_let();
}

fn c1_if_else() {
    let n = 9;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");
        10 * n
    } else {
        println!(", and is a big number, reduce by two");
        n / 2
    };
    println!("{} -> {}", n, big_n);
}

fn c2_loop() {
    let mut count = 0u32;
    println!("let's us count until infinity!");

    loop {
        count += 1;
        if count == 3 {
            println!("three");
            // skip the rest of this iteration
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough");
            // Exit this loop
            break;
        }
    }
}

#[allow(unreachable_code)]
fn c2_nested_loop() {
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
}

fn fizzbuzz(n: u32) {
    if n % 6 == 0 {
        println!("fizzbuzz");
    } else if n % 2 == 0 {
        println!("fizz");
    } else if n % 5 == 0 {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

fn c3_while() {
    let mut n = 1;
    while n < 10 {
        fizzbuzz(n);
        n += 1;
    }
}

fn c4_for_range() {
    for n in 1..10 {
        fizzbuzz(n);
    }
}

fn c5_match() {
    let number = 11;

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // Match an inclusive range
        13...19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);
}

fn c5_match_destructure_tuple() {
    // destructure tuple
    let pair = (5, 0);
    match pair {
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _ => println!("It doesn't matter what they are"),
    }
}

#[allow(dead_code)]
enum Color {
    // These 3 are specified solely by their name
    Red,
    Blue,
    Green,
    // These likewise tie `u32` tuples to different names: color models
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn c5_match_destructure_enum() {
    let color = Color::CMY(122, 17, 40);

    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => {
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                     c,
                     m,
                     y,
                     k)
        }
        // Don't need another arm because all variants have been examined
    }
}

fn c5_match_destructure_pointer_ref() {
    // Assign a reference of type `i32`. The `&` signifies there
    // is a reference being assigned.
    let reference = &4;

    match reference {
        // If `reference`s is pattern matched against `&val`, it results
        // in a comparison like:
        // `&i32`
        // `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the `&`, you dereference before matching.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // What if you don't start with a reference? `reference` was a `&`
    // because the right side was already a reference. This is not
    // a reference because the right side is not one.
    let _not_a_reference = 3;

    // Rust provides `ref` for exactly this purpose. It modifies the
    // assignment so that a reference is created for the element; this
    // reference is assigned.
    let ref _is_a_reference = 3;

    // Accordingly, by defining 2 values without references, references
    // can be retrieved via `ref` and `ref mut`.
    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Use `ref mut` similarly.
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }
}

fn c5_destructure_struct() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // destructure members of the struct
    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;
    println!("a = {}, b = {}, y = {}", a, b, y);

    // you can destructure structs and rename the variables,
    // the order is not important
    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);

    // and you can also ignore some variables:
    let Foo { y, .. } = foo;
    println!("y = {}", y);

    // this will give an error: pattern does not mention field `x`
    // let Foo { y } = foo;
}

fn c5_match_guard() {
    let pair = (12, -12);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}

fn age() -> u32 {
    18
}

fn c5_match_binding() {
    match age() {
        0 => println!("I'm not born yet I guess"),
        n @ 1...12 => println!("I'm a child of age {:?}", n),
        n @ 13...19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }
}

fn c6_if_let() {
    // All have type `Option<i32>`
    let number = Some(7);
    let letter: Option<i32> = None;
    let emotion: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructures `number` into
    // `Some(i)`, evaluate the block (`{}`).
    if let Some(i) = number {
        println!("Matched {:?}", i);
    }

    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to the failure case.
        println!("Didn't match a number. Let's go with a letter!");
    };

    // Provide an altered failing condition.
    let i_like_letters = false;

    if let Some(i) = emotion {
        println!("Matched {:?}!", i);
        // Destructure failed. Evaluate an `else if` condition to see if the
        // alternate failure branch should be taken:
    } else if i_like_letters {
        println!("Didn't match a number, let's go with a letter");
    } else {
        // The condition evaluated false. This branch is the default:
        println!("I don't like letters, let's go with an emotion.");
    }
}

fn c7_while_let() {
    // Make `optional` of type `Option<i32>`
    let mut optional = Some(0);

    // Repeatedly try this test.
    loop {
        match optional {
            // If `optional` destructures, evaluate the block.
            Some(i) => {
                if i > 3 {
                    println!("Greater than 3, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
                // ^ Requires 3 indentations!
            }
            // Quit the loop when the destructure fails:
            _ => {
                break;
            }
            // ^ Why should this be required? There must be a better way!
        }
    }

    // Better way to handle the situation like up
    // This reads: "while `let` destructures `optional` into
    // `Some(i)`, evaluate the block (`{}`). Else `break`.
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 3 {
            println!("Greater than 3, quit");
            optional = None;
        } else {
            println!("`i` is `{:?}`, Try again.", i);
            optional = Some(i + 1);
        }
        // ^ Less rightward drift and doesn't require
        // explicitly handling the failing case.
    }
    // ^ `if let` had additional optional `else`/`else if`
    // clauses. `while let` does not have these.
}
