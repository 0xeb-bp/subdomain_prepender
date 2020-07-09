//subdomain_prepender written by @0xeb_bp

use std::io::{self, BufReader, BufWriter};
use std::io::prelude::*;
use std::fs::File;

extern crate clap;
use clap::{App, Arg};


fn main() -> io::Result<()> {

    let args = App::new("subdomain_prepender")
    .about("Prepends a list of subdomains from list of domains to a specified output file.")
    .version("0.2")
    .author("written by raiz_")
    .arg(
        Arg::with_name("domains file")
            .help("List of domains")
            .short("d")
            .long("domains")
            .takes_value(true)
            .required(true),
    )
    .arg(
        Arg::with_name("subdomains file")
            .help("List of subdomains")
            .short("s")
            .long("subdomains")
            .takes_value(true)
            .required(true),
    )
    .arg(
        Arg::with_name("output file")
            .help("Output destination")
            .short("o")
            .long("output")
            .takes_value(true)
            .required(true),
    )
    .get_matches();

    let domains_file = args.value_of("domains file").unwrap();
    let subdomains_file = args.value_of("subdomains file").unwrap();
    let output_file = args.value_of("output file").unwrap();
   
    let domains = File::open(domains_file)?;
    let domains = BufReader::new(domains);

    let output = File::create(output_file)?;
    let mut output = BufWriter::new(output);

    for domain in domains.lines() {
        let subdomains = File::open(subdomains_file)?;
        let subdomains = BufReader::new(subdomains);
        let domain = domain?;

        for subdomain in subdomains.lines() {
                writeln!(output, "{}.{}", subdomain?, domain)?;
        }
    }

    output.flush().unwrap();

    Ok(())
}
