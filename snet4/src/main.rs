use ::clap::{App, Arg};
use ::indoc::indoc;
use ::snet::ipv4::Network;
use ::std::convert::TryFrom;

const ARG_NAME_LIST_SNETS: &str = "list-snets";
const ARG_NAME_LIST_ALL: &str = "list-all";

fn main() {
    let matches = App::new("snet4")
        .version("1.0")
        .about(indoc! {"
            snet4 provides subnet information about an IPv4 network, and optionally
            lists all hosts, subnet addresses and broadcast addresses.

            In many cases, IPv4 network information is fairly obvious when presented
            with dotted decimal notation, but subnets which cross the octet boundary
            are much less obvious. This is where snet4 can help.

            When invoked without arguments, snet4 will determine the class of the
            network, number of subnets and hosts per subnet.
        "})
        .arg(
            Arg::with_name(ARG_NAME_LIST_SNETS)
                .short("s")
                .long(ARG_NAME_LIST_SNETS)
                .required(false)
                .help("List all base subnet network addresses")
                .conflicts_with(ARG_NAME_LIST_ALL),
        )
        .arg(
            Arg::with_name(ARG_NAME_LIST_ALL)
                .short("a")
                .long(ARG_NAME_LIST_ALL)
                .required(false)
                .help(indoc! {"
                    Lists all network address, network broadcast
                    addresses, subnet addresses, subnet broadcast
                    addresses and host addresses within each
                    subnet.
                "})
                .conflicts_with(ARG_NAME_LIST_SNETS),
        )
        .arg(
            Arg::with_name("network")
                .index(1)
                .required(true)
                .help("Network address in CIDR notation (e.g. 192.168.13.160/28)"),
        )
        .get_matches();

    let network = match Network::try_from(matches.value_of("network").unwrap()) {
        Err(e) => panic!("{}", e),
        Ok(network) => network,
    };

    if matches.is_present(ARG_NAME_LIST_SNETS) {
        for address in network.subnets() {
            println!("{}", address);
        }
    } else if matches.is_present(ARG_NAME_LIST_ALL) {
        for address in network.addresses() {
            println!("{}", address);
        }
    } else {
        println!("{}", network);
    }
}
