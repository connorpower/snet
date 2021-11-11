use crate::{Error, Result};
use ::std::iter::once;
use ::std::{
    convert::TryFrom,
    fmt::{Binary, Debug, Display},
};
use ::strum::{Display, EnumCount};

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumCount)]
pub enum Class {
    /// N, H, H, H
    #[strum(serialize = "class A network")]
    A,
    /// N, N, H, H
    #[strum(serialize = "class B network")]
    B,
    /// N, N, N, H
    #[strum(serialize = "class C network")]
    C,
    /// Multicast
    #[strum(serialize = "class D network")]
    D,
    /// Reserved Use
    #[strum(serialize = "class E network")]
    E,
}

impl Class {
    /// All network classes, in order of longest bit pattern first.
    const DISCRIMINANTS: [Self; Self::COUNT] = [Self::E, Self::D, Self::C, Self::B, Self::A];

    const fn network_bits(&self) -> Option<u8> {
        match self {
            Self::A => Some(1 * 8),
            Self::B => Some(2 * 8),
            Self::C => Some(3 * 8),
            Self::D => None,
            Self::E => None,
        }
    }

    fn mask(&self) -> u32 {
        match self {
            Self::A => 0b10000000_00000000_00000000_00000000,
            Self::B => 0b11000000_00000000_00000000_00000000,
            Self::C => 0b11100000_00000000_00000000_00000000,
            Self::D => 0b11110000_00000000_00000000_00000000,
            Self::E => 0b11110000_00000000_00000000_00000000,
        }
    }

    fn pattern(&self) -> u32 {
        match self {
            Self::A => 0b00000000_00000000_00000000_00000000,
            Self::B => 0b10000000_00000000_00000000_00000000,
            Self::C => 0b11000000_00000000_00000000_00000000,
            Self::D => 0b11100000_00000000_00000000_00000000,
            Self::E => 0b11110000_00000000_00000000_00000000,
        }
    }
}

impl From<u32> for Class {
    fn from(address: u32) -> Self {
        for class in &Self::DISCRIMINANTS {
            if address & class.mask() == class.pattern() {
                return *class;
            }
        }
        unreachable!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumCount)]
pub enum ReservedAddress {
    /// 127/8 reserved loopback (note that this is /8, and not just the
    /// commonly used 127.0.0.1/32 address))
    #[strum(serialize = "loopback")]
    Loopback,

    /// 255.255.255.255/32
    #[strum(serialize = "local broadcast")]
    LocalBroadcast,
}

impl TryFrom<u32> for ReservedAddress {
    type Error = ();

    fn try_from(address: u32) -> ::std::result::Result<ReservedAddress, Self::Error> {
        for reserved_address in &Self::DISCRIMINANTS {
            if address & reserved_address.mask() == reserved_address.pattern() {
                return Ok(*reserved_address);
            }
        }
        Err(())
    }
}

impl ReservedAddress {
    /// All network classes, in order of longest bit pattern first.
    const DISCRIMINANTS: [Self; Self::COUNT] = [Self::LocalBroadcast, Self::Loopback];

    fn mask(&self) -> u32 {
        match self {
            Self::Loopback => 0b11111111_00000000_00000000_00000000,
            Self::LocalBroadcast => 0b11111111_11111111_11111111_11111111,
        }
    }

    fn pattern(&self) -> u32 {
        match self {
            Self::Loopback => 0b01111111_00000000_00000000_00000000,
            Self::LocalBroadcast => 0b11111111_11111111_11111111_11111111,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Address(u32);

impl Debug for Address {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let str = format!("{:032b} - {:>15}", self, self);
        write!(f, "{}", str)?;

        if let Some(width) = f.width() {
            let c = f.fill();
            for _ in 0..width.saturating_sub(str.len()) {
                write!(f, "{}", c)?;
            }
        }

        Ok(())
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_fmt(format_args!(
            "{}.{}.{}.{}",
            self.0 >> 3 * 8 & 0xFF,
            self.0 >> 2 * 8 & 0xFF,
            self.0 >> 1 * 8 & 0xFF,
            self.0 >> 0 * 8 & 0xFF,
        ))
    }
}

impl Binary for Address {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_fmt(format_args!("{:032b}", self.0))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddressType {
    Network(Address, Class),
    Subnet(Address),
    Host(Address),
    SubnetBroadcast(Address),
    NetworkBroadcast(Address),
}

impl AddressType {
    pub const fn address(&self) -> Address {
        match self {
            Self::Network(a, _)
            | Self::Subnet(a)
            | Self::Host(a)
            | Self::SubnetBroadcast(a)
            | Self::NetworkBroadcast(a) => *a,
        }
    }
}

impl Display for AddressType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Network(_, c) => write!(f, "{}", c),
            Self::Subnet(_) => write!(f, "subnet"),
            Self::Host(_) => write!(f, "host"),
            Self::SubnetBroadcast(_) => write!(f, "subnet broadcast"),
            Self::NetworkBroadcast(_) => write!(f, "network broadcast"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Network {
    /// The base network address.
    address: u32,

    /// The length of the subnet mask, in bits.
    subnet_mask_len: u8,
}

impl Display for Network {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_fmt(format_args!(
            "{}\n\
            Subnets:      {}\n\
            Hosts/subnet: {}",
            self.class(),
            self.num_subnets()
                .map(|n| n.to_string())
                .unwrap_or("N/A".to_string()),
            self.num_subnets()
                .map(|_| self.num_hosts_per_subnet().to_string())
                .unwrap_or("N/A".to_string())
        ))
    }
}

impl TryFrom<&str> for Network {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let (octals, mask) = {
            let mut parts = value.split('/');
            (
                parts
                    .next()
                    .ok_or(Error::InvalidAddress)?
                    .split('.')
                    .map(|s| s.parse().map_err(|_| Error::InvalidAddress))
                    .collect::<Result<Vec<_>, _>>()?,
                parts
                    .next()
                    .ok_or(Error::InvalidAddress)?
                    .parse()
                    .map_err(|_| Error::InvalidAddress)?,
            )
        };
        if octals.len() != 4 {
            return Err(Error::InvalidAddress);
        }

        Self::from_dotted_decimal_parts(octals[0], octals[1], octals[2], octals[3], mask)
    }
}

impl Network {
    fn from_dotted_decimal_parts(
        octal1: u8,
        octal2: u8,
        octal3: u8,
        octal4: u8,
        subnet_mask_len: u8,
    ) -> Result<Self> {
        if subnet_mask_len > 32 {
            return Err(Error::InvalidSubnetMask);
        }

        let address = (octal1 as u32) << 3 * 8
            | (octal2 as u32) << 2 * 8
            | (octal3 as u32) << 1 * 8
            | (octal4 as u32) << 0 * 8;

        if let Ok(reserved) = ReservedAddress::try_from(address) {
            return Err(Error::ReservedAddress(reserved));
        }

        Ok(Self {
            address,
            subnet_mask_len,
        })
    }

    pub fn class(&self) -> Class {
        Class::from(self.address)
    }

    pub fn net_mask(&self) -> Option<u32> {
        self.class()
            .network_bits()
            .map(|network_len| !0x0 << (32 - network_len))
    }

    pub fn subnet_mask(&self) -> u32 {
        if self.subnet_mask_len == 0 {
            0x0
        } else {
            !0x0 << (32 - self.subnet_mask_len)
        }
    }

    pub fn num_subnets(&self) -> Option<u32> {
        let net_mask = match self.net_mask() {
            Some(n) => n,
            None => return None,
        };
        let subnet_mask = self.subnet_mask();

        Some(if subnet_mask == 0 {
            0
        } else {
            ((net_mask ^ subnet_mask) >> subnet_mask.count_zeros()) - 1
        })
    }

    pub fn num_hosts_per_subnet(&self) -> u32 {
        if self.subnet_mask_len > 30 {
            0
        } else if self.subnet_mask_len == 0 {
            !self.net_mask().unwrap_or_else(|| self.class().mask()) - 1
        } else {
            (!0x0 >> self.subnet_mask_len) - 1
        }
    }

    pub fn subnets(&self) -> impl Iterator<Item = Address> {
        let network_address = self.address;
        let n_subnets = self.num_subnets().unwrap_or(0);
        let shift = 32 - self.subnet_mask_len;

        (1..=n_subnets).map(move |i| Address(network_address | (i << shift)))
    }

    pub fn addresses(&self) -> Box<dyn Iterator<Item = AddressType>> {
        let net_address = self.address;
        let class = self.class();
        let net_iter = once(AddressType::Network(Address(net_address), class));

        let subnet_mask = self.subnet_mask();
        let net_mask = match self.net_mask() {
            Some(m) => m,
            None => return Box::new(net_iter),
        };

        // Total number of host and subnet addresses
        let num_addresses = !net_mask - (2 * !subnet_mask);
        let net_broadcast_iter = once(AddressType::NetworkBroadcast(Address(
            net_address | !net_mask,
        )));

        let hosts = (1..num_addresses).map(move |i| {
            let address = Address(net_address + !subnet_mask + i);

            if address.0 | subnet_mask == !0x0 {
                AddressType::SubnetBroadcast(address)
            } else if !address.0 | subnet_mask == !0x0 {
                AddressType::Subnet(address)
            } else {
                AddressType::Host(address)
            }
        });

        Box::new(net_iter.chain(hosts).chain(net_broadcast_iter))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ::assert_matches::assert_matches;

    #[test]
    fn test_network_from_string() {
        let network_from_str = Network::try_from("192.168.147.0/28").unwrap();
        let network_from_parts = Network::from_dotted_decimal_parts(192, 168, 147, 0, 28).unwrap();

        assert_eq!(network_from_str, network_from_parts);
    }

    #[test]
    fn test_network_from_decimal_parts() {
        let network = Network::from_dotted_decimal_parts(192, 168, 147, 0, 28).unwrap();
        assert_eq!(network.address, 0b11000000_10101000_10010011_00000000);
        assert_eq!(network.subnet_mask_len, 28);
    }

    #[test]
    fn test_network_class() {
        for (expected_class, network_str) in &[
            (Class::A, "125.0.0.0/0"),
            (Class::B, "128.122.0.0/0"),
            (Class::C, "192.168.147.0/0"),
            (Class::D, "224.12.98.255/0"),
            (Class::E, "255.255.255.254/32"),
        ] {
            assert_eq!(
                expected_class,
                &Network::try_from(*network_str).unwrap().class(),
                "failed network address: {}",
                network_str
            );
        }
    }

    #[test]
    fn test_reserved_addresses() {
        assert_matches!(
            Network::try_from("127.0.0.1/0"),
            Err(Error::ReservedAddress(ReservedAddress::Loopback)),
            "Expected network instantiation with reserved address to fail"
        );

        assert_matches!(
            Network::try_from("255.255.255.255/32"),
            Err(Error::ReservedAddress(ReservedAddress::LocalBroadcast)),
            "Expected network instantiation with reserved address to fail"
        )
    }

    #[test]
    fn test_netmask() {
        let network = Network::try_from("192.168.147.0/28").unwrap();
        assert_eq!(
            network.net_mask().unwrap(),
            0b11111111_11111111_11111111_00000000
        );
    }

    #[test]
    fn test_subnet_mask() {
        let network = Network::try_from("192.168.147.0/28").unwrap();
        assert_eq!(network.subnet_mask(), 0b11111111_11111111_11111111_11110000);
    }

    #[test]
    fn test_num_subnets() {
        for (network, expected) in &[
            ("192.168.147.0/0", 0_u32),
            ("192.168.147.0/27", 6),
            ("192.168.147.0/28", 14),
            ("192.168.147.0/30", 62),
            ("192.168.147.0/31", 126),
            ("192.168.147.0/32", 254),
        ] {
            assert_eq!(
                Network::try_from(*network).unwrap().num_subnets().unwrap(),
                *expected,
                "Failed network: {}",
                network
            );
        }
    }

    #[test]
    fn test_num_hosts_per_subnet() {
        for (network, expected) in &[
            ("192.168.147.0/0", 254_u32),
            ("192.168.147.0/27", 30),
            ("192.168.147.0/28", 14),
            ("192.168.147.0/30", 2),
            ("192.168.147.0/31", 0),
            ("192.168.147.0/32", 0),
        ] {
            assert_eq!(
                Network::try_from(*network).unwrap().num_hosts_per_subnet(),
                *expected,
                "failed: {}",
                network
            );
        }
    }

    #[test]
    fn test_network_display() {
        let network = Network::from_dotted_decimal_parts(192, 168, 147, 0, 28).unwrap();
        assert_eq!(
            "class C network\n\
            Subnets:      14\n\
            Hosts/subnet: 14",
            &network.to_string()
        );
    }

    #[test]
    fn test_subnet_iter() {
        let network = Network::try_from("192.168.147.0/28").unwrap();
        let subnets: Vec<String> = network.subnets().map(|a| a.to_string()).collect();

        assert_eq!(
            subnets.iter().map(AsRef::as_ref).collect::<Vec<&str>>(),
            vec![
                "192.168.147.16",
                "192.168.147.32",
                "192.168.147.48",
                "192.168.147.64",
                "192.168.147.80",
                "192.168.147.96",
                "192.168.147.112",
                "192.168.147.128",
                "192.168.147.144",
                "192.168.147.160",
                "192.168.147.176",
                "192.168.147.192",
                "192.168.147.208",
                "192.168.147.224",
            ]
        )
    }

    #[test]
    fn test_addresses_iter_class_c() {
        let network = Network::try_from("192.168.147.0/28").unwrap();
        let addresses: Vec<String> = network.addresses().map(|a| a.to_string()).collect();

        assert_eq!(&addresses[0], "class C network");
        assert_eq!(&addresses[1], "subnet");
        assert_eq!(&addresses[2], "host");
        assert_eq!(&addresses[15], "host");
        assert_eq!(&addresses[16], "subnet broadcast");
        assert_eq!(&addresses[17], "subnet");
        assert_eq!(&addresses[18], "host");
        // ...
        assert_eq!(&addresses[224], "subnet broadcast");
        assert_eq!(&addresses[225], "network broadcast");
    }

    #[test]
    fn test_addresses_iter_class_d() {
        let network = Network::try_from("224.12.98.255/28").unwrap();
        let addresses: Vec<String> = network.addresses().map(|a| a.to_string()).collect();

        assert_eq!(&addresses[0], "class D network");
        assert_eq!(addresses.len(), 1);
    }
}
