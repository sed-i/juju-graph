use std::{io, path::PathBuf, result};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cannot read {path}: {cause}")]
    IO { path: PathBuf, cause: io::Error },

    #[error(transparent)]
    SerdeYaml(#[from] serde_yaml::Error),
}

pub type Result<T> = result::Result<T, Error>;
