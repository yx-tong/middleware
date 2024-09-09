use quick_xml::{DeError, Error as QxmlError};
use reqwest::{
    Error as ReqwestError,
    header::{InvalidHeaderName as HttpInvalidHeaderNameError, InvalidHeaderValue as HttpInvalidHeaderValueError},
};
use std::{io::Error as IoError, string::FromUtf8Error};

#[derive(Debug)]
pub enum OssError {
    Object(ObjectError),
    Io(IoError),
    String(FromUtf8Error),
    Reqwest(ReqwestError),
    Qxml(QxmlError),
    Http(HttpError),
    DeserializeError(DeError),
}

#[derive(Debug)]
pub enum HttpError {
    HttpInvalidHeaderValue(HttpInvalidHeaderValueError),
    HttpInvalidHeaderName(HttpInvalidHeaderNameError),
}

impl From<QxmlError> for OssError {
    fn from(e: QxmlError) -> OssError {
        OssError::Qxml(e)
    }
}

impl From<IoError> for OssError {
    fn from(e: IoError) -> OssError {
        OssError::Io(e)
    }
}

impl From<ReqwestError> for OssError {
    fn from(e: ReqwestError) -> OssError {
        OssError::Reqwest(e)
    }
}

impl From<HttpInvalidHeaderValueError> for OssError {
    fn from(e: HttpInvalidHeaderValueError) -> OssError {
        OssError::Http(HttpError::HttpInvalidHeaderValue(e))
    }
}

impl From<HttpInvalidHeaderNameError> for OssError {
    fn from(e: HttpInvalidHeaderNameError) -> OssError {
        OssError::Http(HttpError::HttpInvalidHeaderName(e))
    }
}

impl From<FromUtf8Error> for OssError {
    fn from(e: FromUtf8Error) -> OssError {
        OssError::String(e)
    }
}

impl From<DeError> for OssError {
    fn from(value: DeError) -> OssError {
        OssError::DeserializeError(value)
    }
}

#[derive(Debug)]
pub enum ObjectError {
    PutError { msg: String },
    GetError { msg: String },
    CopyError { msg: String },
    DeleteError { msg: String },
    HeadError { msg: String },
    PostError { msg: String },
}
