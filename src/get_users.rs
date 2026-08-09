use crate::log;

use std::fs;

pub fn main(error_folder: &String) -> Vec<String> {
    let entries = match fs::read_dir("/home/") {
        Ok(v) => v,
        Err(err) => {
            log::err(
                error_folder,
                &format!("could not get a list of users: {}", err),
            );
            return vec![];
        }
    };

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

    return users;
}
