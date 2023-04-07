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

    pub fn application_data(&self) -> &[u8] {
        let mut application_data_size: usize = 0;
        let mut offset = 0;
        for (i, v) in self.image_bytes.iter().enumerate() {
            if v == &0xff && self.image_bytes[i + 1] == 0xe0 {
                offset = i + 4;
                application_data_size =
                    self.image_bytes[i + 2] as usize + self.image_bytes[i + 3] as usize - 2;
            }
        }
        let vec_slice: &[u8] = &self.image_bytes[offset..application_data_size + offset];
        vec_slice
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
