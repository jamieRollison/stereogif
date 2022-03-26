Group/Project Name
stereogif

### Group member names and NetIDs
Cole Zimmerman (coleaz2)
Jamie Rollison (jamesrr3)
Owen Cushing (owenpc2)

### Project Introduction
The Nimslo 3D is a camera with 4 lenses that takes 4 simultaneous exposures. When the images are developed digitally, they can be put together to make a 3D gif, but "putting together" requires manually making the gif in photoshop. Our project automates this process, with several customization tools and algorithms for developing these sorts of images and other stereoscopic imagery.

<<<<<<< HEAD
### System Overview
Please provide a moderate-length technical description of the major components of your project. This should also function as a sort of ‘roadmap’ for tasks you need to complete for your project to be functional.

### Possible Challenges
A major hurdle will be representing JPG data in a way that can be modified, and outputting our modified data in the form of a GIF. 

### References
The formula we used for finding the maximum information or "energy" in the image was given to us in 128.
=======
System Overview
The user will input sRGB jpegs. We will use a crate to decode these. The program will then prompt the user to click the "pivot" points on each image, and handle these mouse events. Then the program will crop and realign the images so the pivot points are in the same place, so using the images as frames gives a stereoscopic illusion as a rotation in 3d space. Then the program outputs these frames as a gif.

#Possible Challenges
A major hurdle will be representing JPG data in a way that can be modified, and outputting our modified data in the form of a GIF. It may also be difficult to meaningfully handle mouse events.

#References
none (yet)
>>>>>>> f05f6ec265e8916aa3b654ce376b8e8f49868e0e
