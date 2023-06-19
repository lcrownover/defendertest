use std::fs;

use anyhow::{bail, Result};
use clap::Parser;
use pbr::ProgressBar;
use std::io::Stdout;
use std::io::Write;
use std::thread;
use std::time::Duration;

use human_time::ToHumanTimeString;

/// Creates test data for defender testing
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to create the test data at
    #[arg(short, long)]
    path: String,

    /// Total number of 1B inodes to create
    #[arg(short, long, default_value_t = 1000000)]
    total_inodes: u64,

    /// How how deep to create subdirectories
    #[arg(short, long, default_value_t = 20)]
    depth: u64,
}

/// Given a root directory, create a subdirectory, then fill that subdirectory
/// with the specified number of 1B files.
fn create_inode_dir(pb: &mut ProgressBar<Stdout>, root_dir: &str, inodes: u64) -> Result<String> {
    let dir_name = uuid::Uuid::new_v4().to_string();
    let inode_dir_path = format!("{}/{}", root_dir, &dir_name);
    if fs::metadata(&inode_dir_path).is_err() {
        if fs::create_dir(&inode_dir_path).is_err() {
            bail!("Failed to create inode directory {}", &inode_dir_path);
        }
    }

    let mut created: Vec<String> = Vec::new();
    for _ in 0..inodes {
        pb.inc();
        let filename: String;
        loop {
            let u = uuid::Uuid::new_v4().to_string();
            if created.contains(&u) {
                continue;
            }
            filename = u;
            created.push(filename.clone());
            break;
        }
        let inode_filepath = format!("{}/{}", &inode_dir_path, &filename);
        if fs::metadata(&inode_filepath).is_err() {
            match fs::File::create(&inode_filepath) {
                Ok(mut f) => {
                    if f.write_all(&"X".as_bytes()[..1]).is_err() {
                        bail!("Failed to write to inode file {}", &inode_filepath)
                    }
                }
                Err(_) => bail!("Failed to create inode file {}", &inode_filepath),
            }
        }
    }

    Ok(inode_dir_path.into())
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Check if the base path exists
    if fs::metadata(&args.path).is_err() {
        bail!("Provided path {} does not exist", &args.path);
    }

    let root_dir = format!("{}/{}", &args.path, "defendertest_data");
    if fs::metadata(&root_dir).is_err() {
        if fs::create_dir(&root_dir).is_err() {
            bail!("Failed to create root directory {}", &root_dir);
        }
    }

    let start_time = std::time::Instant::now();

    // Use the root_dir as a base and recursively create subdirectories
    let mut current_inodes = 0;
    let total_inodes = args.total_inodes;
    let inodes_per_dir = total_inodes / args.depth;

    let mut next_dir = root_dir.clone();

    let mut pb = ProgressBar::new(total_inodes);
    pb.format("╢▌▌░╟");

    loop {
        if current_inodes >= total_inodes {
            break;
        }
        pb.inc();
        thread::sleep(Duration::from_millis(200));
        next_dir = create_inode_dir(&mut pb, &next_dir, inodes_per_dir)?;

        current_inodes += inodes_per_dir;
    }
    pb.finish_print("Process complete");
    let elapsed = start_time.elapsed().as_millis();

    println!("Total inodes created: {}", total_inodes);
    println!("Elapsed time: {}", Duration::from_millis(elapsed as u64).to_human_time_string());
    println!("Time per inode: {:.8}ms", elapsed as f64 / 1000.0 / total_inodes as f64);

    Ok(())
}