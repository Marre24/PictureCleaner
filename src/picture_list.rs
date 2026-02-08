#[derive(Default)]
struct PictureList {
    value: Vec<String>,
}

impl PictureList {
    fn size(&self) -> usize {
        self.value.len()
    }
}

#[cfg(test)]
mod picture_list_test {
    use crate::picture_list::PictureList;

    #[test]
    fn default_initializes_to_empty_list() {
        let pl = PictureList::default();
        assert_eq!(0, pl.size());
    }
}
