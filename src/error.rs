use std::{io, result};

use thiserror::Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    IO(#[from] io::Error),
    #[error("json error: {0}")]
    JSON(#[from] serde_json::Error),
    #[error("Response result is none")]
    NoResult,
    #[error("Error Response with no parameters")]
    NoParameters,
    #[error("Error Response: {0}")]
    Parameters(String),
    #[error("netc error: {0}")]
    NetC(#[from] netc::error::Error),
    #[error("Error Response: {0}")]
    Description(String),
    #[error("Error Response with no description")]
    NoDescription,
}
