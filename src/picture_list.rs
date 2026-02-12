use std::{
    collections::{linked_list::Iter, LinkedList},
    path::PathBuf,
};

#[derive(Default, Clone)]
pub(crate) struct PictureList {
    value: LinkedList<PathBuf>,
}

impl PictureList {
    pub(crate) fn new(ll: LinkedList<PathBuf>) -> PictureList {
        PictureList { value: ll }
    }

    pub(crate) fn size(&self) -> usize {
        self.value.len()
    }

    pub(crate) fn add(&mut self, path: PathBuf) {
        self.value.push_front(path);
    }

    pub(crate) fn next(&mut self) -> PathBuf {
        self.value
            .pop_front()
            .expect("attempted to get item from empty PictureList")
    }

    pub(crate) fn peek(&self) -> &PathBuf {
        self.value
            .front()
            .expect("attempted to peek from empty PictureList")
    }

    pub(crate) fn transfer_from(&mut self, other: &mut PictureList) {
        self.add(other.next());
    }

    pub(crate) fn path_iterator(&self) -> Iter<PathBuf> {
        self.value.iter()
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
        let path: PathBuf = PathBuf::from("GREEN/FN");
        let mut pl = PictureList::default();
        let size = pl.size();

        pl.add(path);

        assert_eq!(size + 1, pl.size());
    }

    #[test]
    fn gets_first_item() {
        let path: PathBuf = PathBuf::from("GREEN/FN");
        let mut pl = PictureList::default();
        pl.add(path.clone());

        let next: PathBuf = pl.next();

        assert_eq!(path, next);
    }

    #[test]
    fn next_decreases_size() {
        let path: PathBuf = PathBuf::from("GREEN/FN");
        let mut pl = PictureList::default();
        pl.add(path.clone());
        let size = pl.size();

        pl.next();

        assert_eq!(size - 1, pl.size());
    }

    #[test]
    fn size_moves_when_moving_item() {
        let path: PathBuf = PathBuf::from("GREEN/FN");
        let mut pl1 = PictureList::default();
        let mut pl2 = PictureList::default();
        pl1.add(path.clone());
        pl1.add(path.clone());
        pl2.add(path.clone());
        pl2.add(path.clone());
        let size1 = pl1.size();
        let size2 = pl2.size();

        pl1.transfer_from(&mut pl2);

        assert_eq!(size1 + 1, pl1.size());
        assert_eq!(size2 - 1, pl2.size());
    }
}
