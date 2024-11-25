use image::{DynamicImage, GenericImageView, imageops};

fn rotate_image(image: &DynamicImage) -> DynamicImage {
    image.rotate90()
}

fn resize_image(image: &DynamicImage, height: u32, width: u32) -> DynamicImage {
    image.resize(height, width, imageops::FilterType::Nearest)
}

fn adjust_brightness(image: &DynamicImage, value: i32) -> DynamicImage {
    image.brighten(value)
}
