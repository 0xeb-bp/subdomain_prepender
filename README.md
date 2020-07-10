# subdomain_prepender
Subdomain prepender for OSINT and bug bounty

## Usage 

Install Rust if you need it https://www.rust-lang.org/learn/get-started

Compile the code:

`cargo build --release`

Execute the binary from `target/release/`

```
Prepends a list of subdomains from list of domains to a specified output file.

USAGE:
    subdomain_prepender --domains <domains file> --output <output file> --subdomains <subdomains file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --domains <domains file>          List of domains
    -o, --output <output file>            Output destination
    -s, --subdomains <subdomains file>    List of subdomains
```
