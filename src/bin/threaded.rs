extern crate image;

fn process(p: &std::path::Path) {
    println!("path {:?}", p.file_name().unwrap());
    let img = image::open(p).unwrap();
    let _ = image::imageops::unsharpen(&img, 5.0, 1);
}

fn main() {
    let paths = std::fs::read_dir(&std::path::Path::new("images")).unwrap();

    // Note that map creates a (lazy) iterator, so we have to collect
    // the iterator in to a vector to start the threads before we join any.
    let threads: Vec<_> = paths.map({
        | p | {
            let p = p.unwrap().path();
            let j: std::thread::JoinHandle<_> = std::thread::spawn(
                move || process(&p));
            j
        }
    }).collect();
    for t in threads {
        t.join().unwrap();
    }
}
