use gtk::*;

use chooser::{Chooser};
use models::*;
use diesel::prelude::*;
use schema::tasks::dsl::*;
use db_connection::*;

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
        let connection: SqliteConnection = establish_connection();

        self.chooser.add_text_row(&self.chooser.model_store, "null", "Choose a Task");
        let results = tasks.load::<Task>(&connection).expect("Error loading tasks");
        for task in results {
            self.chooser.add_text_row(&self.chooser.model_store, &task.id.to_string(), &task.title);
        }
        self.chooser.combo.set_active(0);
    }
}
