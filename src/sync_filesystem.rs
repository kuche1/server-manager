use crate::term;

pub fn main(error_folder: &String, dry_run: bool) {
    println!("\nsync filesystem...");

    if dry_run {
    } else {
        term::exec(error_folder, "sync", vec![], "sync filesystem");
    }

    println!("    done!");
}
