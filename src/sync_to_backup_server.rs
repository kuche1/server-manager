use crate::log;
use crate::rsync;
use crate::term;

use std::fs;
// use std::process::Command;

const SERVICE_FILES_LOCATION: &str = "/etc/systemd/system/";
// needs to end with `/`
// actually, this is not the only place they can be, but this is good enough

const BACKUP_SERVICE_FILES_LOCATION: &str = "etc_systemd_system";
// needs to not end with `/`
// relative to user's home

// TODO (alive branch tested) (dead branch untested)
fn server_is_dead(error_folder: &String, ip: &String) -> bool {
    //     let cmd = match Command::new("ping").args(["-c", "1", ip]).output() {
    //         Ok(v) => v,
    //         Err(err) => {
    //             log::err(error_folder, &format!("could not call ping: {}", err));
    //             return true;
    //         }
    //     };
    //
    //     if !cmd.status.success() {
    //         log::err(
    //             error_folder,
    //             &format!(
    //                 "ping failure for server `{}`: {}; stderr=`{}`",
    //                 ip,
    //                 cmd.status,
    //                 String::from_utf8_lossy(&cmd.stderr)
    //             ),
    //         );
    //         return true;
    //     }
    //
    //     false
    term::exec(
        error_folder,
        "ping",
        vec!["-c", "1", ip],
        &format!("ping server `{ip}`"),
    )
    .is_none()
}

fn copy_service_files(error_folder: &String, server_ip: &String, server_user: &String) {
    println!("copy service files: working...");
    rsync::main(
        error_folder,
        SERVICE_FILES_LOCATION,
        server_ip,
        server_user,
        BACKUP_SERVICE_FILES_LOCATION,
    );
    println!("copy service files: done!");
}

fn copy_data(error_folder: &String, server_ip: &String, server_user: &String) {
    println!("copy data: working...");

    let entries = match fs::read_dir("/home/") {
        Ok(v) => v,
        Err(err) => {
            log::err(
                error_folder,
                &format!("could not get a list of users: {}", err),
            );
            return;
        }
    };

    // println!("entries: {:?}", entries);
    let mut users = vec![];

    for entry in entries.flatten() {
        let file_type = match entry.file_type() {
            Ok(v) => v,
            Err(_) => continue,
        };

        if !file_type.is_dir() {
            continue;
        }

        let user = entry.file_name();

        let user = match user.to_str() {
            Some(v) => v,
            None => {
                log::err(&error_folder, "unreachable");
                continue;
            }
        };

        // println!("user: {}", user);
        users.push(user.to_owned());
    }

    for user in users {
        // println!("user: {}", user);
        let user_home = &format!("/home/{user}/");
        let backup_folder = format!("home/{user}");

        rsync::main(
            error_folder,
            user_home,
            server_ip,
            server_user,
            &backup_folder,
        );
    }

    println!("copy data: done!");
}

fn remove_deleted(error_folder: &String, server_ip: &String, server_user: &String) {
    println!("remove deleted: working...");

    rsync::remove_deleted(error_folder, "/home/", server_ip, server_user, "home");

    println!("remove deleted: done!");
}

pub fn main(error_folder: &String, server_ip: &String, server_user: &String) {
    if server_is_dead(error_folder, server_ip) {
        log::err(
            error_folder,
            "could not sync to backup server, as it is dead",
        );
        return;
    }

    copy_service_files(error_folder, server_ip, server_user);

    copy_data(error_folder, server_ip, server_user);

    remove_deleted(error_folder, server_ip, server_user);
}
