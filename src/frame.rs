extern crate show_image;
extern crate jpeg_decoder;
use jpeg_decoder::Decoder;
use std::fs::File;
use std::io::BufReader;

use show_image::{ImageView, ImageInfo, create_window, PixelFormat};


/// our personal way to encapsulate image data.
/// most of it will be taken from the one outlined in jpeg_decoder, so it aligns mostly with that.
/// however, it will also include the things we will need to put the frames together.
pub struct Frame {
  metadata: jpeg_decoder::ImageInfo,
  height: u16,
  width: u16,

  pixels: Vec<u8>,
  pivot_pixel: u8
}

impl Frame {
  pub fn new(filename: String) -> Frame {
    let (pixels, metadata) = Frame::read(filename);
    Frame {
        metadata,
        height: metadata.height,
        width: metadata.width,
        pixels,
        pivot_pixel: Frame::choose_pivot()
    }
  }

  /// decodes an image
  fn read(filename: String) -> (Vec<u8>, jpeg_decoder::ImageInfo) {
    let file = File::open(filename).expect("failed to open file");
    let mut decoder = Decoder::new(BufReader::new(file));
    let pixels = decoder.decode().expect("failed to decode image");
    let metadata = decoder.info().unwrap();
    (pixels, metadata)
  }

  /// have the user choose the pivot point
  fn choose_pivot() -> u8 {
    0
  }

  pub fn render(&self) -> Result<(), Box<dyn std::error::Error>> {
    let image = 
      ImageView::new(ImageInfo::new(
        PixelFormat::Rgb8, 
        self.width.into(), 
        self.height.into()), 
        &self.pixels[..]
       );
    // Create a window with default options and display the image.
    let window = create_window("image", Default::default())?;
    window.set_image("image-001", image)?;
    window.wait_until_destroyed().unwrap();
    Ok(())
  }
}