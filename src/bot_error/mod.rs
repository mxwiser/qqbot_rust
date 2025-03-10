use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("serde_json {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("{0}")]
    Error(std::string::String),
}
pub  struct ThrErr;
impl ThrErr {
    pub fn thr_err() -> Error{
        return Error::Error("err".to_string());
    }
}