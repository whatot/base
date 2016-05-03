use std::env;
use std::fs;
use std::error::Error;

fn example() -> Result<(), Box<Error>> {
    let here = try!(env::current_dir());
    println!("list current dir: {}", here.display());
    for entry in try!(fs::read_dir(&here)) {
        let path = try!(entry).path();
        let md = try!(fs::metadata(&path));
        println!("    {} {} bytes", path.display(), md.len());
    }
    Ok(())
}

fn main() {
    example().unwrap()
}
