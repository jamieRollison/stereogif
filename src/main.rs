pub mod frame;
use frame::Frame;


#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
  Frame::new("C:/Users/james/Documents/cs128/cs128env/src/stereogif/test_data/owen_cup_1.jpg".into());

  Ok(())
}

