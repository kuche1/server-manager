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

    let args = {
        // `-a` is equivalent to `-rlptgoD` (see `rsync --help`)
        // `-r` recursive (or `-d` for non-recursive)
        // `-l` preserve symlinks
        // `-p` preserve permissions
        // `-t` "preserve modification times"
        // `-g` preserve group
        // `-o` preserve owner
        // `-D` "same as --devices --specials"
        // `-N` "preserve create times (newness)"

        if recursive { "-rlptgoD" } else { "-dlptgoD" }
    };

    term::exec(
        error_folder,
        "rsync",
        vec![
            args,
            // "-v", // verbose
            "--delete-after",
            &format!("--bwlimit={BANDWIDTH_LIMIT_KIB}"),
            source_path,
            dest_path,
        ],
        &format!(
            "rsync: source_path=`{source_path}`, dest_path=`{dest_path}`, recursive=`{recursive}`",
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

// TODO this should actually just call rsync non-recuresively, instead of calling a regular rsync
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
