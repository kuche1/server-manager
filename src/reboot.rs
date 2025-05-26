// use crate::log;
use crate::term;

// use std::process::Command;

// TODO untested
pub fn main(error_folder: &String) {
    //     let cmd = match Command::new("reboot").output() {
    //         Ok(v) => v,
    //         Err(err) => {
    //             log::err(error_folder, &format!("could not call reboot: {}", err));
    //             return;
    //         }
    //     };
    //
    //     if !cmd.status.success() {
    //         log::err(
    //             error_folder,
    //             &format!(
    //                 "call to reboot failed -> {}; stderr=`{}`",
    //                 cmd.status,
    //                 String::from_utf8_lossy(&cmd.stderr)
    //             ),
    //         );
    //         return;
    //     }
    term::exec(error_folder, "reboot", vec![], "reboot server");
}
