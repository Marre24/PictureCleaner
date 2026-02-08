use crate::picture_list::PictureList;
use std::{
    fs::{self, ReadDir},
    io::Error,
    path::PathBuf,
};

const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "gif", "bmp", "webp", "svg"];

pub(crate) fn get_picture_list_for(dir: &str) -> PictureList {
    let mut pl: PictureList = Default::default();

    add_paths_to(&mut pl, fs::read_dir(dir));

    pl
}

fn add_paths_to(pl: &mut PictureList, maybe_entries: Result<ReadDir, Error>) {
    if let Err(error) = maybe_entries {
        print!("Could not find entries: {}", error);
        return;
    }
    let entries = maybe_entries.unwrap();

    for maybe_entry in entries {
        if let Err(_e) = maybe_entry {
            continue;
        }
        let entry = maybe_entry.unwrap();
        let path: PathBuf = entry.path();

        if path.is_dir() {
            add_paths_to(pl, path.read_dir());
            continue;
        }

        if IMAGE_EXTENSIONS.contains(&extension_str_for(&path).as_str()) {
            println!("Found: {}", path.display());
            pl.add(path);
        }
    }
}

fn extension_str_for(path: &PathBuf) -> String {
    if let Some(ext) = path.extension() {
        if let Some(ext_str) = ext.to_str() {
            return ext_str.to_lowercase();
        }
    }
    String::new()
}

#[cfg(test)]
mod picture_list_test {
    use crate::file_manager::get_picture_list_for;

    #[test]
    fn integration_test() {
        get_picture_list_for("/home/rackarn/Pictures/casio_camera");
    }
}
