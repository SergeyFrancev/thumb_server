#[cfg(test)]
mod resolve_img {
    use std::path::PathBuf;

    use config_file::FromConfigFile;
    use thumb_server::Config;
    use thumb_server::Resolver;

    // use crate::BASE_DIR;
    // const BASE_DIR: &str = "/Users/nulldata/Documents/projects/rust/thumb_server/tests/testdata";
    const BASE_DIR: &str = "./tests/testdata";
    const PATH_TO_CONF: &str = "./tests/test-conf.toml";

    // fn setup<'a>() -> Resolver<'a> {
    // let conf = Config::from_config_file(PATH_TO_CONF).unwrap();
    // Resolver::from_conf(&conf)
    // let conf = Config::from_config_file(PATH_TO_CONF).unwrap();
    // Resolver::from_confi(&Config::from_config_file(PATH_TO_CONF).unwrap())
    // }

    #[test]
    fn valid_path_is_ok() {
        let conf = Config::from_config_file(PATH_TO_CONF).unwrap();
        let resolver = Resolver::from_conf(&conf);
        let res = resolver.resolve("/image.jpeg");
        assert!(res.is_ok(), "Valid path is not OK");
        let r = res.unwrap();
        let img = r.result().unwrap();
        assert_eq!(img, &PathBuf::from(BASE_DIR).join("image.jpeg"))
    }

    #[test]
    fn not_available_top_dir() {
        let conf = Config::from_config_file(PATH_TO_CONF).unwrap();
        let resolver = Resolver::from_conf(&conf);
        let res = resolver.resolve("/../image.jpeg");
        assert!(res.is_err(), "Top level is available");
    }

    #[test]
    fn error_on_deep_to_top_level() {
        let conf = Config::from_config_file(PATH_TO_CONF).unwrap();
        let resolver = Resolver::from_conf(&conf);
        let res = resolver.resolve("/dir.dot/../../image.jpeg");
        assert!(res.is_err(), "Top level is available");
    }

    #[test]
    fn not_available_uri_part_only_dot() {
        let conf = Config::from_config_file(PATH_TO_CONF).unwrap();
        let resolver = Resolver::from_conf(&conf);
        let res = resolver.resolve("/dir/./image.jpeg");
        assert!(res.is_err(), "Uri with only dot is available");
    }

    #[test]
    fn available_uri_dir_contains_dot() {
        let conf = Config::from_config_file(PATH_TO_CONF).unwrap();
        let resolver = Resolver::from_conf(&conf);
        let res = resolver.resolve("/dir.dot/image.jpeg");
        assert!(res.is_ok(), "Image dir with dot not available");
        let r = res.unwrap();
        let img = r.result().unwrap();
        assert_eq!(img, &PathBuf::from(BASE_DIR).join("dir.dot/image.jpeg"))
    }

    #[test]
    fn path_to_dir_is_invalid() {
        let conf = Config::from_config_file(PATH_TO_CONF).unwrap();
        let resolver = Resolver::from_conf(&conf);
        let res = resolver.resolve("/dir.dot");
        assert!(res.is_err(), "Path to dir is valid");
    }

    #[test]
    fn path_to_dir_with_end_slash_is_invalid() {
        let conf = Config::from_config_file(PATH_TO_CONF).unwrap();
        let resolver = Resolver::from_conf(&conf);
        let res = resolver.resolve("/dir.dot/");
        assert!(res.is_err(), "Path to dir with end slash is valid");
    }

    #[test]
    fn return_source_and_target_for_thumb() {
        let conf = Config::from_config_file(PATH_TO_CONF).unwrap();
        let resolver = Resolver::from_conf(&conf);
        let res = resolver.resolve("/60x80/image.jpeg");
        assert!(res.is_ok(), "Thumb Uri to correct source invalid");
        // assert_eq!(res.unwrap().uri(), &PathBuf::from("60x80/image.jpeg"))
        let res = res.unwrap();
        assert_eq!(
            res.source(),
            PathBuf::from(BASE_DIR).join("image.jpeg"),
            "Invalid path to source"
        );
        assert_eq!(
            res.target(),
            PathBuf::from(BASE_DIR).join("60x80/image.jpeg"),
            "Invalid path to target"
        );
    }

    #[test]
    fn relative_uri_is_valid() {
        let conf = Config::from_config_file(PATH_TO_CONF).unwrap();
        let resolver = Resolver::from_conf(&conf);
        let res = resolver.resolve("60x80/image.jpeg");
        assert!(res.is_ok(), "Invalid uri without start '/'");
        assert_eq!(res.unwrap().uri(), &PathBuf::from("60x80/image.jpeg"))
    }

    #[test]
    fn erro_on_unavaliable_size() {
        let conf = Config::from_config_file(PATH_TO_CONF).unwrap();
        let resolver = Resolver::from_conf(&conf);
        let res = resolver.resolve("/321x123/image.jpeg");
        assert!(res.is_err());
    }

    #[test]
    fn initialize_size_by_uri() {
        let conf = Config::from_config_file(PATH_TO_CONF).unwrap();
        let resolver = Resolver::from_conf(&conf);
        let res = resolver.resolve("/60x80/image.jpeg");
        assert!(res.is_ok(), "Path to dir with end slash is valid");
        let img = res.unwrap();
        assert_eq!(img.width(), 60);
        assert_eq!(img.height(), 80);
    }

    #[test]
    fn correct_source() {
        let conf = Config::from_config_file(PATH_TO_CONF).unwrap();
        let resolver = Resolver::from_conf(&conf);
        let res = resolver.resolve("/60x80/image.jpeg");
        assert!(res.is_ok(), "Error on generate thumb");
        let img = res.unwrap();
        assert_eq!(img.source(), PathBuf::from(BASE_DIR).join("image.jpeg"));
    }

    #[test]
    fn correct_source_for_deep_dir() {
        let conf = Config::from_config_file(PATH_TO_CONF).unwrap();
        let resolver = Resolver::from_conf(&conf);
        let res = resolver.resolve("/dir.dot/60x80/image2.jpeg");
        assert!(res.is_ok(), "Error on generate new thumb");
        let img = res.unwrap();
        assert_eq!(
            img.source(),
            PathBuf::from(BASE_DIR).join("dir.dot/image2.jpeg")
        );
    }

    #[test]
    fn correct_parse_duplicate_size() {
        let conf = Config::from_config_file(PATH_TO_CONF).unwrap();
        let resolver = Resolver::from_conf(&conf);
        let res = resolver.resolve("/dir.dot/60x80/60x80/image.jpeg");
        assert!(res.is_ok(), "Duplicate size part is invalid");
        let img = res.unwrap();
        assert_eq!(
            img.source(),
            PathBuf::from(BASE_DIR).join("dir.dot/60x80/image.jpeg")
        );
    }

    #[test]
    fn return_result_for_exist_source() {
        let conf = Config::from_config_file(PATH_TO_CONF).unwrap();
        let resolver = Resolver::from_conf(&conf);
        let res = resolver.resolve("/dir.dot/image.jpeg");
        assert!(res.is_ok(), "Exist source return ERROR");
        let r = res.unwrap();
        let img = r.result().unwrap();
        assert_eq!(
            img,
            &PathBuf::from_iter([conf.base_dir(), &PathBuf::from("dir.dot/image.jpeg")])
        );
    }

    #[test]
    fn return_result_for_exist_thumb() {
        let conf = Config::from_config_file(PATH_TO_CONF).unwrap();
        let resolver = Resolver::from_conf(&conf);
        let res = resolver.resolve("/dir.dot/60x80/image.jpeg");
        assert!(res.is_ok(), "Exist thumb return ERROR");
        let r = res.unwrap();
        let img = r.result().unwrap();
        assert_eq!(
            img,
            &PathBuf::from_iter([conf.base_dir(), &PathBuf::from("dir.dot/60x80/image.jpeg")])
        );
    }

    #[test]
    fn error_for_not_found_source() {
        let conf = Config::from_config_file(PATH_TO_CONF).unwrap();
        let resolver = Resolver::from_conf(&conf);
        let res = resolver.resolve("/dir.dot/60x80/image3.jpeg");
        assert!(res.is_err(), "Resolved unavaliable source");
    }
}
