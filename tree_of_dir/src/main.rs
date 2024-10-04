#![allow(unused)]
use std::{fs, io, path::PathBuf};

fn main() {
    // let a = match get_tree_dir() {
    //     Ok(u) => Vec<_>,
    //     Err(err) => eprintln!("error: {}", err),
    // };
    let a = get_tree_dir().unwrap();
    println!("{:?}", a);
}
fn get_tree_dir() -> io::Result<Vec<PathBuf>> {
    let mut entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();

    // The entries have now been sorted by their path.
    // println!("{:?}", entries);
    Ok(entries)
}