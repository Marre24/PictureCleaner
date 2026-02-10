use crate::picture_list::PictureList;
use std::{
    fs::{self, ReadDir},
    io::Error,
    path::PathBuf,
};

const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "gif", "bmp", "webp", "svg"];
const SAVE_DIR: &str = "/save/";
const DELETE_DIR: &str = "/delete/";

pub(crate) fn commit_state(src_folder: &str, save: &PictureList, delete: &PictureList) {
    move_each_picture_to(save, format!("{}{}", src_folder, SAVE_DIR));
    move_each_picture_to(delete, format!("{}{}", src_folder, DELETE_DIR));
}

fn move_each_picture_to(save: &PictureList, save_dir: String) {
    let result = fs::create_dir_all(&save_dir);
    if let Err(e) = result {
        println!("Could not move file: {} error: ", e)
    }

    for image in save.path_iterator() {
        let maybe_file_name = image.file_name();
        if maybe_file_name.is_none() {
            continue;
        }
        let file_name = maybe_file_name.unwrap().to_str().unwrap();

        let to = PathBuf::from(format!("{}{}", save_dir, file_name));
        let result = fs::rename(image, &to);
        if let Err(e) = result {
            println!(
                "Could not move file: {} to: {}, ERROR: {}",
                image.clone().into_os_string().into_string().unwrap(),
                to.clone().into_os_string().into_string().unwrap(),
                e
            )
        }
    }
}

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
