use clap::Parser;
use trust_dns_resolver::{config::{ResolverConfig, ResolverOpts}, Resolver};

const RECORDS: [&str; 7] = ["A", "AAAA", "CNAME", "MX", "NS", "SOA", "TXT"];

fn enumerate(domain: &str) {
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

    for record in &RECORDS {
        match resolver.lookup(domain, record.parse().unwrap()) {
            Ok(response) => {
                println!("\n{} records for {}:", record, domain);
                for answer in response.iter() {
                    println!("\t{}", answer);
                }
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    domain: String,
}

fn main() {
    let args = Args::parse();

    let domain = &args.domain;

    println!("Enumeration DNS for: {}", domain);

    enumerate(domain);
}
