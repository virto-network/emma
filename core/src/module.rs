use crate::primitives::{Request, Response, ResponseError};

pub trait Module: std::fmt::Debug {
    fn path(&self) -> String;
    fn execute<'a>(&self, request: &Request<'a>) -> Result<Response, ResponseError>;
}
