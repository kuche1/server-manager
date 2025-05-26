use crate::term;

pub fn main(error_folder: &String) {
    println!("reboot: working...");

    term::exec(error_folder, "reboot", vec![], "reboot server");

    println!("reboot: done! (might take some time until the reboot it performed)");
}
