use std::{convert::Infallible, fs::File, io::BufReader, path::PathBuf};

use cli_log::debug;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use std::io::Read;

use crate::{resolve, thumb_img, ThumbServerError};

pub fn return_file(path: &PathBuf) -> Result<Vec<u8>, ThumbServerError> {
    let mut reader = BufReader::new(File::open(path).map_err(ThumbServerError::Io)?);

    let mut buffer = Vec::new();
    let _ = reader
        .read_to_end(&mut buffer)
        .map_err(ThumbServerError::Io)?;

    Ok(buffer)
}

pub async fn serve_file(uri: &str) -> Result<Vec<u8>, ThumbServerError> {
    let img_result = resolve(uri)?;

    // Return exist image
    if let Some(file) = img_result.result() {
        debug!("SUCCESS File: {}", file.display());
        return return_file(&file);
    }

    // Resolve thumb request
    let target = img_result.target();
    let _ = thumb_img(
        &img_result.source(),
        &target,
        img_result.width(),
        img_result.height(),
    )?;

    debug!("SUCCESS File: {}", target.display());
    return_file(&target)
}

async fn hello(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    debug!("=================================");
    debug!("Req uri: {}", request.uri().path());
    let path = request.uri().path();
    match serve_file(&path).await {
        Ok(file_content) => Ok(Response::new(Body::from(file_content))),
        // Ok(_) => Ok(Response::new(Body::from(format!("Path: {}", path)))),
        Err(_) => Ok(Response::builder().status(404).body(Body::empty()).unwrap()),
    }
}

#[tokio::main]
pub async fn start(port: u16) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // pretty_env_logger::init();

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(hello)) }
    });

    let addr = ([127, 0, 0, 1], port).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
