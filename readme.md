# Atlantinator
A tool for combining a set of images into a single texture-atlas.

# How to install
With rust and cargo installed, you can install Atlantinator by running

``cargo install atlantinator``

Alternatively you can install the current head of this repository with

``cargo install --git https://github.com/TillWege/Atlantinator.git``

After that you should be able to use it where ever, simply by running ``atlantinator`` in your terminal of choice

# How to use
Run ``atlantinator`` in a folder with the set of images you want to combine into a single texture-atlas. It will create an ``atlas.png`` file with all of the images stitched together into one.

# Current limitations
Atlantinator is currently limited in the following regards:
- Only works on PNG-files
- Only works if all images are the same dimensions
- Always saves the output as atlas.png in the current working directory