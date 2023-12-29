use clap::{Arg, Command};
use nix::errno;
use nix::sys::statvfs;
use std::path::PathBuf;

struct AppArgs {
    path: PathBuf,
    poll_interval: u32,
}

struct DiskSpace {
    free: u64,
    used: u64,
    total: u64,
}

fn main() {
    let args = match AppArgs::new() {
        Some(args) => args,
        None => {
            eprintln!("Error: Invalid arguments");
            std::process::exit(1);
        }
    };

    let poll_interval = std::time::Duration::from_millis(args.poll_interval.into());

    loop {
        match get_disk_space(&args.path) {
            Ok(disk_space) => print_disk_space(disk_space),
            Err(err) => eprintln!("Failed to get disk space: {}", err),
        }
        std::thread::sleep(poll_interval);
    }
}

impl AppArgs {
    fn new() -> Option<Self> {
        let matches = Command::new("yambar-disk")
            .version("1.0.0")
            .about("Disk space module for Yambar")
            .arg(
                Arg::new("path")
                    .long("path")
                    .value_name("PATH")
                    .help("Path to any file or directory on the mounted disk to monitor")
                    .default_value("/"),
            )
            .arg(
                Arg::new("poll-interval")
                    .long("poll-interval")
                    .value_name("POLL_INTERVAL")
                    .help("Interval between updates in milliseconds")
                    .value_parser(clap::value_parser!(u32))
                    .default_value("1000"),
            )
            .get_matches();

        let path = matches.get_one::<String>("path")?;
        let poll_interval = matches.get_one::<u32>("poll-interval")?;

        Some(AppArgs {
            path: PathBuf::from(path),
            poll_interval: *poll_interval,
        })
    }
}

impl DiskSpace {
    fn from_statvfs(stat: statvfs::Statvfs) -> Self {
        DiskSpace {
            free: stat.blocks_free() * stat.fragment_size(),
            used: (stat.blocks() - stat.blocks_free()) * stat.fragment_size(),
            total: stat.blocks() * stat.fragment_size(),
        }
    }
}

fn get_disk_space(path: &PathBuf) -> Result<DiskSpace, errno::Errno> {
    match statvfs::statvfs(path.as_path()) {
        Ok(stat) => Ok(DiskSpace::from_statvfs(stat)),
        Err(err) => Err(err),
    }
}

fn print_disk_space(disk_space: DiskSpace) {
    println!("free|int|{}", disk_space.free);
    println!("used|int|{}", disk_space.used);
    println!("total|int|{}", disk_space.total);
    println!(
        "percent_free|range:0-100|{}",
        (disk_space.free as f64 / disk_space.total as f64 * 100.0) as u64
    );
    println!(
        "percent_used|range:0-100|{}",
        (disk_space.used as f64 / disk_space.total as f64 * 100.0) as u64
    );
    println!();
}
