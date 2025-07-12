use std::io::Cursor;

use bytes::Bytes;
use image::{DynamicImage, ImageReader};
use nutype::nutype;

use crate::ApiError;

#[nutype(validate(predicate = |_image| true), derive(Debug, AsRef))]
pub struct EventImage(DynamicImage);

impl TryFrom<Bytes> for crate::EventImage {
    type Error = ApiError;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        let image_raw = {
            ImageReader::new(Cursor::new(value))
                .with_guessed_format()
                .map_err(|e| ApiError::Other(e.to_string()))?
                .decode()
                .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
        };
        Self::try_new(image_raw).map_err(|e| ApiError::InvalidArgument(e.to_string()))
    }
}

impl crate::EventImage {
    pub fn encode_as_webp(&self) -> crate::ApiResult<Bytes> {
        let webp_image = {
            webp::Encoder::from_image(self.as_ref())
                .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
                .encode_lossless()
        };
        let encoded = Bytes::copy_from_slice(&webp_image);
        Ok(encoded)
    }
}
