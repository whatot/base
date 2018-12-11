use std::env;
use std::error::Error;
use std::fs;

fn ls_current() -> Result<(), Box<Error>> {
    let here = env::current_dir()?;
    println!("list current dir: {}", here.display());
    for entry in fs::read_dir(&here)? {
        let path = entry?.path();
        let md = fs::metadata(&path)?;
        println!("    {} {} bytes", path.display(), md.len());
    }
    Ok(())
}

fn main() {
    ls_current().unwrap()
}
