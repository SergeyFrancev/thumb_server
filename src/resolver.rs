use std::path::PathBuf;
use std::str::FromStr;

use crate::*;
use cli_log::*;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct ImgResult {
    base_dir: PathBuf,
    uri: PathBuf,
    height: u32,
    width: u32,
    source_uri: PathBuf,
    result: Option<PathBuf>,
}

impl ImgResult {
    fn new(base_dir: PathBuf, uri: PathBuf) -> ImgResult {
        ImgResult {
            base_dir,
            uri,
            height: 0,
            width: 0,
            source_uri: PathBuf::new(),
            result: None,
        }
    }

    fn is_zero_size(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    pub fn source(&self) -> PathBuf {
        self.base_dir.join(self.source_uri.clone())
    }

    pub fn target(&self) -> PathBuf {
        self.base_dir.join(self.uri.clone())
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn result(&self) -> Option<PathBuf> {
        self.result.clone()
    }

    pub fn uri(&self) -> &PathBuf {
        &self.uri
    }
}

#[derive(Debug, Clone)]
pub struct Resolver<'a> {
    base_dir: &'a PathBuf,
    sizes: &'a Vec<String>,
}

impl Resolver<'_> {
    pub fn from_conf(conf: &Config) -> Resolver {
        Resolver {
            base_dir: conf.base_dir(),
            sizes: conf.sizes(),
        }
    }

    pub fn resolve(&self, uri: &str) -> Result<ImgResult, ThumbServerError> {
        let uri_path = self.validate_uri(uri)?;

        let mut image = ImgResult::new(self.base_dir.clone(), uri_path);

        // Exist file returned here
        {
            let result = image.target();
            if result.is_file() {
                debug!("File: {}", result.display());
                image.result = Some(result);
                return Ok(image);
            }
        }

        let image = self.parse_thumb_param(image)?;
        if image.is_zero_size() {
            return Err(ThumbServerError::ZeroThumbSize);
        }
        let source = image.source();
        if !source.is_file() {
            debug!("ERROR Source not found: {}", source.display());
            return Err(ThumbServerError::FileNotFound);
        }
        Ok(image)
    }

    fn validate_uri(&self, uri: &str) -> Result<PathBuf, ThumbServerError> {
        debug!("Start test Uri: {}", uri);

        // Check uri not contains relative parts ".." or "."
        if Regex::new(r"\/\.+\/").unwrap().is_match(uri) {
            debug!("Uri contains /.+/ ERROR: {}", uri);
            return Err(ThumbServerError::InvalidUri);
        }

        // Check URI is file
        let path = PathBuf::from_str(uri);
        if let Err(_) = path {
            debug!("Invalid path from Uri ERROR: {}", uri);
            return Err(ThumbServerError::InvalidUri);
        }
        let mut path = path.unwrap();
        // if !path.has_root() {
        //     path = PathBuf::from("/").join(path);
        // }
        if path.has_root() {
            path = PathBuf::from(path.strip_prefix("/").unwrap());
        }
        if path.extension().is_none() {
            debug!("Uri without extension file ERROR: {}", uri);
            return Err(ThumbServerError::InvalidUri);
        }

        // let img = ImgResult::new(path);
        // img.source = PathBuf::from_iter([self.base_dir, &PathBuf::from(&uri[1..])]);
        // let path = PathBuf::from_iter([self.base_dir, &PathBuf::from(&uri[1..])]);
        // debug!("Resolve Path OK: {}", img.source.display());
        Ok(path)
    }

    fn parse_thumb_param(&self, mut img: ImgResult) -> Result<ImgResult, ThumbServerError> {
        let p = PathBuf::from("/").join(&img.uri);
        let size = Regex::new(r"^/?(?<directory>.*)?/(?<size>[\d]+x[\d]+)/(?<file>[^\/]+$)")
            .unwrap()
            .captures(p.to_str().unwrap());

        return match size {
            Some(caps) => {
                let size_part = caps.name("size").unwrap().as_str();
                if self.sizes.contains(&size_part.to_string()) {
                    let mut size = size_part.split('x').map(|p| p.parse::<u32>().unwrap());
                    img.width = size.next().unwrap();
                    img.height = size.next().unwrap();
                    debug!("SUCCESS parse size: {}x{}", img.width, img.height);

                    let directory = match caps.name("directory") {
                        Some(dir_name) => PathBuf::from(dir_name.as_str()),
                        None => PathBuf::from(""),
                    };
                    let file = PathBuf::from(caps.name("file").unwrap().as_str());
                    img.source_uri = PathBuf::from_iter([directory, file]);
                    debug!("Calc source Uri: {}", img.source_uri.display());
                    Ok(img)
                } else {
                    Err(ThumbServerError::NotAllowedSize)
                }
            }
            None => Err(ThumbServerError::InvalidUri),
        };
    }
}

pub fn resolve(uri: &str) -> Result<ImgResult, ThumbServerError> {
    let conf = conf::get().lock().unwrap();
    let resolver = Resolver::from_conf(&conf);
    return resolver.resolve(uri);
}
