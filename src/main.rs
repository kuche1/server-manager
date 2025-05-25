// needs to be run as root, as to be able to stop the services (and perform a restart)

mod args;
mod get_services;
mod log;
mod sync_filesystem;
mod wait;

use std::process::Command;

fn main() {
    //// get args

    let args = args::get();

    let error_folder = &args.error_folder;

    //// wait until it's time to restart

    wait::main(args.restart_at, args.check_time_sleep_sec);

    //////
    ////// get services
    //////

    let services = get_services::main(&error_folder, &args.services_prefix);

    //////
    ////// stop services
    //////

    for service in &services {
        println!("stopping: {service}");

        let cmd = match Command::new("systemctl").args(["stop", service]).output() {
            Ok(v) => v,
            Err(err) => {
                log::err(
                    error_folder,
                    &format!(
                        "could not call systemd to stop service `{}`: {}",
                        service, err
                    ),
                );
                continue;
            }
        };

        if !cmd.status.success() {
            log::err(
                error_folder,
                &format!(
                    "could not stop service `{}`: {}; stderr=`{}`",
                    service,
                    cmd.status,
                    String::from_utf8_lossy(&cmd.stderr)
                ),
            )
        }
    }

    //////
    ////// TODO sync to server
    //////

    {
        // ping server

        // copy services

        // copy files
    }

    //////
    ////// sync filesystem
    //////

    sync_filesystem::main(error_folder);

    //////
    ////// TODO reboot
    //////
}
