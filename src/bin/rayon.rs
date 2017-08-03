extern crate image;
extern crate rayon;

fn process(p: &std::path::Path) {
    println!("path {:?}", p.file_name().unwrap());
    let img = image::open(p).unwrap();
    let _ = image::imageops::unsharpen(&img, 5.0, 1);
}

fn main() {
    let paths = std::fs::read_dir(&std::path::Path::new("images")).unwrap();

    rayon::scope(|scope| {
        for p in paths {
            scope.spawn(move |_| process(&(p.unwrap().path())));
        }
    });
}
