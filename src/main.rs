// needs to be run as root, as to be able to stop the services (and perform a restart)

// TODO: this assumes that after this script has ran once systemd will call it again

mod args;
mod get_services;
mod get_users;
mod log;
mod reboot;
mod rsync;
mod start_services_if_enabled;
mod stop_services;
mod sync_filesystem;
mod sync_to_backup_server;
mod term;
mod update_distro_debian;
mod wait_until_its_time_to_work;

fn main() {
    let args = args::get();
    let error_folder = &args.error_folder;
    let do_update_distro_debian = args.update_server_debian; // TODO: ideally is this is False we would individually stop each service, then back it up to the backup server, then start it again, as to avoid downtime ALTHO this is no longer relevant now that I have added the option to not restart the server regardless

    wait_until_its_time_to_work::main(args.restart_at, args.check_time_sleep_sec, args.dry_run);

    let services = get_services::main(error_folder, &args.services_regex, &args.service_exception);
    let users = get_users::main(error_folder);

    stop_services::main(error_folder, args.dry_run, &services);
    sync_filesystem::main(error_folder, args.dry_run);

    sync_to_backup_server::main(
        error_folder,
        &args.backup_server_ip,
        &args.backup_server_user,
        args.dry_run,
        users,
    );

    if do_update_distro_debian {
        update_distro_debian::main(error_folder, args.dry_run);
        sync_filesystem::main(error_folder, args.dry_run);
        reboot::main(error_folder, args.dry_run);
    } else {
        start_services_if_enabled::main(error_folder, args.dry_run, &services);
    }
}
