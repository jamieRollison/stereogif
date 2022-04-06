mod gifmaker;
use gifmaker::frame::Frame;
use std::env;
use std::fs;

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
  // get command line arguments
  let args: Vec<String> = env::args().collect();
  // println!("{:?}", args);
  if args.len() == 1 {
    panic!("Usage: \"directory path\": specify a directory to pull images from, \"file\": specify an output file for the GIF.");
  }
  let input_directory = args[1].to_string();
  let output_directory = args[2].to_string();

  let paths = fs::read_dir(input_directory).unwrap();

  let mut frames: Vec<Frame> = Vec::new();

  let mut order_counter: u8 = 0;
  for path in paths {
    order_counter += 1;
    frames.push(Frame::new(path.unwrap().path().display().to_string(), order_counter));
  }
  
  gifmaker::make(frames, &output_directory);
  Ok(())
}