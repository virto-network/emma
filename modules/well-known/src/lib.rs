use emma_core::{
    map,
    module::Module,
    primitives::{Request, Response, ResponseError},
};
use serde_json::json;

#[derive(Debug)]
pub struct WellKnownMatrixClientModule;

impl Module for WellKnownMatrixClientModule {
    fn path(&self) -> String {
        "/.well-known/matrix/client".to_string()
    }

    fn execute<'a>(&self, _: &Request<'a>) -> Result<Response, ResponseError> {
        Ok(Response {
            meta: map! {
              "HTTP_Status_Code" => "200",
              "HTTP_Header_Content-Type" => "application/json"
            },
            body: json!({
              "m.homeserver": {
                "base_url": "http://localhost:8484"
              }
            })
            .to_string()
            .into_bytes(),
        })
    }
}
