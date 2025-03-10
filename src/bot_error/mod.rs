use thiserror::Error;


#[macro_export] 
macro_rules! json_ok_or {
    ($a:expr,$b:expr) => {
        $a.get($b).ok_or(ThrErr::thr_err())?
    };
}


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

