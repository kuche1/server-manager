use clap::Parser; // cargo add clap --features derive

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Folder to write errors to
    #[arg(long)]
    error_folder: String,

    /// At what hour the restart is going to occur, for example 15 for 15:00
    #[arg(short, long, default_value_t = 4)]
    restart_at: u8,

    /// Time to sleep if restart time has not been reached
    #[arg(long, default_value_t = 3600)]
    check_time_sleep_sec: u64,

    /// Stop all services with this prefix
    #[arg(short, long)]
    prefix: String,
}

fn main() {
    let args = Args::parse();

    if args.restart_at >= 24 {
        panic!(
            "invalid hour `{}`, needs to be less than 24",
            args.restart_at
        );
    }
}
