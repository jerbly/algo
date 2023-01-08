use thiserror::Error;

#[derive(Error, Debug)]
pub enum AlgoError {
    #[error("illegal argument")]
    IllegalArgument,
    #[error("no blocked sites")]
    NoBlockedSites,
}
