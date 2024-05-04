use std::path::PathBuf;
use once_cell::sync::Lazy;
use dirs_next::config_dir;

pub struct Global;

impl Global {

    pub const APP_NAME: &'static str = "Paimon";
    pub const APP_VERSION: &'static str = "0.0.1";
    pub const APP_AUTHOR: &'static str = "@Ravenlib";
    pub const APP_HOMEPAGE: &'static str = "https://github.com/Ravenlib/Paimon";
    
    pub const MONLIB_API_REQUEST: &'static str = "http://localhost/Monlib/api/";
    pub const PAIMON_SCRAPE_API_REQUEST: &'static str = "http://localhost:5001/api?url=";
    
    // pub const API_USER_ENDPOINT: &'static str = "user";
    pub const API_LISTS_ENDPOINT: &'static str = "lists";
    
    pub const APP_FOLDER: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = config_dir().expect("No config directory");
        path.push(Self::APP_NAME);
        path
    });
    
    pub const ENV_URL: &'static str = "https://pastebin.com/raw/wZGaNtsL";

}
