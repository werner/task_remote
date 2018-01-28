use gtk::*;

use chooser::{Chooser};
use models::*;
use diesel::prelude::*;
use schema::tasks::dsl::*;
use db_connection::*;
use models::{MutTask};
use form::{Form};
use server_package::{ServerPackage};
use ssh::{Ssh};

#[derive(Clone)]
pub struct TaskPackage {
    pub chooser: Chooser,
    pub save_btn: Button,
    pub delete_btn: Button,
    pub run_btn: Button
}

impl TaskPackage {

    pub fn new() -> TaskPackage {
        TaskPackage {
            chooser:
                Chooser {
                    combo: ComboBox::new(),
                    model_store: ListStore::new(&[Type::String, Type::String]),
                },
            save_btn: Button::new_with_label("Save"),
            delete_btn: Button::new_with_label("Delete"),
            run_btn: Button::new_with_label("Execute")
        }
    }

    pub fn prepare_buttons(&self, window: &ApplicationWindow, form: &Form, server_pack: &ServerPackage) {
        let this_for_save = self.clone();
        self.save_btn.connect_clicked(clone!(form => move |_| {
            let task = form.load();
            let connection: SqliteConnection = establish_connection();
            task.save(&connection, this_for_save.chooser.combo.get_active_id().unwrap().parse::<i32>().unwrap());
            this_for_save.fill();
        }));

        let this_for_delete = self.clone();
        self.delete_btn.connect_clicked(clone!(window => move |_| {
            let dialog = Dialog::new_with_buttons(Some("Add a Server"), Some(&window), DialogFlags::empty(), &[("Yes", 1), ("No", 2)]);
            let content = dialog.get_content_area();

            let label = Label::new_with_mnemonic(Some("Are you sure?"));

            content.pack_start(&label, false, false, 1);

            let response = {
                dialog.show_all();
                dialog.run()
            };

            if let 1 = response {
                let connection: SqliteConnection = establish_connection();
                MutTask::destroy(&connection, this_for_delete.chooser.combo.get_active_id().unwrap().parse::<i32>().unwrap());
                this_for_delete.fill();
            };
            dialog.destroy();
        }));

        let server_pack_cloned = server_pack.clone();
        self.run_btn.connect_clicked(clone!(form => move |_| {
            let server_pack_to_run = server_pack_cloned.clone();
            idle_add(clone!(form => move || {
                let connection: SqliteConnection = establish_connection();
                if let Ok(mut_server) = MutServer::find(&connection, server_pack_to_run.chooser.combo.get_active_id().unwrap().parse::<i32>().unwrap()) {
                    let mut ssh = Ssh::new(&mut_server.user, &mut_server.domain_name);
                    match ssh.connect() {
                        Ok(sess) => {
                            let file_name = ssh.upload_code(&sess, &form.get_code());
                            let command_to_run = &form.command.get_text().unwrap();
                            let code_to_execute = command_to_run.replace("$CODE", &format!("/tmp/{}", file_name));
                            let string_output = ssh.execute(&sess, &code_to_execute);
                            ssh.execute(&sess, &format!("rm /tmp/{}", file_name));
                            form.set_output(&string_output);
                        },
                        Err(error) => println!("{}", error)
                    }
                } else {
                    println!("Server Not Found");
                }

                Continue(false)
            }));
        }));

    }

    pub fn fill(&self) {
        let connection: SqliteConnection = establish_connection();

        self.chooser.model_store.clear();
        self.chooser.add_text_row(&self.chooser.model_store, "-1", "Choose a Task");
        self.chooser.add_text_row(&self.chooser.model_store, "0", "New Task");
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
                    if let Ok(task) = MutTask::find(&connection, id_db) {
                        form.set_values(task);
                    } else {
                        form.clear();
                        println!("Not Found");
                    }
                }
            }
        });
    }
}
