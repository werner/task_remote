use gtk::*;

use chooser::{Chooser};

pub struct TaskChooser {
    pub chooser: Chooser
}

impl TaskChooser {
    pub fn new() -> TaskChooser {
        TaskChooser {
            chooser:
            Chooser {
                combo: ComboBox::new(),
                model_store: ListStore::new(&[Type::String, Type::String]),
            }
        }
    }
}
