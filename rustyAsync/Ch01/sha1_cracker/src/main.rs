use sha1::Digest;

use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    };

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
        return Ok(());
    }

    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash not valid".into());
    }

    // for the purpose of experiment wordlist is borrowed from,
    // https://github.com/skerkour/black-hat-rust/blob/main/ch_01/sha1_cracker/wordlist.txt
    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);

    for line in reader.lines() {
        // let line = line?.trim().to_string();
        let line = line?;
        let common_password = line.trim();
        if hash_to_crack == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
            println!("Password found: {}", &common_password);
            return Ok(());
        }
    }
    println!("Password not found in wordlist :/ ");

    Ok(())
}
