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

    pub fn end_of_image(&self) -> usize {
        for (i, v) in self.image_bytes.iter().enumerate() {
            if v == &0xff && self.image_bytes[i + 1] == 0xd9 {
                return i + 1;
            }
        }
        return 0;
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

pub fn image() -> Result<(), String> {
    let image_bytes = match fs::read("example.jpg") {
        Ok(val) => val,
        Err(_e) => return Err(String::from("ERROR:error reading file")),
    };

    let image = Image::new(image_bytes.clone());

    let end_of_image = image.end_of_image();
    println!("{}", end_of_image);

    Ok(())
}
