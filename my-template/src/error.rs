/*
 * @Author: plucky
 * @Date: 2023-04-20 17:23:32
 * @LastEditTime: 2023-04-22 17:26:01
 * @Description: 
 */

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Generic: {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("unknown data error")]
    Unknown,
    
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Generic(s.to_string())
    }
}
    
