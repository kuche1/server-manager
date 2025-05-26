use crate::term;

const BANDWIDTH_LIMIT_KIB: u32 = 20480; // 20MiB
// TODO concatinate at compiletime

fn rsync(
    error_folder: &String,
    source_path: &str,
    dest_ip: &str,
    dest_user: &str,
    dest_user_home_relative_path: &str,
    recursive: bool,
) {
    let dest_path = &format!("/home/{dest_user}/{dest_user_home_relative_path}");
    let dest_path = &format!("{dest_user}@{dest_ip}:{dest_path}");

    term::exec(
        error_folder,
        "rsync",
        vec![
            "-av",
            "--delete-after",
            &format!("--bwlimit={BANDWIDTH_LIMIT_KIB}"),
            &format!("--recursive={recursive}"),
            source_path,
            dest_path,
        ],
        &format!(
            "rsync [recursive={recursive}]: source_path=`{source_path}`, dest_path=`{dest_path}`",
        ),
    );
}

pub fn main(
    error_folder: &String,
    source_path: &str,
    dest_ip: &str,
    dest_user: &str,
    dest_user_home_relative_path: &str,
) {
    println!("rsync: sync ({source_path}): working...");

    rsync(
        error_folder,
        source_path,
        dest_ip,
        dest_user,
        dest_user_home_relative_path,
        true,
    );

    println!("rsync: sync ({source_path}): done!");
}

pub fn remove_deleted(
    error_folder: &String,
    source_path: &str,
    dest_ip: &str,
    dest_user: &str,
    dest_user_home_relative_path: &str,
) {
    println!("rsync: remove deleted ({source_path}): working...");

    rsync(
        error_folder,
        source_path,
        dest_ip,
        dest_user,
        dest_user_home_relative_path,
        false,
    );

    println!("rsync: remove deleted ({source_path}): done!");
}
