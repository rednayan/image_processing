use std::fs;
use std::result::Result;
#[derive(Debug, Clone)]
pub struct Image {
    pub image_bytes: Vec<u8>,
}

impl Image {
    pub fn new(image_bytes: Vec<u8>) -> Self {
        Self { image_bytes }
    }
}

pub trait Jpeg {
    fn is_jpeg(&self) -> Result<bool, String>;
}

impl Jpeg for Image {
    fn is_jpeg(&self) -> Result<bool, String> {
        if self.image_bytes[0] == 0xff && self.image_bytes[1] == 0xD8 {
            return Ok(true);
        }
        return Err("Not a valid JPEG file".to_string());
    }
}

fn read_image(path: &str) -> Result<Vec<u8>, String> {
    let image_bytes = match fs::read(path) {
        Ok(val) => val,
        Err(e) => return Err(String::from("there has been an error")),
    };
    Ok(image_bytes)
}

pub fn image() -> Result<(), String> {
    let image_bytes = read_image("example.tx");
    let image = Image::new(image_bytes.unwrap());
    image.is_jpeg().unwrap();

    Ok(())
}
