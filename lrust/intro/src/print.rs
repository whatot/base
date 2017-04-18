fn main() {
    let a = [10, 20, 30, 40];
    for w in a.iter() {
        println!("{}", w);
    }

    for number in (1..4).rev() {
        println!("{}", number);
    }

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}
