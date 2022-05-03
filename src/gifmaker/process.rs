use crate::Frame;
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
pub fn align(frames: &mut Vec<Frame>) {
    // let start = std::time::Instant::now();
    crop_top(frames, vec![208; 4]);
    let top_distance = find_distances_from_top(frames);
    let bot_distance = find_distances_from_bottom(frames);
    let left_distance = find_distances_from_left(frames);
    let right_distance = find_distances_from_right(frames);
    crop_top(frames, top_distance);
    crop_bottom(frames, bot_distance);
    crop_left(frames, left_distance);
    crop_right(frames, right_distance);
    // let elapsed = start.elapsed();
    // println!("cropping takes: {} ms", elapsed.as_millis());
    // println!("new dimensions are {} by {}", frames[0].width(), frames[0].height());
  }
  
  fn find_distances_from_top(frames: &Vec<Frame>) -> Vec<u16> {
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
  
  fn find_distances_from_bottom(frames: &Vec<Frame>) -> Vec<u16> {
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
  
  fn find_distances_from_left(frames: &Vec<Frame>) -> Vec<u16> {
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
  
  fn find_distances_from_right(frames: &Vec<Frame>) -> Vec<u16> {
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
    }
  }
  
  /// see documentation of align().
  fn crop_left(frames: &mut Vec<Frame>, to_cut: Vec<u16>) {
    for index in 0..frames.len() {
      let frame = &mut frames[index];
      for row in 0..frame.height() {
        let start = 3 * (row as usize) * (frame.width() - to_cut[index]) as usize;
        frame.pixels_mut().drain(start..(start + 3 * to_cut[index] as usize));
      }
      frame.decrease_width(to_cut[index]);
    }
  }
  
  /// Takes the amount in to cut off of each frame.
  /// it does this by clipping every width-th element out of each array.
  fn crop_right(frames: &mut Vec<Frame>, to_cut: Vec<u16>) {
    // println!("{:#?}", to_cut);
    for index in 0..frames.len() {
      let frame = &mut frames[index];
      for row in 0..frame.height() {
        let start = 3 * (row as usize + 1) * (frame.width() - to_cut[index]) as usize;
        frame.pixels_mut().drain(start..(start + 3 * to_cut[index] as usize));
      }
      frame.decrease_width(to_cut[index]);
    }
  }