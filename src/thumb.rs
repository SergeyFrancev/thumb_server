use std::fs;
use std::path::PathBuf;

use cli_log::{debug, error};

use crate::ThumbServerError;

fn create_dir_for_file(path: &PathBuf) -> Result<(), ThumbServerError> {
    let parent = path.parent();
    if let Some(parent_dir) = parent {
        let res = fs::create_dir_all(parent_dir);
        if let Err(_) = res {
            return Err(ThumbServerError::CreateDirectoryError(PathBuf::from(
                parent_dir,
            )));
        }
        Ok(())
    } else {
        Err(ThumbServerError::InvalidThumbDirectory(path.clone()))
    }
}

pub fn thumb_img(
    source: &PathBuf,
    target: &PathBuf,
    nw: u32,
    nh: u32,
) -> Result<(), ThumbServerError> {
    debug!("Create thumb from:{}", source.display());
    let image_res = image::open(source);
    if let Err(err) = image_res {
        error!("ERROR open source: {} [{}]", source.display(), err);
        return Err(ThumbServerError::CreateThumbError(err));
    }

    create_dir_for_file(target)?;

    let thumb = image_res.unwrap().thumbnail(nw, nh);
    let save_result = thumb.save(target);
    if let Err(_err) = save_result {
        error!("ERROR create target: {}", _err);
        return Err(ThumbServerError::CreateThumbError(_err));
    }

    Ok(())
}
