use gtk::*;

use chooser::{Chooser};
use models::*;
use diesel::prelude::*;
use schema::tasks::dsl::*;
use db_connection::*;
use models::{MutTask};
use form::{Form};

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
            self.chooser.add_text_row(&self.chooser.model_store,
                                      &task.id.to_string(),
                                      &task.title);
        }
        self.chooser.combo.set_active(0);
    }

    pub fn connect_change(&self, form: Form) {
        self.chooser.combo.connect_changed(move |combo| {
            if let Some(string_id) = combo.get_active_id() {
                if let Ok(id_db) = string_id.parse::<i32>() {
                    let connection: SqliteConnection = establish_connection();
                    let task = MutTask::find(&connection, id_db);
                    form.set_values(task);
                }
            }
        });
    }
}
