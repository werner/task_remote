use gtk::*;

#[derive(Clone)]
pub struct Chooser {
    pub combo: ComboBox,
    pub model_store: ListStore
}

impl Chooser {
    pub fn prepare(&self) {
        self.combo.set_model(Some(&self.model_store));
        self.combo.set_id_column(0);
        self.combo.set_entry_text_column(1);

        let cell = CellRendererText::new();
        self.combo.pack_start(&cell, true);
        self.combo.add_attribute(&cell, "text", 1);
    }

    pub fn add_text_row(&self, store: &ListStore, col1: &str, col2: &str) -> TreeIter {
        store.insert_with_values(None, &[0, 1], &[&String::from(col1), &String::from(col2)])
    }

    pub fn add_db_row(&self, store: &ListStore, id: i32, text: &str) -> TreeIter {
        store.insert_with_values(None, &[0, 1], &[&id, &String::from(text)])
    }
}
