### How to  
- Clone the project. `git clone https://github.com/jamieRollison/stereogif.git`
- You'll need to specify an input directory and an output file. There are a number of examples you can use in the repo. Example usage is also below. Feel free to inspect the images before they're processed for comparison.
  - Example: `cargo run "input_images_folder" "output.gif"`
- The images in the folder will appear one at a time. Click the pivot point in each image, and try to be as precise as possible.
  - Example pivot points include a subject's nose or eye, or an object in focus (like the balloon).
  - You can play around with clicking different parts of the image to see how it crops if you'd like, but it probably won't turn out as intended.
- The images will be processed. The console output should specify when the program is done.
- Open the image! It's wherever the output was specfied to be.

### Example run commands
- `cargo run example_inputs/owen_cup output/cup.gif` We used this one for initial testing
- `cargo run example_inputs/debug_crosshair output/cross.gif` We used this for debugging. If you pick the center of the crosshair as the pivot, it demonstrates how the program crops to keep that part in focus.
- `cargo run example_inputs/owen_computer output/computer.gif`
- `cargo run example_inputs/outside output/outside.gif` this one doesn't work quite as well due to the shot composition