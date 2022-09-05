use std::path::Path;
use std::{fs, path};
extern crate dirs;

fn main() {
    //let home = dirs::home_dir().unwrap();
    let dl = dirs::download_dir().unwrap();
    let pics = dirs::picture_dir().unwrap();
    let vids = dirs::video_dir().unwrap();
    let docs = dirs::document_dir().unwrap();
    let zips = dirs::document_dir().unwrap().join("Zips");
    let txts = dirs::document_dir().unwrap().join("TextFiles");
    let pdfs = dirs::document_dir().unwrap().join("Pdfs");

    let exts = vec![
        (
            vec![".jpeg", ".gif", ".jpg", ".png", ".svg", ".webp", ".ico"],
            pics,
        ),
        (vec![".7z", ".rar", "zip"], zips),
        (vec![".webm", ".mp4", ".mpg", ".mov"], vids),
        (vec![".docx", ".odb"], docs),
        (vec![".txt"],txts),
        (vec![".pdf"],pdfs),
    ];

    let files = fs::read_dir(dl).unwrap();

    for file in files {
        let file = file.unwrap();
        for tuples in &exts {
            for extentions in &tuples.0 {
                match Path::new(&file.path().to_str().unwrap()).file_name() {
                    Some(filename) => {
                        if filename.to_str().unwrap().ends_with(extentions) {
                            println!("{}", &filename.to_str().unwrap());

                            let _ = fs::rename(file.path().as_os_str(), tuples.1.join(filename));
                        }
                    }
                    None => {}
                }
            }
        }
    }
}
