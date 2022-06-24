use nom::Needed;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IOError(#[from] io::Error),

    #[error("Parsing {0} requires {1:?} bytes/chars.")]
    NomIncomplete(String, Needed),
}
