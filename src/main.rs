#[macro_use]
extern crate cli_log;

fn main() -> Result<(), thumb_server::ThumbServerError> {
    init_cli_log!("thumb-server");
    let _ = thumb_server::run();
    info!("bye");
    Ok(())
}
