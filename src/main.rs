extern crate image;

use std::fs::File;
use std::path::Path;

use image::GenericImage;

fn process(p: &Path) {
    println!("path {:?}", p.file_name().unwrap());
    let img = image::open(p).unwrap();
    let img = image::imageops::unsharpen(&img, 5.0, 1);

}

fn process_thread(p: &Path) {
    let p = p.clone();
    std::thread::spawn(move || {
        println!("path {:?}", p.file_name().unwrap());
        let img = image::open(p).unwrap();
        let img = image::imageops::unsharpen(&img, 5.0, 1);
    });
}

fn main() {
    let paths = std::fs::read_dir(&Path::new("images")).unwrap();
    for p in paths.take(4) {
        process(&(p.unwrap().path()));
    }

    
    let _ = process(&Path::new("test.jpg"));

    
    // Use the open function to load an image from a Path.
    // ```open``` returns a dynamic image.

    // // The dimensions method returns the images width and height
    // println!("dimensions {:?}", img.dimensions());

    // // The color method returns the image's ColorType
    // println!("{:?}", img.color());

    //let ref mut fout = File::create(&Path::new("out.jpg")).unwrap();

    // Write the contents of this image to the Writer in PNG format.
    //let _ = img.save(&Path::new("out.jpg")).unwrap();
}
