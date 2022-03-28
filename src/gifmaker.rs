pub mod frame;
use std::sync::mpsc::*;
extern crate show_image;
use show_image::{ImageView, ImageInfo, create_window, PixelFormat, WindowProxy, event::WindowEvent, glam::Vec2};
use frame::Frame;
extern crate image;

/// this function will take in frames and create a gif.
/// it will output to the specified filepath.
pub fn output(frames: &mut Vec<Frame>, filename: String) {
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
  for frame in frames {
    render_frame(frame, &window).unwrap();
  choose_pivots(frame, &window);
  }
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
          println!("{:?}", mouse_event.position);
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
  frame.set_pivot(pivot_pixel);
}

/// this function takes each frame and lines them up based on pivot point.
/// it does this by:
/// 1) start with the first frame. it will be the base.
/// 2) take the second frame. Consider the deltas (x and y) between the first pivot and the second.
///    if the delta is positive (first - second), the first dy rows of the first frame will be dropped and the first dx entries
///    in each column will be dropped. then the last dy rows and dx columns of the remaining rows will be dropped on the decond frame.
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