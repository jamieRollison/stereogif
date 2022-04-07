pub mod frame;
pub mod process;
use frame::Frame;
extern crate show_image;
use show_image::{ImageView, ImageInfo, create_window, PixelFormat, WindowProxy, event::WindowEvent, glam::Vec2};
extern crate gif;
use std::fs::File;
use std::thread;
use std::sync::mpsc::*;
use std::thread::{JoinHandle};

/// this function will take in frames and create a gif.
/// it will output to the specified filepath.
pub fn make(mut frames: Vec<Frame>, filename: &String) {
  let frame_options = show_image::WindowOptions {
    preserve_aspect_ratio: true,
    background_color: show_image::Color{red: 0., green: 0., blue: 0., alpha: 0.},
    start_hidden: false,
    size: Some([(frames[0].width() as f32 / 2_f32.sqrt() ) as u32, (frames[0].height() as f32 / 2_f32.sqrt()) as u32]),
    resizable: false,
    borderless: false,
    overlays_visible: true,
    default_controls: false,
  };

  println!("You will be shown the images in order. Click the pivot on each one (try to be exact as possible)");
  let window = create_window("Please click the point about which the image should pivot", frame_options).unwrap();
  for frame in frames.iter_mut() {
    render_frame(frame, &window).unwrap();
    choose_pivots(frame, &window);
  }
  window.run_function(|w| {w.destroy();});
  
  process::align(&mut frames);
  output(frames,filename);  
}

/// changes the existing window instead of creating a new one
pub fn render_frame(frame: &Frame, window: &WindowProxy) -> Result<(), Box<dyn std::error::Error>> {
  let data = &frame.pixels()[..];
  let image = 
    ImageView::new(ImageInfo::new(
      PixelFormat::Rgb8, 
      frame.width().into(), 
      frame.height().into()), 
      data
    );
  window.set_image("image", image)?;
  Ok(())
}

/// have the user choose the pivot point
/// change the pivot pixel to the coordinates of the chosen pixel on the image.
/// this code is some of the worst i've ever written.
pub fn choose_pivots(frame: &mut Frame, window: &WindowProxy) {
  let (coord_tx, coords_recv): (Sender<Vec2>, Receiver<Vec2>) = channel();
  window.add_event_handler(move |_,event,_| {
    match event {
      WindowEvent::MouseButton(mouse_event) => {
        if mouse_event.button.is_left() && mouse_event.state.is_pressed() {
          match coord_tx.send(mouse_event.position) {
            Ok(_) => {},
            Err(_) => {}
          } 
        }
      },
      _ => {}
    }
  }).unwrap();
  let vec2_coords = match coords_recv.recv() {
    Ok(thing) => thing,
    Err(_) => {panic!("You will need to click on the pivot point."); }
  };
  let pivot_pixel = ((vec2_coords.x as f32 * 2_f32.sqrt()) as u16, (vec2_coords.y as f32 * 2_f32.sqrt()) as u16);
  // println!("{:?}", pivot_pixel);
  frame.set_pivot(pivot_pixel);
}

fn output(frames: Vec<Frame>, filename: &String) {
  // prepare to encode file
  let mut output_file = File::create(filename).unwrap();
  let mut encoder = gif::Encoder::new(
    &mut output_file, 
    frames[0].width(), 
    frames[0].height(),
    &[]).unwrap();
  encoder.set_repeat(gif::Repeat::Infinite).unwrap();

  // measure the start time for debugging purposes
  // let start = std::time::Instant::now();

  // split up the frames so they will be encoded in parallel
  let (handles, receiver) = split(frames);

  // in the meantime, the user can look at the image

  // wait for the frames to finish being encoded
  for handle in handles {
    handle.join().unwrap();
    // println!("done encoding a frame");
  }
  // let elapsed = start.elapsed();
  // println!("output takes: {} ms", elapsed.as_millis());
  println!("Saving to {}", &filename);
  // the frames may come back out of order. they need to be sorted
  let mut frames_in_order = sort_received(receiver);

  // write the frames to the output file
  for gif_frame in frames_in_order.iter() {
    encoder.write_frame(gif_frame).unwrap();
  }

  // add the frames (but not the beginning or end) to create a looping effect
  // this can be destructive because it is the last thing the program does
  frames_in_order.pop().unwrap();
  frames_in_order.reverse();
  frames_in_order.pop();
  for gif_frame in frames_in_order.iter() {
    encoder.write_frame(gif_frame).unwrap();
  }
}

// encodes the frames as a GIF frame. Multithreading speeds this up significantly
fn split(frames: Vec<Frame>) -> (Vec<JoinHandle<()>>, Receiver<(u8, gif::Frame<'static>)>) {
  let mut join_handles: Vec<JoinHandle<()>> = Vec::new();
  let (tx,rx) : (Sender<(u8, gif::Frame)>, Receiver<(u8, gif::Frame)>) = channel();
  println!("Encoding the frames as a gif. This should only take a few seconds");
  for frame in frames {
    let tx_ = tx.clone();
    join_handles.push(thread::spawn(move || {
      // println!("encoding frame...");
      let mut gif_frame = gif::Frame::from_rgb_speed(frame.width(), frame.height(), &mut frame.pixels(), 30);
      gif_frame.delay = 12; // we chose .12 s per frame because it looks nice :)
      tx_.send((frame.order(), gif_frame)).unwrap();
    }));
  }
  drop(tx);
  (join_handles, rx)
}

fn sort_received(received: Receiver<(u8, gif::Frame<'static>)>) -> Vec<gif::Frame<'static>> {
  let mut received_as_vec: Vec<(u8, gif::Frame)> = Vec::new();
  let mut frame_vector: Vec<gif::Frame<'static>> = Vec::new();
  for item in received {
    received_as_vec.push(item);
  }
  received_as_vec.sort_by(|a,b|a.0.cmp(&b.0));
  for frame in received_as_vec {
    frame_vector.push(frame.1);
  }
  frame_vector
}