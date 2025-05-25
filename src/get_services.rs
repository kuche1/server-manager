use crate::log;
use crate::term;

// use std::process::Command;

const SERVICE_SUFFIX: &str = ".service";

pub fn main(error_folder: &String, services_prefix: &String) -> Vec<String> {
    //     let cmd = match Command::new("systemctl")
    //         .args([
    //             "list-units",
    //             "--type=service",
    //             "--all",
    //             "--no-legend",
    //             "--plain",
    //         ])
    //         .output()
    //     {
    //         Ok(v) => v,
    //         Err(err) => {
    //             log::err(
    //                 error_folder,
    //                 &format!("could not call systemd to get services: {}", err),
    //             );
    //             return vec![];
    //         }
    //     };
    //
    //     if !cmd.status.success() {
    //         log::err(
    //             error_folder,
    //             &format!(
    //                 "called systemd, but failure occured: {}; stderr=`{}`",
    //                 cmd.status,
    //                 String::from_utf8_lossy(&cmd.stderr)
    //             ),
    //         )
    //     }
    term::exec(
        error_folder,
        "systemctl",
        vec![
            "list-units",
            "--type=service",
            "--all",
            "--no-legend",
            "--plain",
        ],
        "get service names",
    );

    let data = cmd.stdout;
    let data = String::from_utf8_lossy(&data);

    let mut services = vec![];

    for line in data.lines() {
        let idx = match line.find(SERVICE_SUFFIX) {
            Some(v) => v,
            None => {
                log::err(
                    error_folder,
                    &format!(
                        "could not find suffix `{}` on line `{}`",
                        SERVICE_SUFFIX, line
                    ),
                );
                continue;
            }
        };

        let service_name = &line[0..idx + SERVICE_SUFFIX.len()];

        if !service_name.starts_with(services_prefix) {
            continue;
        }

        services.push(service_name.to_owned());
    }

    services
}
