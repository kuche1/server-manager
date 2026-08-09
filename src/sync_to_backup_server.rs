use crate::rsync;
use crate::term;

// use std::process::Command;

const SERVICE_FILES_LOCATION: &str = "/etc/systemd/system/";
// needs to end with `/`
// actually, this is not the only place they can be, but this is good enough

const BACKUP_SERVICE_FILES_LOCATION: &str = "etc_systemd_system";
// needs to not end with `/`
// relative to user's home

// TODO (alive branch tested) (dead branch untested)
pub fn server_is_dead(error_folder: &String, ip: &String) -> bool {
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

pub fn copy_service_files(
    error_folder: &String,
    dry_run: bool,
    server_ip: &String,
    server_user: &String,
) {
    println!("copy service files: working...");
    rsync::main(
        error_folder,
        dry_run,
        SERVICE_FILES_LOCATION,
        server_ip,
        server_user,
        BACKUP_SERVICE_FILES_LOCATION,
    );
    println!("copy service files: done!");
}

pub fn copy_user_data(
    error_folder: &String,
    server_ip: &String,
    server_user: &String,
    dry_run: bool,
    user: &String,
) {
    println!("copy user data...");

    let user_home = &format!("/home/{user}/");
    let backup_folder = format!("home/{user}");

    rsync::main(
        error_folder,
        dry_run,
        user_home,
        server_ip,
        server_user,
        &backup_folder,
    );

    println!("copied user data!");
}

pub fn remove_deleted_users(
    error_folder: &String,
    server_ip: &String,
    dry_run: bool,
    server_user: &String,
) {
    println!("remove deleted: working...");

    rsync::remove_deleted(
        error_folder,
        dry_run,
        "/home/",
        server_ip,
        server_user,
        "home",
    );

    println!("remove deleted: done!");
}
