Group/Project Name
stereogif

Group member names and NetIDs
Cole Zimmerman (coleaz2)
Jamie Rollison (jamesrr3)
Owen Cushing (owenpc2)

#Project Introduction
The Nimslo 3D is a camera with 4 lenses that takes 4 simultaneous exposures. When the images are developed digitally, they can be put together to make a 3D gif, but "putting together" requires manually making the gif in photoshop. Our project automates this process, with several customization tools and algorithms for developing these sorts of images and other stereoscopic imagery.

System Overview
The user will input sRGB jpegs. We will use a crate to decode these. The program will then prompt the user to click the "pivot" points on each image, and handle these mouse events. Then the program will crop and realign the images so the pivot points are in the same place, so using the images as frames gives a stereoscopic illusion as a rotation in 3d space. Then the program outputs these frames as a gif.

#Possible Challenges
A major hurdle will be representing JPG data in a way that can be modified, and outputting our modified data in the form of a GIF. It may also be difficult to meaningfully handle mouse events.

#References
none (yet)
