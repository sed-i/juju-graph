use std::{io, result};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cannot read {file}: {cause}")]
    IO { file: String, cause: io::Error },

    #[error(transparent)]
    SerdeYaml(#[from] serde_yaml::Error),
}

pub type Result<T> = result::Result<T, Error>;
