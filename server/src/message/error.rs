#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "The client must set its ID")]
    ClientIdNotSpecified,
    #[fail(display = "The client must set the requested address")]
    AddressNotSpecified,
    #[fail(display = "Invalid address")]
    InvalidAddress,
    #[fail(display = "The dynamic pool has been exhausted")]
    DynamicPoolExhausted,
    #[fail(display = "The offer not found")]
    OfferNotFound,
    #[fail(display = "The requested address is not offered")]
    AddressNotOffered,
    #[fail(display = "The offer is expired")]
    OfferExpired,
    #[fail(display = "The lease not found")]
    LeaseNotFound,
    #[fail(display = "The lease has different address")]
    LeaseHasDifferentAddress,
}