use crate::ipv4::ReservedAddress;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

#[derive(Debug, ::thiserror::Error)]
pub enum Error {
    #[error("Subnet mask was invalid")]
    InvalidSubnetMask,
    #[error("Network address was invalid")]
    InvalidAddress,
    #[error("Reserved address cannot be used as a network: {0}")]
    ReservedAddress(ReservedAddress),
}
