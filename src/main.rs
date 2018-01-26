extern crate gio;
extern crate gtk;
extern crate sourceview;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate ssh2;
extern crate uuid;

#[macro_use]
mod utils;

mod language_chooser;
mod task_chooser;
mod server_chooser;
mod source_view;
mod chooser;
mod models;
mod schema;
mod db_connection;
mod form;
mod ssh;

use task_chooser::{TaskChooser};
use gio::prelude::*;
use gtk::*;
use diesel::prelude::*;
use db_connection::*;
use form::*;
use server_chooser::{ServerChooser};
use ssh::{Ssh};
use models::{MutServer};

use std::env::args;

fn build_ui(application: &Application) {
    let window = ApplicationWindow::new(application);

    window.set_title("Task Remote");
    window.set_border_width(10);
    window.set_position(WindowPosition::Center);
    window.set_default_size(800, 600);

    window.connect_delete_event(clone!(window => move |_, _| {
        window.destroy();
        Inhibit(false)
    }));

    let hbox = Box::new(Orientation::Horizontal, 2);
    let vbox_scripts = Box::new(Orientation::Vertical, 6);

    let task_chooser: TaskChooser = TaskChooser::new();

    task_chooser.chooser.prepare();
    task_chooser.fill();
    vbox_scripts.pack_start(&task_chooser.chooser.combo, false, false, 5);

    let form = Form::new();

    form.title.set_placeholder_text("Title");
    vbox_scripts.pack_start(&form.title, false, false, 5);

    form.command.set_placeholder_text("Command");
    vbox_scripts.pack_start(&form.command, false, false, 5);

    vbox_scripts.pack_start(&form.get_view_from_sourceview(), true, true, 5);

    let label = Label::new_with_mnemonic(Some("Output"));
    label.set_mnemonic_widget(Some(&form.output));
    vbox_scripts.pack_start(&label, false, false, 1);
    vbox_scripts.pack_start(&form.output, false, false, 1);

    hbox.pack_start(&vbox_scripts, true, true, 1);

    let vbox_options = Box::new(Orientation::Vertical, 4);

    form.language_chooser.chooser.prepare();
    form.language_chooser.fill();

    vbox_options.pack_start(&form.language_chooser.chooser.combo, false, false, 5);

    form.language_chooser.connect_change(&form);

    let save_button: Button = Button::new_with_label("Save");
    let task_choose2 = task_chooser.clone();
    save_button.connect_clicked(clone!(form => move |_| {
        let task = form.load();
        let connection: SqliteConnection = establish_connection();
        task.save(&connection, task_choose2.chooser.combo.get_active_id().unwrap().parse::<i32>().unwrap());
    }));

    vbox_options.pack_start(&save_button, false, false, 5);

    let server_pack: ServerChooser = ServerChooser::new();
    server_pack.chooser.prepare();
    server_pack.fill();
    vbox_options.pack_start(&server_pack.widget(&window), false, false, 5);

    let run_button: Button = Button::new_with_label("Execute");
    run_button.connect_clicked(clone!(form => move |_| {
        let connection: SqliteConnection = establish_connection();
        if let Ok(mut_server) = MutServer::find(&connection, server_pack.chooser.combo.get_active_id().unwrap().parse::<i32>().unwrap()) {
            let mut ssh = Ssh::new(&mut_server.user, &mut_server.domain_name);
            match ssh.connect() {
                Ok(sess) => {
                    let file_name = ssh.upload_code(&sess, &form.get_code());
                    let command = &form.command.get_text().unwrap();
                    let to_execute = command.replace("$CODE", &format!("/tmp/{}", file_name));
                    let output = ssh.execute(&sess, &to_execute);
                    ssh.execute(&sess, &format!("rm /tmp/{}", file_name));
                    form.set_output(&output);
                },
                Err(error) => println!("{}", error)
            }
        } else {
            println!("Server Not Found");
        }
    }));
    vbox_options.pack_start(&run_button, false, false, 5);

    hbox.pack_start(&vbox_options, true, true, 1);

    window.add(&hbox);

    task_chooser.connect_change(form);
    window.show_all();
}

fn main() {
    let application = Application::new("com.task_remote",
                                       gio::ApplicationFlags::empty())
                                       .expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
