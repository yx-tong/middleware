#![feature(try_trait_v2)]

#[cfg(feature = "axum")]
mod for_axum;
mod for_error;
mod for_poem;
mod for_serde;

#[cfg(feature = "utoipa")]
mod for_utoipa;

pub use RgbaColor::{Failure, Success};
use std::borrow::Cow;

pub trait ApiError {
    fn error_code(&self) -> i32 {
        -1
    }
    fn error_message(&self) -> Cow<str> {
        Cow::Borrowed("")
    }
}
