use std::env::VarError;

use thiserror::Error;



#[derive(Error, Debug)]
pub enum Error {
    #[error("serde_json {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("varError {0}")]
    VarError(#[from] VarError),
    #[error("{0}")]
    Error(std::string::String),
}
pub  struct ThrErr;
impl ThrErr {
    pub fn thr_err(msg:String) -> Error{
        return Error::Error(msg);
    }

}

#[macro_export] 
macro_rules! json_ok_or {
    ($a:expr,$b:expr) => {
        $a.get($b).ok_or(bot_error::ThrErr::thr_err(format!("json_ok_or {}",$b) .to_string()))?
    };
}
#[allow(unused_macros)]
#[macro_export] 
macro_rules! ok_or {
    ($a:expr) => {
        $a.ok_or(bot_error::ThrErr::thr_err(format!("ok_or err") .to_string()))?
    };
    ($a:expr,$b:expr) => {
        $a.ok_or(bot_error::ThrErr::thr_err(format!("ok_or {}",$b) .to_string()))?
    };
}


