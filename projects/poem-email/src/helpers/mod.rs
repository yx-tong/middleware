use lettre::{
    Message,
    error::Error,
    message::{Mailbox, header::ContentType},
    transport::smtp::response::Response,
};
use std::{borrow::Cow, io::ErrorKind, str::FromStr};

pub trait EmailSender {
    fn send_message(&self, message: &Message) -> Result<Response, Error>;

    fn company_name(&self) -> Cow<str>;
    fn sender_mail(&self) -> Mailbox;
    fn login_code(&self, receiver: &str, code: &str, unsubscribe: &str) -> Result<Response, Error> {
        let subject = match self.company_name().as_ref() {
            "" => "登录验证".to_string(),
            s => format!("【{}】 登录验证", s),
        };
        let receiver = Mailbox::from_str(receiver).map_err(|e| Error::Io(std::io::Error::new(ErrorKind::InvalidInput, e)))?;
        let message = Message::builder()
            .from(self.sender_mail())
            .reply_to(self.sender_mail())
            .to(receiver)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(format!(
                r#"
<p>您的动态验证码为：{code}，您正在进行邮箱登录，此验证码五分钟内有效。</p>
<p>如非本人操作，请忽略本邮件。</p>
<p>退订链接: {unsubscribe} (退订后您将无法使用邮箱登录或注册!)</p>
"#
            ))?;
        self.send_message(&message)
    }
}
