use std::collections::HashSet;
use std::env::current_dir;
use std::fs::read_dir;
use std::io::Result;
use image::{DynamicImage, GenericImageView};

fn main() -> Result<()>{
    let cwd = current_dir().expect("Could not get current directory");
    let files = read_dir(cwd).expect("Could not read files in current directory");

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

    println!("All images are the same size");
    println!("Number of images: {}", images.len());
    println!("Dims: {:?}", dims.iter().next().unwrap());

    Ok(())
}
