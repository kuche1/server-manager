use crate::term;

pub fn main(
    error_folder: &String,
    source_path: &str,
    dest_ip: &str,
    dest_user: &str,
    dest_user_home_relative_path: &str,
) {
    let dest_path = &format!("/home/{dest_user}/{dest_user_home_relative_path}");
    let dest_path = &format!("{dest_user}@{dest_ip}:{dest_path}");

    println!("rsync: working: {source_path} -> {dest_path}");

    term::exec(
        error_folder,
        "rsync",
        vec![
            "-av",
            "--delete-after",
            "--bwlimit=20480", // 20MiB // TODO concatinate at compiletime
            source_path,
            dest_path,
        ],
        &format!("sync: source_path=`{source_path}`, dest_path=`{dest_path}`",),
    );

    println!("rsync: done");
}
