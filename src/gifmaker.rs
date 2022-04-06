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

  let window = create_window("Please click the point about which the image should pivot", frame_options).unwrap();
  for frame in frames.iter_mut() {
    render_frame(frame, &window).unwrap();
    choose_pivots(frame, &window);
  }
  window.run_function(|w| {w.destroy();});
  align(&mut frames);

  output(frames,filename);  
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
  let start = std::time::Instant::now();

  // split up the frames so they will be encoded in parallel
  let (handles, receiver) = split(frames);

  // in the meantime, the user can look at the image

  // wait for the frames to finish being encoded
  for handle in handles {
    handle.join().unwrap();
    println!("done encoding a frame");
  }
  let elapsed = start.elapsed();
  println!("output takes: {} ms", elapsed.as_millis());

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
  println!("Outputting frames to gif. This may take a few minutes");
  for frame in frames {
    let tx_ = tx.clone();
    join_handles.push(thread::spawn(move || {
      println!("encoding frame...");
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
/// 
/// 
/// cole method:
/// crop by:
/// for all images, find min distance between pivot and left, right, up, and down
/// crop by cutting from the minimum distance to the edge for each image 
fn align(frames: &mut Vec<Frame>) {
  let top_distance = find_distances_from_top(frames);
  let bot_distance = find_distances_from_bottom(frames);
  let left_distance = find_distances_from_left(frames);
  let right_distance = find_distances_from_right(frames);
  crop_top(frames, top_distance);
  crop_bottom(frames, bot_distance);
  crop_left(frames, left_distance);
  crop_right(frames, right_distance);
  println!("new dimensions are {} by {}", frames[0].width(), frames[0].height());
}

fn find_distances_from_top(frames: &mut Vec<Frame>) -> Vec<u16> {
  let mut min_distances: Vec<u16> = Vec::new();
  let mut y_values: Vec<u16> = Vec::new();
  for frame in frames.iter() {
    y_values.push(frame.pivot_y());
  }
  let min = y_values.iter().min().unwrap();
  for value in y_values.iter() {
    min_distances.push(value - min);
  }
  min_distances
}

fn find_distances_from_bottom(frames: &mut Vec<Frame>) -> Vec<u16> {
  let mut min_distances: Vec<u16> = Vec::new();
  let mut y_values: Vec<u16> = Vec::new();
  for frame in frames.iter() {
    y_values.push(frame.height() - frame.pivot_y());
  }
  let min = y_values.iter().min().unwrap();
  for value in y_values.iter() {
    min_distances.push(value - min);
  }
  min_distances
}

fn find_distances_from_left(frames: &mut Vec<Frame>) -> Vec<u16> {
  let mut min_distances: Vec<u16> = Vec::new();
  let mut x_values: Vec<u16> = Vec::new();
  for frame in frames.iter() {
    x_values.push(frame.pivot_x());
  }
  let min = x_values.iter().min().unwrap();
  for value in x_values.iter() {
    min_distances.push(value - min);
  }
  min_distances
}

fn find_distances_from_right(frames: &mut Vec<Frame>) -> Vec<u16> {
  let mut min_distances: Vec<u16> = Vec::new();
  let mut x_values: Vec<u16> = Vec::new();
  for frame in frames.iter() {
    x_values.push(frame.width() - frame.pivot_x());
  }
  let min = x_values.iter().min().unwrap();
  for value in x_values.iter() {
    min_distances.push(value - min);
  }
  min_distances
}

/// see documentation of align().
fn crop_top(frames: &mut Vec<Frame>, to_cut: Vec<u16>) {
  for index in 0..frames.len() {
    println!("cutting {} lines", to_cut[index]);
    println!("height is {} so new height will be {}", frames[index].height(), frames[index].height() - to_cut[index]);
    let amount_to_cut = (frames[index].width() as u64 * to_cut[index] as u64 * 3) as usize;
    frames[index].decrease_height(to_cut[index]);
    *frames[index].pixels_mut() = frames[index].pixels_mut()[amount_to_cut..].to_vec();
  }
}

/// see documentation of align().
fn crop_bottom(frames: &mut Vec<Frame>, to_cut: Vec<u16>) {
  for index in 0..frames.len() {
    let amount_to_cut = frames[index].width() as u64 * to_cut[index] as u64 * 3;
    let bottom_edge = frames[index].pixels().len() - amount_to_cut as usize;
    frames[index].decrease_height(to_cut[index]);
    *frames[index].pixels_mut() = frames[index].pixels_mut()[..bottom_edge].to_vec();
    println!("height is now {}", frames[index].height());
  }
}

/// see documentation of align().
fn crop_left(frames: &mut Vec<Frame>, to_cut: Vec<u16>) {
  for index in 0..frames.len() {
    println!("Cropping frame {}", index);
    println!("cutting {} columns from the left", to_cut[index]);
    let frame = &mut frames[index];
    for row in 0..frame.height() {
      let start = 3 * (row as usize) * (frame.width() - to_cut[index]) as usize;
      frame.pixels_mut().drain(start..(start + 3 * to_cut[index] as usize));
    }
    frame.decrease_width(to_cut[index]);
  }
}

/// see documentation of align().
fn crop_right(frames: &mut Vec<Frame>, to_cut: Vec<u16>) {
  for index in 0..frames.len() {
    println!("Cropping frame {}", index);
    println!("cutting {} columns from the right", to_cut[index]);
    let frame = &mut frames[index];
    frame.decrease_width(to_cut[index]);
    for row in 0..frame.height() {
      let start = 3 * (row as usize+1) * (frame.width() - to_cut[index]) as usize;
      frame.pixels_mut().drain(start..(start + 3 * to_cut[index] as usize));
    }
    println!("width is now {}", frames[index].width());
  }
}