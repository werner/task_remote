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

    pub fn fill(&self) {
        self.chooser.add_text_row(&self.chooser.model_store, "null", "Choose a Task");
        self.chooser.combo.set_active(0);
    }
}
