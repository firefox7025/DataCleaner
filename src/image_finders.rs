extern crate glob;

use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;



pub fn find_images(_input_dir: &Path) -> HashSet<PathBuf> {
    let extensions = ["jpg", "JPG", "png", "PNG"];
    let mut paths = HashSet::new();
    for ext in extensions.iter() {
    for entry in glob::glob(&(_input_dir.to_str().unwrap().to_owned()  + "**/*." + ext)).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => paths.insert(path),
            _ => false,
        };
    }
 }
    info!("found {} images to sort", paths.len());
    return paths;
}



