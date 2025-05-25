use std::process::Command;

use crate::log;

// TODO untested
fn server_is_dead(error_folder: &String, ip: &String) -> bool {
    // ping -c 1 IP

    let cmd = match Command::new("ping").args(["-c", "1", ip]).output() {
        Ok(v) => v,
        Err(err) => {
            log::err(error_folder, &format!("could not call ping: {}", err));
            return true;
        }
    };

    if !cmd.status.success() {
        log::err(
            error_folder,
            &format!(
                "could ping failure for server `{}`: {}; stderr=`{}`",
                ip,
                cmd.status,
                String::from_utf8_lossy(&cmd.stderr)
            ),
        );
        return true;
    }

    false
}

// TODO untested
// TODO implement
pub fn main(error_folder: &String, server_ip: &String, server_user: &String) {
    // ping server
    if server_is_dead(error_folder, server_ip) {
        log::err(
            error_folder,
            "could not sync to backup server, as it is dead",
        );
        return;
    }

    println!("copying service metadata...");
    // copy services

    // copy files
}
