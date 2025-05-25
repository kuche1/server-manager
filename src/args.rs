use clap::Parser; // cargo add clap --features derive

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Folder to write errors to
    #[arg(long)]
    pub error_folder: String,

    /// At what hour the restart is going to occur, for example 15 for 15:00
    #[arg(short, long, default_value_t = 4)]
    pub restart_at: u8,

    /// Time to sleep if restart time has not been reached
    #[arg(long, default_value_t = 3000)] // 3000sec = 50min
    pub check_time_sleep_sec: u64,

    /// Stop all services with this prefix before restarting
    #[arg(short, long)]
    pub services_prefix: String,

    /// IP of the backup server
    #[arg(long)]
    pub backup_server_ip: String,

    /// user on the backup server
    #[arg(long)]
    pub backup_server_user: String,
}

pub fn get() -> Args {
    let args = Args::parse();

    if args.restart_at >= 24 {
        panic!(
            "invalid hour `{}`, needs to be less than 24",
            args.restart_at
        );
    }

    if args.restart_at == 0 {
        panic!("restarting at midnight is not supported, sorry");
    }

    args
}
