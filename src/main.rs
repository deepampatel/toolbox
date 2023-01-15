use clap::{App, Arg, SubCommand};
use uuid::Uuid;
use openssl::hash::{hash, MessageDigest};

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
            .arg(Arg::with_name("password")
                 .help("Password to hash")
                 .required(true)
                 .index(1)))
        .subcommand(SubCommand::with_name("store")
            .about("store command")
            .arg(Arg::with_name("name")
                 .help("Name of the command to store")
                 .required(true)
                 .index(1))
            .arg(Arg::with_name("command")
                 .help("Command to store")
                 .required(true)
                 .index(2)))
        .subcommand(SubCommand::with_name("get")
            .about("get stored command")
            .arg(Arg::with_name("name")
                 .help("Name of the command to get")
                 .required(true)
                 .index(1)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("generate") {
        let count: u32 = matches.value_of("count").unwrap().parse().unwrap();
        for _i in 0..count {
            let uuid = Uuid::new_v4();
            println!("UUID: {}", uuid);
        }
    }

    if let Some(matches) = matches.subcommand_matches("hash") {
        let password = matches.value_of("password").unwrap();
        // hash password
        let data = b"\x42\xF4\x97\xE0";
        let res = hash(MessageDigest::md5(), data).unwrap();
        // res string
        let res_str = format!("{:x}", res);
        println!("Hash: {}", res_str);

    }

    if let Some(matches) = matches.subcommand_matches("store") {
        let name = matches.value_of("name").unwrap();
        let command = matches.value_of("command").unwrap();
        println!("Name: {}", name);
        println!("Command: {}", command);
    }

}


// To run 
// cargo run -- generate --count 5
// cargo run -- hash password