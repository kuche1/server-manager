// needs to be run as root, as to be able to stop the services (and perform a restart)

mod args;
mod get_services;
mod log;
mod reboot;
mod stop_services;
mod sync_filesystem;
mod wait_until_its_time_to_restart;

fn main() {
    let args = args::get();
    let error_folder = &args.error_folder;

    wait_until_its_time_to_restart::main(args.restart_at, args.check_time_sleep_sec);

    let services = get_services::main(error_folder, &args.services_prefix);

    stop_services::main(error_folder, services);

    //////
    ////// TODO sync to server
    //////

    {
        // ping server

        // copy services

        // copy files
    }

    //////
    ////// sync filesystem
    //////

    sync_filesystem::main(error_folder);

    reboot::main();
}
