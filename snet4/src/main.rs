use ::clap::{App, Arg};
use ::snet::ipv4::Network;
use ::std::convert::TryFrom;

fn main() {
    let matches = App::new("snet4")
        .version("1.0")
        .about(
            "snet4 provides subnet information for an IPv4 network, and optionally lists all hosts, \
             subnet addresses and broadcast addresses. Useful when subnets cross the octet \
             boundary and subnet ranges become less obvious.",
        )
        .arg(
            Arg::with_name("list-subnets")
                .short("s")
                .long("list-subnets")
                .required(false)
                .help("List all subnet network addresses")
                .conflicts_with("list-addresses"))
        .arg(
            Arg::with_name("list-addresses")
                .short("a")
                .long("list-addresses")
                .required(false)
                .help("List all host addresses, subnets, and broadcast addresses")
                .conflicts_with("list-subnets"))
        .arg(
            Arg::with_name("network")
                .index(1)
                .required(true)
                .help("Network address in CIDR notation (e.g. 192.168.147.0/28)"))
        .get_matches();

    let network = match Network::try_from(matches.value_of("network").unwrap()) {
        Err(e) => panic!("{}", e),
        Ok(network) => network,
    };

    if matches.is_present("list-subnets") {
        for address in network.subnets() {
            println!("{}", address);
        }
    } else if matches.is_present("list-addresses") {
        for address in network.addresses() {
            println!("{}", address);
        }
    } else {
        println!("{}", network);
    }
}
