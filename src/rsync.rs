// use crate::log;
use crate::term;

// use std::process::Command;

pub fn main(
    error_folder: &String,
    source_path: &str,
    dest_ip: &str,
    dest_user: &str,
    dest_user_home_relative_path: &str,
) {
    let dest_path = &format!("/home/{dest_user}/{dest_user_home_relative_path}");
    let dest_path = &format!("{dest_user}@{dest_ip}:{dest_path}");

    println!("rsync: {source_path} -> {dest_path}");

    //     let cmd = match Command::new("rsync")
    //         .args([
    //             "-av",
    //             "--delete-after",
    //             "--bwlimit=20480", // 20MiB // TODO concatinate at compiletime
    //             source_path,
    //             &format!("{dest_user}@{dest_ip}:{dest_path}"),
    //         ])
    //         .output()
    //     {
    //         Ok(v) => v,
    //         Err(err) => {
    //             log::err(error_folder, &format!("could not call rsync: {}", err));
    //             return;
    //         }
    //     };
    //
    //     if !cmd.status.success() {
    //         log::err(
    //             error_folder,
    //             &format!(
    //                 "rsync failure: source_path=`{}`, dest_ip=`{}`, dest_user=`{}`, dest_path=`{}` -> {}; stderr=`{}`",
    //                 source_path,
    //                 dest_ip,
    //                 dest_user,
    //                 dest_path,
    //                 cmd.status,
    //                 String::from_utf8_lossy(&cmd.stderr)
    //             ),
    //         );
    //         return;
    //     }
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
}
