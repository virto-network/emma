use core::error::Error;

use emma_core::{
    map,
    module::Module,
    primitives::{Request, Response, ResponseError},
};

pub trait Router {
    fn register(&mut self, path: String, module: Box<dyn Module>) -> Result<(), Box<dyn Error>>;
    fn execute<'a>(&self, request: Request<'a>) -> Result<Response, ResponseError>;
}

#[cfg(feature = "http")]
pub mod http {
    use super::*;

    #[derive(Default)]
    pub struct HTTPRouter {
        routes: matchit::Router<Box<dyn Module>>,
    }

    impl Router for HTTPRouter {
        fn register(
            &mut self,
            path: String,
            module: Box<dyn Module>,
        ) -> Result<(), Box<dyn Error>> {
            Ok(self.routes.insert(path, module)?)
        }

        fn execute<'a>(&self, request: Request<'a>) -> Result<Response, ResponseError> {
            let module = match self
                .routes
                .at(&request.path.clone().unwrap_or(String::from("/")))
            {
                Ok(module) => module,
                Err(e) => {
                    return Err(ResponseError {
                        meta: map! {
                          "HTTP_Status_Code" => "404"
                        },
                        message: e.to_string(),
                    });
                }
            }
            .value;

            module.execute(&request)
        }
    }
}
