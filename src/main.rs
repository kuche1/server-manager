// needs to be run as root, as to be able to stop the services (and perform a restart)

mod args;
mod get_services;
mod log;
mod reboot;
mod rsync;
mod stop_services;
mod sync_filesystem;
mod sync_to_backup_server;
mod term;
mod wait_until_its_time_to_restart;

fn main() {
    let args = args::get();
    let error_folder = &args.error_folder;

    wait_until_its_time_to_restart::main(args.restart_at, args.check_time_sleep_sec);

    let services = get_services::main(error_folder, &args.services_regex);

    stop_services::main(error_folder, services);

    sync_to_backup_server::main(
        error_folder,
        &args.backup_server_ip,
        &args.backup_server_user,
    );

    sync_filesystem::main(error_folder);

    reboot::main(error_folder);
}
