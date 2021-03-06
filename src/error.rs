// SPDX-FileCopyrightText: © 2020 Etebase Authors
// SPDX-License-Identifier: LGPL-2.1-only

use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    Generic(String),
    UrlParse(String),
    MsgPack(String),
    ProgrammingError(&'static str),
    MissingContent(&'static str),
    Padding(&'static str),
    Base64(&'static str),
    Encryption(&'static str),
    Unauthorized(String),
    Conflict(String),
    PermissionDenied(String),
    NotFound(String),

    Connection(String),
    TemporaryServerError(String),
    ServerError(String),
    Http(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Generic(s) => s.fmt(f),
            Error::UrlParse(s) => s.fmt(f),
            Error::MsgPack(s) => s.fmt(f),
            Error::ProgrammingError(s) => s.fmt(f),
            Error::MissingContent(s) => s.fmt(f),
            Error::Padding(s) => s.fmt(f),
            Error::Base64(s) => s.fmt(f),
            Error::Encryption(s) => s.fmt(f),
            Error::PermissionDenied(s) => s.fmt(f),
            Error::NotFound(s) => s.fmt(f),
            Error::Unauthorized(s) => s.fmt(f),
            Error::Conflict(s) => s.fmt(f),

            Error::Connection(s) => s.fmt(f),
            Error::TemporaryServerError(s) => s.fmt(f),
            Error::ServerError(s) => s.fmt(f),
            Error::Http(s) => s.fmt(f),
        }
    }
}

impl From<Error> for String {
    fn from(err: Error) -> String {
        err.to_string()
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::Generic(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::UrlParse(err.to_string())
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(err: std::ffi::NulError) -> Error {
        Error::Generic(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::UrlParse(err.to_string())
    }
}

impl From<rmp_serde::encode::Error> for Error {
    fn from(err: rmp_serde::encode::Error) -> Error {
        Error::MsgPack(err.to_string())
    }
}

impl From<rmp_serde::decode::Error> for Error {
    fn from(err: rmp_serde::decode::Error) -> Error {
        Error::MsgPack(err.to_string())
    }
}
