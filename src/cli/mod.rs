pub mod args;

use {crate::*, args::Args, clap::Parser, cli_log::*};

// fn thumb(path: PathBuf) -> Result<(), ThumbServerError> {
//     const WIDTH: u32 = 60;
//     const HEIGHT: u32 = 80;

//     let target = path.clone();
//     let file_name = target.file_name().unwrap().to_str().unwrap();
//     let target = target.parent().unwrap().as_os_str().to_str().unwrap();
//     let target = PathBuf::from_iter([target, format!("{WIDTH}x{HEIGHT}").as_str(), file_name]);
//     thumb_img(&path, &target, WIDTH, HEIGHT)
// }

pub fn run() -> Result<(), ThumbServerError> {
    let args = Args::parse();
    debug!("args: {:#?}", &args);
    if args.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    // let _ = thumb(args.conf);
    let config = parse_config(args.conf);
    match config {
        Ok(conf) => {
            debug!("Success confuration");
            conf::init(conf);
            let _ = server::start(args.port);
        }
        Err(err) => return Err(err),
    }
    log_mem(Level::Info);
    Ok(())
}
