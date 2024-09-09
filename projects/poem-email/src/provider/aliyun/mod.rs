use super::*;
use lettre::message::Mailbox;
use std::{borrow::Cow, io::ErrorKind, str::FromStr};

#[derive(Clone, Debug)]
pub struct AliyunMailer {
    smtp: SmtpTransport,
    company: Cow<'static, str>,
    sender: Mailbox,
}

impl AliyunMailer {
    pub fn login(username: &str, password: &str) -> Result<Self, Error> {
        let creds = Credentials::new(username.to_string(), password.to_string());
        let mailer = SmtpTransport::relay("smtpdm.aliyun.com").unwrap();
        let sender = Mailbox::from_str(username).map_err(|e| Error::Io(std::io::Error::new(ErrorKind::InvalidInput, e)))?;
        Ok(Self { smtp: mailer.credentials(creds).build(), company: Cow::Borrowed(""), sender })
    }
    pub fn with_company(mut self, company: &str) -> Self {
        self.company = Cow::Owned(company.to_string());
        self
    }
    pub fn with_sender(mut self, name: &str) -> Self {
        self.sender.name = Some(name.to_string());
        self
    }
}

impl EmailSender for AliyunMailer {
    fn send_message(&self, message: &Message) -> Result<Response, Error> {
        self.smtp.send(message).map_err(|e| Error::Io(std::io::Error::new(std::io::ErrorKind::Interrupted, e)))
    }

    fn company_name(&self) -> Cow<str> {
        Cow::Borrowed(self.company.as_ref())
    }

    fn sender_mail(&self) -> Mailbox {
        self.sender.clone()
    }
}

impl<'a> FromRequest<'a> for &'a AliyunMailer {
    async fn from_request(input: &'a Request, _: &mut RequestBody) -> poem::Result<Self> {
        match input.extensions().get::<AliyunMailer>() {
            Some(s) => Ok(s),
            None => Err(poem::Error::from(GetDataError("`Route` 未配置 `.data(AliyunMailer)`"))),
        }
    }
}
