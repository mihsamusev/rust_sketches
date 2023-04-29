use sha1::Digest;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    str::Bytes,
};

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn to_sha1(text: &str) -> String {
    hex::encode(sha1::Sha1::digest(text.trim().as_bytes()))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
        return Ok(());
    }

    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not of valid length".into());
    }

    let password_file = File::open(&args[1])?;
    let reader = BufReader::new(password_file);

    let valid_lines: Vec<String> = reader
        .lines()
        .filter_map(|r| r.ok())
        .map(|s| to_sha1(&s))
        .collect();
    println!("{:?}", valid_lines);

    Ok(())
}
