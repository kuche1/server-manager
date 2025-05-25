use crate::log;

use std::process::Command;

pub fn exec(error_folder: &String, command: &str, args: Vec<&str>, info: &str) -> Option<()> {
    let cmd = match Command::new(command).args(args).output() {
        Ok(v) => v,
        Err(err) => {
            log::err(
                error_folder,
                &format!("could not call `{command}` ({info}) -> {err}"),
            );
            return None;
        }
    };

    if !cmd.status.success() {
        log::err(
            error_folder,
            &format!(
                "call to `{}` ({info}) failed -> status=`{}`, stderr=`{}`",
                command,
                cmd.status,
                String::from_utf8_lossy(&cmd.stderr)
            ),
        );
        return None;
    }

    return Some(());
}
