use crate::EmailSender;
use lettre::{
    Message, SmtpTransport, Transport,
    error::Error,
    transport::smtp::{authentication::Credentials, response::Response},
};
use poem::{FromRequest, Request, RequestBody, error::GetDataError};

mod aliyun;

pub use self::aliyun::AliyunMailer;
