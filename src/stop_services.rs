// use crate::log;
use crate::term;

// use std::process::Command;

pub fn main(error_folder: &String, services: Vec<String>) {
    for service in &services {
        println!("stopping: {service}");

        //         let cmd = match Command::new("systemctl").args(["stop", service]).output() {
        //             Ok(v) => v,
        //             Err(err) => {
        //                 log::err(
        //                     error_folder,
        //                     &format!(
        //                         "could not call systemd to stop service `{}`: {}",
        //                         service, err
        //                     ),
        //                 );
        //                 continue;
        //             }
        //         };
        //
        //         if !cmd.status.success() {
        //             log::err(
        //                 error_folder,
        //                 &format!(
        //                     "could not stop service `{}`: {}; stderr=`{}`",
        //                     service,
        //                     cmd.status,
        //                     String::from_utf8_lossy(&cmd.stderr)
        //                 ),
        //             )
        //         }
        term::exec(
            error_folder,
            "systemctl",
            vec!["stop", service],
            &format!("stop service `{service}`"),
        );
    }
}
