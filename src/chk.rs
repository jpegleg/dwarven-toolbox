use std::fs::File;
use std::io::Read;
use std::path::Path;
use chrono::{NaiveDateTime, DateTime, Utc};
use std::os::unix::fs::MetadataExt;
extern crate digest;

fn review(file_path: &str) {
    let file_path = Path::new(file_path);
    let metadata = file_path.metadata().expect("Failed to read file metadata");
    let mut file = File::open(&file_path).expect("Failed to open the file");
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).expect("Failed to read the file");
    let num_bytes = bytes.len();
    let chronox: String = Utc::now().to_string();
    println!("{{");
    println!("{:?}: {{", file_path);
    println!("  \"Report time\": \"{}\",", chronox.to_string());
    println!("  \"Number of bytes\": \"{}\",", &num_bytes);
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
    let mut hasher = blake3::Hasher::new();
    hasher.update(&bytes);
    let blake3 = hasher.finalize();
    println!("  \"BLAKE3\": \"{}\"", blake3);
    println!("  }}");
    println!("}}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("{{\"ERROR\": \"Wrong number of args. The only arg is a file path to review.\"}}");
        std::process::exit(1);
    }
    let data = args[1].clone();
    let sdata = String::from(data);
    review(&sdata);
}
