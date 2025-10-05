mod request;
mod compile;
mod run_container;

use hyper::{Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // address to bind to
    let addr = ([127, 0, 0, 1], 3000).into();

    // a MakeService closure — constructs a new service for each connection
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our async function into a `Service`
        Ok::<_, Infallible>(service_fn(request::handle_request))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    // run the server until Ctrl+C
    server.await?;

    Ok(())
}

#[allow(dead_code, unused_imports)]
mod tests {
    use crate::run_container::create_runner;
    use crate::compile::compile_c_file;
    use std::fs;
    use std::path::Path;
    const TMP_DIR: &str = "/tmp/untrusted";

    #[test]
    fn gavin_test() -> Result<(), anyhow::Error> {
        // Create path for binary if path not created
        let path = Path::new(TMP_DIR);
        if !path.exists() {
            fs::create_dir_all(path)?;
        }

        // Compile c file
        let temp_file = format!("{}{}", TMP_DIR, "/temp"); // TODO : make temp file name scalable
        let compiled = compile_c_file("src/test.c", &temp_file);
        match compiled {
            Ok(v) =>  {
                println!("Success: {}", v.display());
                println!("{}", v.display());
            }
            Err(e) => println!("Error: {}", e), // TODO : Pass error to user if it didn't compile through JSON
        }

        let mut temp_string = TMP_DIR.to_string();
        temp_string.push_str("/temp");
        let bin_dir: &str = &temp_string;


        match create_runner(bin_dir) {
            Ok(_) => println!("Created runner."),
            Err(e) => println!("Error with runner creation: {}", e),
        }
        Ok(())
    }
}