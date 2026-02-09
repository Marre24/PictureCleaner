use std::collections::LinkedList;

use crate::picture_list::PictureList;

#[derive(Default)]
pub(crate) struct PictureHandler {
    unchecked_pics: PictureList,
    saved_pics: PictureList,
    deleted_pics: PictureList,
    history: LinkedList<bool>,
}
impl PictureHandler {
    pub(crate) fn init(&mut self, pl: PictureList) {
        self.unchecked_pics = pl;
    }

    pub(crate) fn images_left(&self) -> String {
        self.unchecked_pics.size().to_string()
    }

    pub(crate) fn delete(&mut self) {
        self.deleted_pics.transfer_from(&mut self.unchecked_pics);
        self.history.push_front(false);
    }

    pub(crate) fn save(&mut self) {
        self.saved_pics.transfer_from(&mut self.unchecked_pics);
        self.history.push_front(true);
    }

    pub(crate) fn revert_last_action(&mut self) {
        if self.history.is_empty() {
            println!("Empty history, cannot revert");
            return;
        }
        if self.history.pop_front().unwrap() {
            self.unchecked_pics.transfer_from(&mut self.saved_pics);
        } else {
            self.unchecked_pics.transfer_from(&mut self.deleted_pics);
        }
    }

    pub(crate) fn get_next(&self) -> &std::path::Path {
        self.unchecked_pics.peek()
    }
}
