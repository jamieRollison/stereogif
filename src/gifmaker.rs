pub mod frame;
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
pub fn make(mut frames: Vec<Frame>, filename: String) {
  let frame_options = show_image::WindowOptions {
    preserve_aspect_ratio: true,
    background_color: show_image::Color{red: 0., green: 0., blue: 0., alpha: 0.},
    start_hidden: false,
    size: Some([(frames.get(0).unwrap().width() as f32 / 2_f32.sqrt() ) as u32, (frames.get(0).unwrap().height() as f32 / 2_f32.sqrt()) as u32]),
    resizable: false,
    borderless: false,
    overlays_visible: true,
    default_controls: false,
  };

  let window = create_window("Please click the point about which the image should pivot", frame_options).unwrap();
  for frame in frames.iter_mut() {
    render_frame(frame, &window).unwrap();
    choose_pivots(frame, &window);
  }
  window.run_function(|w| {w.destroy();});
  align(&mut frames);

  let start = std::time::Instant::now();
  output(frames,filename);
  let elapsed = start.elapsed();
  println!("output takes: {} ms", elapsed.as_millis());
}

/// overloaded render, changes the existing window instead of creating a new one
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
  println!("{:?}", pivot_pixel);
  frame.set_pivot(pivot_pixel);
}

fn output(frames: Vec<Frame>, filename: String) {
  let mut output_file = File::create(filename).unwrap();
  let mut encoder = gif::Encoder::new(
    &mut output_file, 
    frames.get(0).unwrap().width(), 
    frames.get(0).unwrap().height(),
    &[]).unwrap();
  encoder.set_repeat(gif::Repeat::Infinite).unwrap();

  let (handles, receiver) = split(frames);
  for handle in handles {
    handle.join().unwrap();
    println!("done encoding a frame");
  }

  let mut frames_in_order = sort_received(receiver);

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

fn split(frames: Vec<Frame>) -> (Vec<JoinHandle<()>>, Receiver<(u8, gif::Frame<'static>)>) {
  let mut join_handles: Vec<JoinHandle<()>> = Vec::new();
  let (tx,rx) : (Sender<(u8, gif::Frame)>, Receiver<(u8, gif::Frame)>) = channel();
  println!("Outputting frames to gif. This may take a few minutes");
  for frame in frames {
    let tx_ = tx.clone();
    join_handles.push(thread::spawn(move || {
      println!("encoding frame...");
      let mut gif_frame = gif::Frame::from_rgb(frame.width(), frame.height(), &mut frame.pixels());
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

/// BETTER METHOD:
/// Once we have all the points of each image we can calculate the crop of 
/// each image independently of each other, and apply the crops on a seperate
/// thread for each image. That way, we don't have to compare images to each 
/// other or anything. 
/// 
/// Given the vector of alignment points, figure out the delta / difference 
/// between each alignment point, then from the height and width of each image,
/// determine the crop that will be applied to each image.
/// 
/// We will have to develop the algorithm / math determining the crop of each
/// image, but in the end, we should have a set of images of the same size
/// with all the alignment points at the same point on the image.


/// this function takes each frame and lines them up based on pivot point.
/// it does this by:
/// 1) start with the first frame. it will be the base.
/// 2) take the second frame. Consider the deltas (x and y) between the first pivot and the second.
///    if the delta is positive (first - second), the first dy rows of the first frame will be dropped and the first dx entries
///    in each column will be dropped. then the last dy rows and dx columns of the remaining rows will be dropped on the second frame.
///    if the delta is negative, the reverse will happen (ie the first row will have its end parts dropped).
///    the crops are done in the functions crop_top(), crop_bottom(), crop_left(), and crop_right().
/// 3) compare frame n to frame n-1 by repeating step 2 for each frame.
fn align(frames: &mut Vec<Frame>) {
  // let mut last_frame = frames.get_mut(0).unwrap();
  // let mut current_frame: &mut Frame;
  // for index in 1..frames.len() {
  //   current_frame = frames.get_mut(index).unwrap();
  //   let (delta_x, delta_y) = (last_frame.pivot_x() - current_frame.pivot_x(), 
  //                                       last_frame.pivot_y() - current_frame.pivot_y());
                            
  //   if delta_x < 0 {
  //     crop_top(last_frame, &delta_x);
  //     crop_bottom(current_frame, &delta_x);
  //   }
  //   last_frame = current_frame;
  // }
}

/// see documentation of align().
fn crop_top(frame: &mut Frame, x: &u16) {

}

/// see documentation of align().
fn crop_bottom(frame: &mut Frame, x: &u16) {

}

/// see documentation of align().
fn crop_left(frame: &mut Frame, y: &u16) {

}

/// see documentation of align().
fn crop_right(frame: &mut Frame, y: &u16) {

}