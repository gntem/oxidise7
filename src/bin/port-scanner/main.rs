use clap::Parser;
use std::net::{TcpStream, ToSocketAddrs};
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const N_THREADS: usize = 200;

fn port_scan(host: &str, port: u16) -> bool {
    let addr = format!("{}:{}", host, port);
    if let Ok(_) = TcpStream::connect_timeout(
        &addr.to_socket_addrs().unwrap().next().unwrap(),
        Duration::from_secs(1),
    ) {
        println!("{}:{} is open", host, port);
        true
    } else {
        false
    }
}

fn port_scan_thread(queue: Arc<Mutex<Vec<u16>>>, host: String) {
    loop {
        let port = {
            let mut queue = queue.lock().unwrap();
            if queue.is_empty() {
                break;
            }
            queue.pop().unwrap()
        };
        port_scan(&host, port);
    }
}

fn scan(host: &str, ports: Vec<u16>) {
    let queue = Arc::new(Mutex::new(ports));
    let mut handles = vec![];

    for _ in 0..N_THREADS {
        let queue = Arc::clone(&queue);
        let host = host.to_string();
        let handle = thread::spawn(move || {
            port_scan_thread(queue, host);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    hostname: String,

    #[arg(short, long)]
    start: String,

    #[arg(short, long)]
    end: String,
}

fn main() {
    let args = Args::parse();

    if (args.start.parse::<u16>().is_err()) || (args.end.parse::<u16>().is_err()) {
        eprintln!("invalid port values");
        process::exit(1);
    }

    let host = &args.hostname;

    if host.parse::<std::net::IpAddr>().is_err() {
        eprintln!("invalid hostname");
        process::exit(1);
    }

    let start_port = args.start.parse::<u16>().unwrap();
    let end_port = args.end.parse::<u16>().unwrap();

    if start_port > end_port {
        eprintln!("invalid port range, start port is greater than end port");
        process::exit(1);
    }

    if start_port < 1 || end_port < 1 {
        eprintln!("invalid port range, port values should be between 1 and 65535");
        process::exit(1);
    }

    println!("scanning host: {}", host);
    println!("time started: {}\n", chrono::Local::now());

    let ports: Vec<u16> = (start_port..end_port).collect();

    scan(host, ports);
}

#[cfg(test)]
mod tests {
    use std::net::TcpListener;

    use super::*;

    #[test]
    fn test_port_scan() {
        // open tcp port 80 on localhost
        let server = thread::spawn(|| {
            let listener = TcpListener::bind("127.0.0.1:80").unwrap();
            listener.accept().unwrap();
        });

        thread::sleep(Duration::from_secs(1));

        let p80 = port_scan("127.0.0.1", 80);
        let p81 = port_scan("127.0.0.1", 81);

        server.join().unwrap();

        assert!(p80);
        assert!(!p81);
    }
}
