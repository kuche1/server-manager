use crate::term;

pub fn main(error_folder: &String) {
    println!("sync filesystem: working...");

    term::exec(error_folder, "sync", vec![], "sync filesystem");

    println!("sync filesystem: done!");
}
