mod helpers;
pub mod provider;

pub use crate::helpers::EmailSender;
pub use lettre::{Address, Message, error::Error};
