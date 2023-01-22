use std::collections::HashSet;
use std::env::current_dir;
use std::fs::read_dir;
use std::io::Result;
use image::{DynamicImage, GenericImageView, GenericImage};

fn main() -> Result<()>{
    let cwd = current_dir().expect("Could not get current directory");
    let files = read_dir(&cwd).expect("Could not read files in current directory");

    let mut images: Vec<DynamicImage> = vec![];
    let mut dims: HashSet<(u32, u32)> = HashSet::new();

    for entry in files {
        let path = entry?.path();

        if path.is_dir() { continue }
        if !(path.extension().unwrap_or(std::ffi::OsStr::new("")) == "png") { continue }

        let img = image::open(path).expect("Could not open Image");
        let dim = img.dimensions();
        dims.insert(dim);
        images.push(img);
    }

    if dims.len() > 1
    {
        println!("Images are not all the same size");
        return Ok(());
    }

    let dims = dims.iter().next().unwrap();

    println!("Number of images: {}", images.len());
    println!("Dims: {:?}", dims);

    let images_per_axis = (images.len() as f32).sqrt().ceil() as u32;
    let new_dim = images_per_axis * dims.0;

    let mut atlas = image::DynamicImage::new_rgba8(new_dim, new_dim );

    for (index, image) in images.iter().enumerate() {
        let x_offset = (index as u32 % images_per_axis) * dims.0;
        let y_offset = (index as u32 / images_per_axis) * dims.1;
    
    
        atlas.copy_from(image, x_offset, y_offset).expect("");
    }

    atlas.save(cwd.join("atlas.png")).expect("Could not save atlas");

    Ok(())
}
