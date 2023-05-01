use std::collections::HashMap;

pub struct Request<'a> {
    pub path: Option<String>,
    pub meta: Option<HashMap<String, String>>,
    pub body: Option<&'a [u8]>,
}
