use crate::term;

pub fn main(
    error_folder: &String,
    server_ip: &String,
    server_user: &String,
    do_update_distro_debian: bool,
    dry_run: bool,
    services: &Vec<String>,
    users: &Vec<String>,
) {
    for user in users {
        println!("user: {user}");
    }

    for service in services {
        println!("service: {service}");
    }
}
