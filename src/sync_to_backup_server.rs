use crate::log;

use std::process::Command;

const SERVICE_FILES_LOCATION: &str = "/etc/systemd/system/";
// needs to end with `/` (I think)
// actually, this is not the only place they can be, but this is good enough

const BACKUP_SERVICE_FILES_LOCATION: &str = "etc_systemd_system";
// located in home

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
                "ping failure for server `{}`: {}; stderr=`{}`",
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
fn copy_service_files(error_folder: &String, server_ip: &String, server_user: &String) {
    println!("copying service files...");

    let cmd = match Command::new("rsync")
        .args([
            "-av",
            "--delete-after",
            "--bwlimit=20480", // 20MiB
            SERVICE_FILES_LOCATION,
            &format!(
                "{}@{}:{}",
                server_user, server_ip, BACKUP_SERVICE_FILES_LOCATION
            ),
        ])
        .output()
    {
        Ok(v) => v,
        Err(err) => {
            log::err(error_folder, &format!("could not call rsync: {}", err));
            return;
        }
    };

    if !cmd.status.success() {
        log::err(
            error_folder,
            &format!(
                "rsync failure for server `{}` user `{}`: {}; stderr=`{}`",
                server_ip,
                server_user,
                cmd.status,
                String::from_utf8_lossy(&cmd.stderr)
            ),
        );
        return;
    }
}

// TODO untested
pub fn main(error_folder: &String, server_ip: &String, server_user: &String) {
    if server_is_dead(error_folder, server_ip) {
        log::err(
            error_folder,
            "could not sync to backup server, as it is dead",
        );
        return;
    }

    copy_service_files(error_folder, server_ip, server_user);

    // TODO copy data
}
