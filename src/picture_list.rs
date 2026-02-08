use std::path::PathBuf;

#[derive(Default)]
struct PictureList {
    value: Vec<PathBuf>,
}

impl PictureList {
    fn size(&self) -> usize {
        self.value.len()
    }

    fn add(&mut self, path: PathBuf) {
        self.value.insert(0, path);
    }
}

#[cfg(test)]
mod picture_list_test {
    use std::path::PathBuf;

    use crate::picture_list::PictureList;

    #[test]
    fn default_initializes_to_empty_list() {
        let pl = PictureList::default();
        assert_eq!(0, pl.size());
    }

    #[test]
    fn size_increments_when_adding_item() {
        let PATH: PathBuf = PathBuf::from("GREEN/FN");
        let mut pl = PictureList::default();
        let size = pl.size();

        pl.add(PATH);

        assert_eq!(size + 1, pl.size());
    }
}
