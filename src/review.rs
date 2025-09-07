use std::env;
use std::fs::File;
use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use chrono::{NaiveDateTime, DateTime, Utc};
use users::{get_user_by_uid, get_group_by_gid};
use std::os::unix::fs::MetadataExt;
use std::fs::OpenOptions;
use blake2::{Blake2b512, Digest};
use sha2::Sha256;
use sha3::{Sha3_256,Sha3_384};
extern crate blake2;
extern crate digest;
extern crate sha2;
extern crate sha3;

#[allow(deprecated)]
fn review(file_path: &str) {
    let file_path = Path::new(file_path);
    let metadata = file_path.metadata().expect("Failed to read file metadata");
    let mut file = File::open(&file_path).expect("Failed to open the file");
    let mut bytes = Vec::new();

    file.read_to_end(&mut bytes).expect("Failed to read the file");

    let num_bytes = bytes.len();
    let num_bits = num_bytes * 8;
    let byte_distribution = bytes.iter().collect::<std::collections::HashSet<_>>().len() as f64 / num_bytes as f64;

    let file_is_open = match OpenOptions::new()
        .read(true)
        .write(true)
        .open(file_path)
    {
        Ok(_) => false,
        Err(_) => true,
    };

    let chronox: String = Utc::now().to_string();
    println!("{{");
    println!("{:?}: {{", file_path);
    println!("  \"Report time\": \"{}\",", chronox.to_string());
    let num_io_blocks = metadata.blocks();
    println!("  \"Number of IO blocks\": \"{}\",", num_io_blocks);
    let blocksize = metadata.blksize();
    println!("  \"Block size\": \"{}\",", blocksize);
    let inode = metadata.ino();
    println!("  \"Inode\": \"{:?}\",", &inode);
    println!("  \"Number of bytes\": \"{}\",", &num_bytes);
    println!("  \"Number of kilobytes\": \"{}\",", num_bytes / 1024);
    println!("  \"Number of megabytes\": \"{}\",", num_bytes / (1024 * 1024));
    println!("  \"Number of bits\": \"{}\",", num_bits);
    println!("  \"Byte distribution\": \"{}\",", byte_distribution);
    let created: DateTime<Utc> = DateTime::from(metadata.created().expect("Failed to get created timestamp."));
    let modified: DateTime<Utc> = DateTime::from(metadata.modified().expect("Failed to get modified timestamp."));
    let accessed: DateTime<Utc> = DateTime::from(metadata.accessed().expect("Failed to get accessed timestamp."));
    let changed: DateTime<Utc> = {
        let ctime = metadata.ctime();
        let ctimesec = metadata.ctime_nsec() as u32;
        let naive_datetime = NaiveDateTime::from_timestamp_opt(ctime, ctimesec).expect("Invalid changed timestamp");
        DateTime::<Utc>::from_utc(naive_datetime, Utc)
    };

    println!("  \"Created timestamp (UTC)\": \"{}\",", created);
    println!("  \"Modified timestamp (UTC)\": \"{}\",", modified);
    println!("  \"Accessed timestamp (UTC)\": \"{}\",", accessed);
    println!("  \"Changed timestamp (UTC)\": \"{}\",", changed);

    let permissions = metadata.permissions();
    let mode = permissions.mode();
    println!("  \"Permissions\": \"{:o}\",", mode);

    let uid = metadata.uid();
    let gid = metadata.gid();

    let owner = match get_user_by_uid(uid) {
        Some(user) => user.name().to_string_lossy().into_owned(),
        None => "-".to_string(),
    };

    let group = match get_group_by_gid(gid) {
        Some(group) => group.name().to_string_lossy().into_owned(),
        None => "-".to_string(),
    };

    println!("  \"Owner\": \"{} (uid: {})\",", owner, uid);
    println!("  \"Group\": \"{} (gid: {})\",", group, gid);

    if file_is_open {
        println!("  \"Open\": \"File is currently open by another program...\",");
    } else {
        println!("  \"Open\": \"File is not open by another program.\",");
    }

    let mut hasher = blake3::Hasher::new();
    hasher.update(&bytes);
    let blake3 = hasher.finalize();
    println!("  \"BLAKE3\": \"{}\",", blake3);
    let mut hasher = Blake2b512::new();
    hasher.update(&bytes);
    let blake2b512 = hasher.finalize();
    println!("  \"BLAKE2B-512\": \"{:x}\",", blake2b512);
    let mut hasher = Sha3_256::new();
    hasher.update(&bytes);
    let sha3256 = hasher.finalize();
    println!("  \"SHA3-256\": \"{:x}\",", sha3256);
    let mut hasher = Sha3_384::new();
    hasher.update(&bytes);
    let sha3384 = hasher.finalize();
    println!("  \"SHA3-384\": \"{:x}\",", sha3384);
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let sha2 = hasher.finalize();
    println!("  \"SHA2\": \"{:x}\"", sha2);

    println!("  }}");
    println!("}}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("{{\"ERROR\": \"Wrong number of args. The only arg is a file path to review.\"}}");
        std::process::exit(1);
    }
    let data = &args[1];
    review(&data);
}
