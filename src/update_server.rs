use crate::term;

pub fn main(error_folder: &String, update_server_debian: bool) {
    if !update_server_debian {
        return;
    }

    println!("update_server: debian: fetch updates: working...");

    let res = term::exec(error_folder, "apt", vec!["update"], "debian: fetch updates");
    if res.is_none() {
        return;
    }

    let res = term::exec(
        error_folder,
        "apt",
        vec!["upgrade"],
        "debian: apply updates",
    );
    if res.is_none() {
        return;
    }

    println!("update_server: debian: fetch updates: done!");
}
