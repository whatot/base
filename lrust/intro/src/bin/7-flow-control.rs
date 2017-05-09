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
