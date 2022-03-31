extern crate jpeg_decoder;
use jpeg_decoder::Decoder;
use std::fs::File;
use std::io::BufReader;

/// our personal way to encapsulate image data.
/// most of it will be taken from the one outlined in jpeg_decoder, so it aligns mostly with that.
/// however, it will also include the things we will need to put the frames together.
pub struct Frame {
  metadata: jpeg_decoder::ImageInfo,
  height: u16,
  width: u16,

  pixels: Vec<u8>,
  pivot_pixel: (u16,u16)
}

impl Frame {
  pub fn new(filename: String) -> Frame {
    let (pixels, metadata) = Frame::read(filename);
    let frame = Frame {
        metadata,
        height: metadata.height,
        width: metadata.width,
        pixels,
        pivot_pixel: (metadata.width / 2, metadata.height / 2)
    };
    frame
  }

  /// decodes an image
  fn read(filename: String) -> (Vec<u8>, jpeg_decoder::ImageInfo) {
    let file = File::open(filename).expect("failed to open file");
    let mut decoder = Decoder::new(BufReader::new(file));
    let pixels = decoder.decode().expect("failed to decode image");
    let metadata = decoder.info().unwrap();
    (pixels, metadata)
  }

  /// Getter for the x value of the pivot.
  pub fn pivot_x(&self) -> u16 {
    self.pivot_pixel.0.clone()
  }

  /// Getter for the y value of the pivot.
  pub fn pivot_y(&self) -> u16 {
    self.pivot_pixel.1.clone()
  }

  /// Getter for pixels
  pub fn pixels(&self) -> Vec<u8> {
    self.pixels.clone()
  }

  /// getter for height
  pub fn height(&self) -> u16 {
    self.height
  }

  /// getter for width
  pub fn width(&self) -> u16{
    self.width
  }

  /// setter for pivot point
  pub fn set_pivot(&mut self, pivot: (u16, u16)) {
    self.pivot_pixel = pivot;
  }
}

