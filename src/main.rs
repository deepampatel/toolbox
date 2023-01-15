use clap::{App, Arg, SubCommand};
use uuid::Uuid;
use openssl::hash::{hash, MessageDigest};
use rpassword::prompt_password;

fn main() {
    let matches = App::new("rucksack")
        .version("0.1.0")
        .author("Your Name <you@example.com>")
        .about("A utility CLI for generating UUIDs, hashing passwords, and storing and quickly accessing one-liner commands")
        .subcommand(SubCommand::with_name("generate")
            .about("generate UUID")
            .arg(Arg::with_name("count")
                 .help("Number of UUIDs to generate")
                 .default_value("1")
                 .long("count")))
        .subcommand(SubCommand::with_name("hash")
            .about("hash password")
            .arg(Arg::with_name("method")
                .help("Hashing method to use (md5, sha1, sha256, sha512)")
                .required(true)
                .index(1))
            .arg(Arg::with_name("salt")
                .help("Salt to use for hashing (leave empty for random salt)")
                .long("salt")
                .takes_value(true)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("generate") {
        let count: u32 = matches.value_of("count").unwrap().parse().unwrap();
        for _i in 0..count {
            let uuid = Uuid::new_v4();
            println!("UUID: {}", uuid);
        }
    }

    // if let Some(matches) = matches.subcommand_matches("hash") {
    //     let salt = match matches.value_of("salt") {
    //         Some(salt) => salt.as_bytes(),
    //         None => openssl::crypto::rand::rand_bytes(8),
    //     };
    //     let password = rpassword::prompt_password("Password: ")?;
    //     let method = matches.value_of("method").unwrap();

    //     let hashed_password = match method {
    //         "md5" => hash(MessageDigest::md5(), password.as_bytes()).unwrap(),
    //         "sha1" => hash(MessageDigest::sha1(), password.as_bytes()).unwrap(),
    //         "sha256" => hash(MessageDigest::sha256(), password.as_bytes()).unwrap(),
    //         "sha512" => hash(MessageDigest::sha512(), password.as_bytes()).unwrap(),
    //         _ => panic!("Invalid hashing method"),
    //     };

    //     // Digest bytes array to hex string
    //     let hashed_password = hashed_password.iter().map(|b| format!("{:02x}", b)).collect::<String>();

    //     println!("Hashed password: {}", hashed_password);

    // }

}


// To run 
// cargo run -- generate --count 5
// cargo run -- hash password