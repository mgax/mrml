#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown fragment {0}")]
    UnknownFragment(String),
}
