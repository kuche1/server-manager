use crate::term;

pub fn main(error_folder: &String, update_server_debian: bool) {
    if !update_server_debian {
        return;
    }

    println!("update_server: debian: fetch updates: working...");

    let output = term::exec(
        error_folder,
        "apt-get",
        vec!["update"],
        "debian: fetch updates",
    );
    let output = match output {
        Some(v) => v,
        None => return,
    };
    let output = String::from_utf8_lossy(&output.stdout); // TODO: do not use the lossy version
    println!("{}", output);

    println!("update_server: debian: fetch updates: done!");

    println!("update_server: debian: apply updates: working...");

    let output = term::exec(
        error_folder,
        "apt-get",
        vec!["upgrade", "--yes"],
        "debian: apply updates",
    );
    let output = match output {
        Some(v) => v,
        None => return,
    };
    let output = String::from_utf8_lossy(&output.stdout); // TODO: do not use the lossy version
    println!("{}", output);

    println!("update_server: debian: apply updates: done!");
}
