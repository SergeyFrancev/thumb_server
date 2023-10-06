use std::{convert::Infallible, fs::File, path::PathBuf};

use cli_log::debug;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use std::io::Read;

use crate::{resolve, thumb_img, ThumbServerError};

fn return_file(path: &PathBuf) -> Result<Vec<u8>, ThumbServerError> {
    let file = File::open(path);
    if let Err(err) = file {
        return Err(ThumbServerError::Io(err));
    }

    let mut file = file.unwrap();
    let mut buffer = Vec::new();
    let read_result = file.read_to_end(&mut buffer);
    if let Err(err) = read_result {
        return Err(ThumbServerError::Io(err));
    }

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
    let thumb = thumb_img(
        &img_result.source(),
        &target,
        img_result.width(),
        img_result.height(),
    );

    if let Err(err) = thumb {
        return Err(err);
    }

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
pub async fn start() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // pretty_env_logger::init();

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(hello)) }
    });

    let addr = ([127, 0, 0, 1], 1234).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
