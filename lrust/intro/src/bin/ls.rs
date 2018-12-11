use std::env;
use std::fs;
use std::error::Error;

fn ls_current() -> Result<(), Box<Error>> {
    let here = r#try!(env::current_dir());
    println!("list current dir: {}", here.display());
    for entry in r#try!(fs::read_dir(&here)) {
        let path = r#try!(entry).path();
        let md = r#try!(fs::metadata(&path));
        println!("    {} {} bytes", path.display(), md.len());
    }
    Ok(())
}

fn main() {
    ls_current().unwrap()
}
