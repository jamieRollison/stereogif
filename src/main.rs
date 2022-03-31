mod gifmaker;
use gifmaker::frame::Frame;
use std::env;
use std::fs;

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
  // get command line arguments
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);
  if args.len() == 1 {
    panic!("Usage: -i \"path\": specify a directory to pull images from. -o \"file\": specify an output file for the GIF.");
  }
  let input_directory = match args.iter().position(|flag| flag == "-i") {
    Some(index) => { args.get(index+1).unwrap().to_string() },
    None => "".to_string()
  };
  let output_directory = match args.iter().position(|flag| flag == "-o") {
    Some(index) => { args.get(index+1).unwrap().to_string() },
    None => "".to_string()
  };

  let paths = fs::read_dir(input_directory).unwrap();

  let mut frames: Vec<Frame> = Vec::new();

  for path in paths {
    frames.push(Frame::new(path.unwrap().path().display().to_string()));
  }
  
  gifmaker::make(&mut frames, output_directory);
  Ok(())
}

