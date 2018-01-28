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
mod task_package;
mod server_package;
mod source_view;
mod chooser;
mod models;
mod schema;
mod db_connection;
mod form;
mod ssh;

use task_package::{TaskPackage};
use gio::prelude::*;
use gtk::*;
use form::*;
use server_package::{ServerPackage};

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

    let task_package: TaskPackage = TaskPackage::new();

    task_package.chooser.prepare();
    task_package.fill();
    vbox_scripts.pack_start(&task_package.chooser.combo, false, false, 5);

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

    let vbox_options = Box::new(Orientation::Vertical, 5);

    form.language_chooser.chooser.prepare();
    form.language_chooser.fill();

    vbox_options.pack_start(&form.language_chooser.chooser.combo, false, false, 5);

    form.language_chooser.connect_change(&form);

    let server_pack: ServerPackage = ServerPackage::new();
    server_pack.chooser.prepare();
    server_pack.fill();
    vbox_options.pack_start(&server_pack.widget(&window), false, false, 5);

    task_package.prepare_buttons(&window, &form, &server_pack);
    vbox_options.pack_start(&task_package.save_btn, false, false, 5);
    vbox_options.pack_start(&task_package.delete_btn, false, false, 5);
    vbox_options.pack_start(&task_package.run_btn, false, false, 5);

    hbox.pack_start(&vbox_options, true, true, 1);

    window.add(&hbox);

    task_package.connect_change(form);
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
