use crate::log;

use std::process::Command;

pub fn main(error_folder: &String) {
    let cmd = match Command::new("sync").output() {
        Ok(v) => v,
        Err(err) => {
            log::err(error_folder, &format!("could call sync: {}", err));
            return;
        }
    };

    if !cmd.status.success() {
        log::err(error_folder, "sync failure");
        return;
    }
}
