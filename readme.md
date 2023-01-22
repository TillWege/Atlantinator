# Atlantinator
A tool for combining a set of images into a single texture-atlas.

# How to install
With rust and cargo installed, you can install Atlantinator by running

``cargo install --git https://github.com/TillWege/Atlantinator.git``

After that you should be able to use it whereever, simply by running ``atlantinator`` in your terminal of choice

# How to use
Run ``Atlantinator`` in a Folder with the set of images you want to combine into a single texture-atlas. It will create a atlast.png file with all of them stitched together into one.

# Current limitations
Atlantinator is currently limited in the following regards:
- Only works on PNG-Files
- Only works if all images are the same Dimensions
- Always saves the Output as atlas.png in the current working directory