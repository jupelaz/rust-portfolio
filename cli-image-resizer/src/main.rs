use clap::Parser;
use image::{DynamicImage, ImageError};
use image::imageops::FilterType;
use std::path::Path;

#[derive(Parser)]
struct Args {
    input: String,
    output: String,
    #[arg(long)]
    width: u32,
    #[arg(long)]
    height: u32,
}

/// Resizes an image to the specified dimensions
/// 
/// # Arguments
/// * `input_path` - Path to the input image file
/// * `output_path` - Path where the resized image will be saved
/// * `width` - Target width in pixels
/// * `height` - Target height in pixels
/// 
/// # Returns
/// * `Ok(())` if the operation was successful
/// * `Err(ImageError)` if there was an error opening, resizing, or saving the image
pub fn resize_image<P: AsRef<Path>>(
    input_path: P,
    output_path: P,
    width: u32,
    height: u32,
) -> Result<(), ImageError> {
    let img = image::open(input_path)?;
    let resized = img.resize(width, height, FilterType::Nearest);
    resized.save(output_path)?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    if let Err(e) = resize_image(&args.input, &args.output, args.width, args.height) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgba};
    
    #[test]
    fn test_resize_image_in_memory() {
        // Create a simple 2x2 test image in memory
        let mut test_image = ImageBuffer::new(2, 2);
        test_image.put_pixel(0, 0, Rgba([255, 0, 0, 255]));   // Red
        test_image.put_pixel(1, 0, Rgba([0, 255, 0, 255]));   // Green
        test_image.put_pixel(0, 1, Rgba([0, 0, 255, 255]));   // Blue
        test_image.put_pixel(1, 1, Rgba([255, 255, 0, 255])); // Yellow
        
        let dynamic_img = DynamicImage::ImageRgba8(test_image);
        
        // Resize the image to 4x4
        let resized = dynamic_img.resize(4, 4, FilterType::Nearest);
        
        // Verify dimensions
        assert_eq!(resized.width(), 4);
        assert_eq!(resized.height(), 4);
        
        // Convert back to ImageBuffer for pixel testing
        let resized_buffer = resized.to_rgba8();
        
        // Test that the resize worked correctly (nearest neighbor should duplicate pixels)
        // Check a few sample pixels
        assert_eq!(resized_buffer.get_pixel(0, 0), &Rgba([255, 0, 0, 255]));   // Red
        assert_eq!(resized_buffer.get_pixel(2, 0), &Rgba([0, 255, 0, 255]));   // Green
        assert_eq!(resized_buffer.get_pixel(0, 2), &Rgba([0, 0, 255, 255]));   // Blue
        assert_eq!(resized_buffer.get_pixel(2, 2), &Rgba([255, 255, 0, 255])); // Yellow
    }
}
