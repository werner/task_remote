use gtk::*;

use chooser::{Chooser};
use form::{Form};

#[derive(Clone)]
pub struct LanguageChooser {
    pub chooser: Chooser
}

impl LanguageChooser {
    pub fn new() -> LanguageChooser {
        LanguageChooser {
            chooser:
                Chooser {
                    combo: ComboBox::new(),
                    model_store: ListStore::new(&[Type::String, Type::String]),
                }
        }
    }

    pub fn fill(&self) {
        self.chooser.add_text_row(&self.chooser.model_store, "null", "Choose a Language");
        self.chooser.add_text_row(&self.chooser.model_store, "ruby", "Ruby");
        self.chooser.add_text_row(&self.chooser.model_store, "python", "Python");
        self.chooser.add_text_row(&self.chooser.model_store, "perl", "Perl");
        self.chooser.combo.set_active(0);
    }

    pub fn connect_change(&self, form: &Form) {
        self.chooser.combo.connect_changed(clone!(form => move |combo| {
            if let Some(id) = combo.get_active_id() {
                form.source_view.configure_sourceview(&id);
            }
        }));
    }

}