use std::{io, path::PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cannot read {path}: {cause}")]
    IO { path: PathBuf, cause: io::Error },

    #[error(transparent)]
    SerdeYaml(#[from] serde_yaml::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
