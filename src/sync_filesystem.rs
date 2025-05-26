// use crate::log;
use crate::term;

// use std::process::Command;

pub fn main(error_folder: &String) {
    println!("sync filesystem: working...");

    //     let cmd = match Command::new("sync").output() {
    //         Ok(v) => v,
    //         Err(err) => {
    //             log::err(error_folder, &format!("could call sync: {}", err));
    //             return;
    //         }
    //     };
    //
    //     if !cmd.status.success() {
    //         log::err(error_folder, "sync failure");
    //         return;
    //     }
    term::exec(error_folder, "sync", vec![], "sync filesystem");

    println!("sync filesystem: done!");
}
