use std::result::Result;

#[derive(Debug, Clone)]
pub struct ApplicationData<'a> {
    pub application_data: &'a [u8],
    pub is_exif: bool,
    pub is_jfif: bool,
    pub format_name: Vec<u8>,
    pub format_version: Vec<u8>,
    pub density_field: u8,
    pub horizontal_px_density: Vec<u8>,
    pub vertical_px_density: Vec<u8>,
    pub thumbnail_image: u8,
    pub thumbnail_image_width: u8,
    pub thumbnail_image_height: u8,
}

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

    pub fn application_data(&self) -> ApplicationData {
        let mut application_data_size: usize = 0;
        let mut offset = 0;
        let mut is_exif = false;
        let mut is_jfif = false;
        for (i, v) in self.image_bytes.iter().enumerate() {
            if v == &0xff && self.image_bytes[i + 1] == 0xe0 {
                offset = i + 4;
                application_data_size =
                    self.image_bytes[i + 2] as usize + self.image_bytes[i + 3] as usize - 2;
            } else if v == &0xff && self.image_bytes[i + 1] == 0xe1 {
                is_exif = true;
            }
        }
        let application_data: &[u8] = &self.image_bytes[offset..application_data_size + offset];
        if application_data[0] == 0x4a
            && application_data[1] == 0x46
            && application_data[2] == 0x49
            && application_data[3] == 0x46
        {
            is_jfif = true;
        }

        let format_name: Vec<u8> = vec![
            application_data[0],
            application_data[1],
            application_data[2],
            application_data[3],
        ];
        let format_version: Vec<u8> = vec![application_data[4], application_data[5]];
        let density_field: u8 = application_data[6];
        let horizontal_px_density: Vec<u8> = vec![application_data[7], application_data[8]];
        let vertical_px_density: Vec<u8> = vec![application_data[9], application_data[10]];
        let thumbnail_image: u8 = application_data[11];
        let thumbnail_image_width: u8 = application_data[12];
        let thumbnail_image_height: u8 = application_data[13];
        let app_data = ApplicationData {
            application_data,
            is_exif,
            is_jfif,
            format_name,
            format_version,
            density_field,
            horizontal_px_density,
            vertical_px_density,
            thumbnail_image,
            thumbnail_image_height,
            thumbnail_image_width,
        };

        app_data
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
