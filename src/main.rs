mod gifmaker;
use gifmaker::frame::Frame;
use std::env;
use std::fs;
// use std::thread;

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


// this was for batch processing but it isn't user friendly

// fn main()-> Result<(), Box<dyn std::error::Error>> {
//   let args: Vec<String> = env::args().collect();
//   if args.len() == 1 {
//     panic!("You are missing command line arguments. See RUN.md for usage guide.");
//   }
//   let input_directory = args[1].to_string();
//   let output_directory = args[2].to_string();
//   let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();

//   let base_directory = fs::read_dir(input_directory).unwrap();
//   let mut order_counter: u8 = 0;

//   if args.contains(&"-m".into()) {
//     let mut gif_counter = 0;
//     for path in base_directory {
//       let mut frames: Vec<Frame> = Vec::new();
//       let child_directory = fs::read_dir(path.unwrap().path()).unwrap();
      
//       let filename = format!("{}\\{}.gif", output_directory, gif_counter);
//       gif_counter += 1;
//       for image in child_directory {
//         order_counter += 1;
//         frames.push(Frame::new(image.unwrap().path().display().to_string(), order_counter));
//       }
//       handles.push(thread::spawn(move || {gifmaker::make(frames, &filename);}));
//     }
//     for handle in handles {
//       handle.join().unwrap();
//     }
//   } else {
//     let mut frames: Vec<Frame> = Vec::new();
//     for path in base_directory {
//       order_counter += 1;
//       frames.push(Frame::new(path.unwrap().path().display().to_string(), order_counter));
//     }
//     gifmaker::make(frames, &output_directory);
//   }
//   Ok(())
// }