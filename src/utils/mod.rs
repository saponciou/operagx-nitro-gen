use self::errors::ErrorT;

pub mod cmd;
pub mod constants;
pub mod errors;
pub mod proxy;

pub type Res<T> = Result<T, ErrorT>;
