use crate::log;
use crate::term;

use std::process::Command;

fn is_enabled(error_folder: &String, service: &str) -> Option<bool> {
    let cmd = match Command::new("systemctl")
        .args(vec!["is-enabled", "--quiet", service])
        .output()
    {
        Ok(v) => v,
        Err(err) => {
            log::err(
                error_folder,
                &format!("could not check if service `{service}` is enabled -> {err}"),
            );
            return None;
        }
    };

    // 0 -> is enabled
    // 1 -> is disabled
    // 4 -> service does not exist
    let return_code = match cmd.status.code() {
        Some(v) => v,
        None => {
            log::err(
                error_folder,
                &format!(
                    "could not get the command return code when checking if `{service}` is enabled"
                ),
            );
            return None;
        }
    };

    match return_code {
        0 => return Some(true),
        1 => return Some(false),
        _ => {
            log::err(
                error_folder,
                &format!(
                    "got unexpected return code when checking if `{service}` is enabled -> {return_code}"
                ),
            );
            return None;
        }
    };
}

pub fn main(error_folder: &String, dry_run: bool, service: &String) {
    println!("start service...");

    let enabled = match is_enabled(error_folder, service) {
        Some(v) => v,
        None => return, // do not attempt to start if anything went wrong
    };

    if enabled {
    } else {
        println!("    NOT starting disabled service!");
        return;
    }

    if dry_run {
    } else {
        term::exec(
            error_folder,
            "systemctl",
            vec!["start", service],
            &format!("start service `{service}`"),
        );
    }

    println!("    done!");
}
