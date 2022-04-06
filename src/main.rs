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
    panic!("Usage: \"directory path\": specify a directory to pull images from, \"file\": specify an output file for the GIF.");
  }
  let input_directory = args.get(1).unwrap().to_string();
  let output_directory = args.get(2).unwrap().to_string();

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

// fn main() {
//   let mut vector: Vec<u16> = Vec::new();
//   for i in 0..144 {
//     vector.push(i);
//   }
//   let mut counter = 0;
//   for _row in 0..9 {
//     for _col in 0..16 {
//       print!("{} ", vector[counter]);
//       if counter < 10 {
//         print!(" ");
//       }
//       if counter < 100 {
//         print!(" ");
//       }
//       counter += 1;
//     }
//     println!();
//   }
//   for row in 0..9 {
//     let start = (row + 1 as usize) * (16 - 2) as usize;
//     vector.drain(start..(start + 2 as usize));
//   }
//   println!();
//   counter = 0;
//   for _row in 0..9 {
//     for _col in 0..14 {
//       if counter < 10 {
//         print!(" ");
//       }
//       if counter < 94 {
//         print!(" ");
//       }
//       print!("{} ", vector[counter]);
//       counter += 1;
//     }
//     println!();
//   }
// }