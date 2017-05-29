
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

use std::error::Error;
use std::fmt::{ Display, Result, Formatter };
use base64::{ DecodeError };
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum CryptoError {
    InvalidString(String),
    InvalidByte(usize, u8, DecodeError),
    InvalidLength(DecodeError),
    UTF8Error(FromUtf8Error)
}

impl  Display for CryptoError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            CryptoError::InvalidString(ref string)    => write!(f, "Invalid string error: \"{}\"", string),
            CryptoError::InvalidByte(index, bytes, _) => write!(f, "Invalid byte {}, offset {}.", bytes, index),
            CryptoError::InvalidLength(_)             => write!(f, "Encoded text cannot have a 6-bit remainder."),
            CryptoError::UTF8Error(_)                 => write!(f, "Invalid size.")
        }
    }
}

impl Error for CryptoError {

    fn description(&self) -> &str {
        match *self {
            CryptoError::InvalidString(_)             => "invalid string.",
            CryptoError::InvalidByte(_, _, ref error) => error.description(),
            CryptoError::InvalidLength(ref error)     => error.description(),
            CryptoError::UTF8Error(ref error)         => error.description()
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl From<DecodeError> for CryptoError {
    fn from(error: DecodeError) -> CryptoError {
        match error {
            DecodeError::InvalidByte(index, bytes) => CryptoError::InvalidByte(index, bytes, error),
            DecodeError::InvalidLength             => CryptoError::InvalidLength(error)
        }
    }
}

impl From<FromUtf8Error> for CryptoError {
    fn from(error: FromUtf8Error) -> CryptoError {
        CryptoError::UTF8Error(error)
    }
}