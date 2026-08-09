use crate::term;

pub fn main(error_folder: &String, dry_run: bool, services: &Vec<String>) {
    for service in services {
        println!("stopping: {service}");

        if dry_run {
        } else {
            term::exec(
                error_folder,
                "systemctl",
                vec!["stop", service],
                &format!("stop service `{service}`"),
            );
        }
    }
}
