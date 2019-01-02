extern crate image;
extern crate img_hash;

use std::path::PathBuf;
use img_hash::{ImageHash, HashType};

pub fn hash_image(image: PathBuf, algorithim: HashType) -> img_hash::ImageHash {
    let image = image::open(image).unwrap();
    let hash1 = ImageHash::hash(&image, 16, algorithim);
    return hash1;
}

pub fn diff(hash1: ImageHash, hash2: ImageHash) -> f32 {
    return hash1.dist_ratio(&hash2);
}
