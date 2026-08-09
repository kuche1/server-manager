use crate::log;
use crate::start_service_if_enabled;
use crate::sync_to_backup_server;
use crate::term;

pub fn main(
    error_folder: &String,
    server_ip: &String,
    server_user: &String,
    start_service_again_after_stopping_it: bool,
    dry_run: bool,
    services: &Vec<String>,
    mut users: Vec<String>,
) {
    let backup_server_is_alive = !sync_to_backup_server::server_is_dead(error_folder, server_ip);

    if !backup_server_is_alive {
        log::err(error_folder, "backup server is dead");
    }

    if backup_server_is_alive {
        sync_to_backup_server::copy_service_files(error_folder, dry_run, server_ip, server_user);
    }

    for (service_idx, service) in services.iter().enumerate() {
        println!("\n[{}/{}] {}", service_idx + 1, services.len(), service);

        println!("stopping...");

        if dry_run {
        } else {
            term::exec(
                error_folder,
                "systemctl",
                vec!["stop", service],
                &format!("stop service `{service}`"),
            );
        }

        println!("stopped!");

        let idx = match users.iter().position(|u| u == service) {
            Some(v) => v,
            None => continue,
        };

        let user = users.swap_remove(idx);

        sync_to_backup_server::copy_user_data(error_folder, server_ip, server_user, dry_run, &user);

        if start_service_again_after_stopping_it {
            start_service_if_enabled::main(error_folder, dry_run, service);
        }
    }

    println!("\n{} users with no associated services", users.len());

    users.sort();

    for (user_idx, user) in users.iter().enumerate() {
        println!("\n[{}/{}] {}", user_idx + 1, users.len(), user);
        sync_to_backup_server::copy_user_data(error_folder, server_ip, server_user, dry_run, user);
    }

    println!("");
    sync_to_backup_server::remove_deleted_users(error_folder, server_ip, dry_run, server_user);
}
