use crate::term;

pub fn main(error_folder: &String, dry_run: bool) {
    println!("reboot: working...");

    if dry_run {
    } else {
        term::exec(error_folder, "reboot", vec![], "reboot server");
    }

    println!("reboot: done! (might take some time until the reboot it performed)");
}
