extern crate gio;
extern crate gtk;
extern crate sourceview;
#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
mod utils;

mod language_chooser;
mod task_chooser;
mod source_view;
mod chooser;
mod models;
mod schema;
mod db_connection;

use language_chooser::{LanguageChooser};
use task_chooser::{TaskChooser};
use source_view::{SourceView};
use gio::prelude::*;
use gtk::*;
use sourceview::{View, ViewExt};
use models::*;
use diesel::prelude::*;
use db_connection::*;

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
    let vbox_scripts = Box::new(Orientation::Vertical, 5);

    let task_chooser: TaskChooser = TaskChooser::new();

    task_chooser.chooser.prepare();
    task_chooser.fill();
    vbox_scripts.pack_start(&task_chooser.chooser.combo, false, false, 5);

    let title = Entry::new();
    title.set_placeholder_text("Title");
    vbox_scripts.pack_start(&title, false, false, 5);

    let pre_hook = Entry::new();
    pre_hook.set_placeholder_text("Pre hook");
    vbox_scripts.pack_start(&pre_hook, false, false, 5);

    let source_view: SourceView = SourceView::new();

    let view = View::new_with_buffer(&source_view.buffer);
    view.has_focus();
    view.grab_focus();
    view.set_show_line_numbers(true);
    view.set_auto_indent(true);
    vbox_scripts.pack_start(&view, true, true, 5);

    let post_hook = Entry::new();
    post_hook.set_placeholder_text("Post hook");
    vbox_scripts.pack_start(&post_hook, false, false, 5);

    hbox.pack_start(&vbox_scripts, true, true, 1);

    let vbox_options = Box::new(Orientation::Vertical, 4);

    let language_chooser: LanguageChooser = LanguageChooser::new();

    language_chooser.chooser.prepare();
    language_chooser.fill();
    language_chooser.connect_change(source_view);

    vbox_options.pack_start(&language_chooser.chooser.combo, false, false, 5);

    let save_button: Button = Button::new_with_label("Save");
    save_button.connect_clicked(move |_| {
        let connection: SqliteConnection = establish_connection();
        let task = MutTask::new(title.get_text().unwrap_or(String::new()),
                                pre_hook.get_text(),
                                view.get_buffer().unwrap().get_text(&view.get_buffer().unwrap().get_start_iter(),
                                                                    &view.get_buffer().unwrap().get_end_iter(),
                                                                    true).unwrap_or(String::new()),
                                post_hook.get_text(),
                                language_chooser.chooser.combo.get_active_id());
        task.create(&connection);
    });
    vbox_options.pack_start(&save_button, false, false, 5);

    hbox.pack_start(&vbox_options, true, true, 1);

    window.add(&hbox);

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
