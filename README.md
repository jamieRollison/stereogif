## Group/Project Name
stereogif

## Group member names and NetIDs
Cole Zimmerman (coleaz2)
Jamie Rollison (jamesrr3)
Owen Cushing (owenpc2)

## Project Introduction
The Nimslo 3D is a camera with 4 lenses that takes 4 simultaneous exposures. When the images are developed digitally, they can be put together to make a 3D gif, but "putting together" requires manually making the gif in photoshop. Our project automates this process, with several customization tools and algorithms for developing these sorts of images and other stereoscopic imagery.

## System Overview
The user puts in a directory, and everything in the directory is found and used. We use a JPEG decoder crate to take each file and make it into a frame. Then the user is prompted to click the point on each frame that the gif should pivot around (this is usually an object such as a head, which will move slightly from frame to frame). The frames are aligned and cropped such that the pivot points line up. Then the frames are encoded into GIF format.

## Possible Challenges
A major hurdle will be representing JPG data in a way that can be modified, and outputting our modified data in the form of a GIF. 

## References
