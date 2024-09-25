fn main() {
    let regular_str = "Hello, World!";
    let str_with_escape = "Hello \"Hello\" Hello!";
    let str_with_escape = "\"Hello\"\" Hello!";
    let str_with_escape = "\"Hello\"";
    let empty = "";
    let ml_string = "Hello, 
World!";
    let nested_1 = "Hello there!
/* this is not actually a ML comment. It looks like one, but it's part of  a string. */
    ";
/* This is a ML comment
It looks like a multi line string ends here: "
but that is not true, it's just a quote within an ML comment. */
}