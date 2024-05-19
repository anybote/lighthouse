use serde::Deserialize;
use std::fs;

const CONTENT_CONFIG_FILE_PATH: &str = "assets/config/content.toml";
const IMAGE_METADATA_FILE_PATH: &str = "assets/config/images.toml";
const SERVER_CONFIG_FILE_PATH: &str = "assets/config/server.toml";

#[derive(Deserialize)]
#[readonly::make]
pub struct ContentConfig {
    pub title: String,
    pub header: String,
    pub subheader: String,
}

impl ContentConfig {
    pub fn read() -> ContentConfig {
        let contents = fs::read_to_string(CONTENT_CONFIG_FILE_PATH).unwrap_or_else(|_| panic!("content config file not found at {}",
            CONTENT_CONFIG_FILE_PATH));

        toml::from_str(&contents).expect("unable to parse content config file")
    }
}

#[derive(Deserialize)]
#[readonly::make]
pub struct ImageConfig {
    pub images: Vec<ImageMetadatum>,
}

#[derive(Clone, Deserialize)]
#[readonly::make]
pub struct ImageMetadatum {
    pub file: String,
    pub alt: String,
    pub start: i64,
    pub end: i64,
}

impl ImageConfig {
    pub fn read() -> ImageConfig {
        let contents = fs::read_to_string(IMAGE_METADATA_FILE_PATH).unwrap_or_else(|_| panic!("image metadata file not found at {}",
            IMAGE_METADATA_FILE_PATH));

        toml::from_str(&contents).expect("unable to parse image metadata file")
    }
}

#[derive(Deserialize)]
#[readonly::make]
pub struct ServerConfig {
    pub logging_level: String,
}

impl ServerConfig {
    pub fn read() -> ServerConfig {
        let contents = fs::read_to_string(SERVER_CONFIG_FILE_PATH).unwrap_or_else(|_| panic!("server config file not found at {}",
            SERVER_CONFIG_FILE_PATH));

        toml::from_str(&contents).expect("unable to parse server config file")
    }
}
