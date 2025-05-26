use crate::term;

pub fn main(error_folder: &String, services: Vec<String>) {
    for service in &services {
        println!("stopping: {service}");

        term::exec(
            error_folder,
            "systemctl",
            vec!["stop", service],
            &format!("stop service `{service}`"),
        );
    }
}
