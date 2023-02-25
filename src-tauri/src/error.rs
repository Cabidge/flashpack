use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("[surreal] {0}")]
    Surreal(#[from] surrealdb::Error),
    #[error(transparent)]
    Custom(#[from] anyhow::Error),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}
