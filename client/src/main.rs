#![feature(error_in_core)]

use core::error::Error;
use emma_core::{map, module::Module, primitives::Request};
use server::Router;
use std::{collections::HashMap};

mod server;

fn main() -> Result<(), Box<dyn Error>> {
    let mut router = if cfg!(feature = "http") {
        server::http::HTTPRouter::default()
    } else {
        unimplemented!("Non-HTTP router not implemented... yet!")
    };

    let modules: Vec<Box<dyn Module>> = vec![Box::new(
        emma_modules_well_known::WellKnownMatrixClientModule,
    )];

    for module in modules {
        router.register(module.path(), module)?;
    }

    let path = std::env::var("PATH_TRANSLATED").unwrap_or(String::from("/"));

    let res = router.execute(Request {
        path: Some(path.clone()),
        meta: Some(map! {}),
        body: None,
    });

    let print_prelude = |meta: HashMap<String, String>, failed: bool| {
        println!(
            "Content-Type: {}",
            meta.get("HTTP_Header_Content-Type")
                .unwrap_or(&String::from("text/plain"))
        );
        println!(
            "Status: {}",
            meta.get("HTTP_Status_Code")
                .unwrap_or(&String::from(if failed { "500" } else { "200" }))
        );
        println!();
    };

    match res {
        Ok(res) => {
            print_prelude(res.meta, false);
            println!("{}", String::from_utf8(res.body)?)
        }
        Err(error) => {
            print_prelude(error.meta, true);
            println!("{}", error.message);
        }
    }

    Ok(())
}
