use gtk::*;
use source_view::*;

pub struct LanguageChooser {
  pub combo: ComboBox,
  model_store: ListStore
}

impl LanguageChooser {
  pub fn new() -> LanguageChooser {
    LanguageChooser { 
      combo: ComboBox::new(), 
      model_store: ListStore::new(&[Type::String, Type::String])
    }
  }

  pub fn prepare(&self) {
    self.combo.set_model(Some(&self.model_store));
    self.combo.set_id_column(0);
    self.combo.set_entry_text_column(1);

    let cell = CellRendererText::new();
    self.combo.pack_start(&cell, true);
    self.combo.add_attribute(&cell, "text", 1);
  }

  pub fn fill(&self) {
    self.add_text_row(&self.model_store, "ruby", "Ruby");
    self.add_text_row(&self.model_store, "python", "Python");
    self.add_text_row(&self.model_store, "perl", "Perl");
  }

  pub fn connect_change(&self, source_view: SourceView) {
    self.combo.connect_changed(move |combo| {
        if let Some(id) = combo.get_active_id() {
            source_view.configure_sourceview(&id);
        }
    });
  }

  fn add_text_row(&self, store: &ListStore, col1: &str, col2: &str) -> TreeIter {
    store.insert_with_values(None, &[0, 1], &[&String::from(col1), &String::from(col2)])
  }

}