use ::clap::{App, Arg};
use ::indoc::indoc;
use ::snet::ipv4::Network;
use ::std::convert::TryFrom;

const ARG_NAME_LIST_SNETS: &str = "list-snets";
const ARG_NAME_LIST_ALL: &str = "list-all";
const ARG_NAME_FMT_BINARY: &str = "binary";
const ARG_NAME_FMT_DECIMAL: &str = "decimal";

fn main() {
    let matches = App::new("snet4")
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
            Arg::with_name(ARG_NAME_FMT_BINARY)
                .short("b")
                .long(ARG_NAME_FMT_BINARY)
                .required(false)
                .help("Format output addresses in binary"),
        )
        .arg(
            Arg::with_name(ARG_NAME_FMT_DECIMAL)
                .short("d")
                .long(ARG_NAME_FMT_DECIMAL)
                .required(false)
                .help("Format output addresses in dotted decimal"),
        )
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

    let fmt_binary = matches.is_present(ARG_NAME_FMT_BINARY);
    let fmt_decimal = matches.is_present(ARG_NAME_FMT_DECIMAL);

    if matches.is_present(ARG_NAME_LIST_SNETS) {
        list_subnets(&network, fmt_binary, fmt_decimal);
    } else if matches.is_present(ARG_NAME_LIST_ALL) {
        list_all(&network, fmt_binary, fmt_decimal);
    } else {
        println!("{}", network);
    }
}

fn list_subnets(network: &Network, fmt_binary: bool, fmt_decimal: bool) {
    for address in network.subnets() {
        if fmt_binary {
            print!("{:b}", address);
        }
        if fmt_binary && fmt_decimal {
            print!(" - ")
        }
        if fmt_decimal || !fmt_binary {
            print!("{}", address);
        }
        println!();
    }
}

fn list_all(network: &Network, fmt_binary: bool, fmt_decimal: bool) {
    for address_type in network.addresses() {
        if fmt_binary {
            print!("{:b}", address_type.address());
        }
        if fmt_binary && fmt_decimal {
            print!(" - ")
        }
        if fmt_decimal || !fmt_binary {
            print!("{}", address_type.address());
        }
        println!(" ({})", address_type);
    }
}
