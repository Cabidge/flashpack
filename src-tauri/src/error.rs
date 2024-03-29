use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("[sqlx] {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("[migrate] {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),
    #[error("[io] {0}")]
    Io(#[from] std::io::Error),
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
