mod helpers;
pub mod provider;

pub use crate::helpers::EmailSender;
pub use lettre::{
    Address, Message,
    address::AddressError,
    error::Error,
    message::{Mailbox, MessageBuilder, header::ContentType},
};
