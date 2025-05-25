use chrono::NaiveTime; // cargo add chrono
use clap::{Parser, error}; // cargo add clap --features derive
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::thread;
use std::time::Duration;

const SERVICE_SUFFIX: &str = ".service";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Folder to write errors to
    #[arg(long)]
    error_folder: String,

    /// At what hour the restart is going to occur, for example 15 for 15:00
    #[arg(short, long, default_value_t = 4)]
    restart_at: u8,

    /// Time to sleep if restart time has not been reached
    #[arg(long, default_value_t = 3000)] // 3000sec = 50min
    check_time_sleep_sec: u64,

    /// Stop all services with this prefix before restarting
    #[arg(short, long)]
    services_prefix: String,
}

// untested
fn logerr(error_folder: &String, msg: &str) {
    eprintln!("ERROR: {msg}");

    if let Err(err) = fs::create_dir_all(error_folder) {
        eprintln!("ERROR: could not create error folder: {}", err);
        return;
    }

    let now = chrono::offset::Local::now();
    let file_name = now.format("%Y-%m-%d_%H-%M-%S-%f"); // %f - nanoseconds

    let mut f = match File::options()
        .append(true)
        .create(true)
        .open(format!("{}/{}", error_folder, file_name))
    {
        Ok(f) => f,
        Err(err) => {
            eprintln!("ERROR: could not create error file: {}", err);
            return;
        }
    };

    if let Err(err) = writeln!(&mut f, "{}", msg) {
        eprintln!("ERROR: could not write to error file: {}", err);
    }
}

fn main() {
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

    let error_folder = &args.error_folder;

    // {
    //     // wait until it's time to restart

    //     let restart_at = args.restart_at;
    //     let sleep_sec = args.check_time_sleep_sec;

    //     let target = NaiveTime::from_hms_opt(restart_at.into(), 0, 0).unwrap();

    //     loop {
    //         let now = chrono::offset::Local::now().time();

    //         println!("{target} >? {now}");

    //         if now > target {
    //             println!("too late for a restart; sleeping {} sec", sleep_sec);
    //             thread::sleep(Duration::from_secs(sleep_sec));
    //         } else {
    //             break;
    //         }
    //     }

    //     loop {
    //         let now = chrono::offset::Local::now().time();

    //         println!("{target} <? {now}");

    //         if now < target {
    //             println!("too early for a restart; sleeping {} sec", sleep_sec);
    //             thread::sleep(Duration::from_secs(sleep_sec));
    //         } else {
    //             break;
    //         }
    //     }
    // }

    let services = 'get_services: {
        let cmd = match Command::new("systemctl")
            .args([
                "list-units",
                "--type=service",
                "--all",
                "--no-legend",
                "--plain",
            ])
            .output()
        {
            Ok(v) => v,
            Err(err) => {
                logerr(
                    error_folder,
                    &format!("could not call systemd to get services: {}", err),
                );
                break 'get_services vec![];
            }
        };

        if !cmd.status.success() {
            logerr(
                error_folder,
                &format!(
                    "called systemd, but failure occured: {}; stderr=`{}`",
                    cmd.status,
                    String::from_utf8_lossy(&cmd.stderr)
                ),
            )
        }

        let data = cmd.stdout;
        let data = String::from_utf8_lossy(&data);

        let mut services = vec![];

        for line in data.lines() {
            let idx = match line.find(SERVICE_SUFFIX) {
                Some(v) => v,
                None => {
                    logerr(
                        error_folder,
                        &format!(
                            "could not find suffix `{}` on line `{}`",
                            SERVICE_SUFFIX, line
                        ),
                    );
                    continue;
                }
            };

            let service_name = &line[0..idx + SERVICE_SUFFIX.len()];

            services.push(service_name.to_owned());
        }

        services
    };

    for service in services {
        println!("service: {service}");
    }
}
