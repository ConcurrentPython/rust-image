extern crate image;

fn process(p: &std::path::Path) {
    println!("path {:?}", p.file_name().unwrap());
    let img = image::open(p).unwrap();
    let _ = image::imageops::unsharpen(&img, 5.0, 1);
}

fn main() {
    let paths = std::fs::read_dir(&std::path::Path::new("images")).unwrap();
    for p in paths {
        process(&(p.unwrap().path()));
    }
}
