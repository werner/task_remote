extern crate gio;
extern crate gtk;
extern crate sourceview;

#[macro_use]
mod utils;

mod language_chooser;
mod source_view;

use language_chooser::{LanguageChooser};
use source_view::{SourceView};
use gio::prelude::*;
use gtk::prelude::*;
use sourceview::{View, ViewExt};

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Task Remote");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(800, 600);

    window.connect_delete_event(clone!(window => move |_, _| {
        window.destroy();
        Inhibit(false)
    }));

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    let vbox_scripts = gtk::Box::new(gtk::Orientation::Vertical, 4);

    let title = gtk::Entry::new();
    title.set_placeholder_text("Title");
    vbox_scripts.pack_start(&title, false, false, 5);

    let pre_hook = gtk::Entry::new();
    pre_hook.set_placeholder_text("Pre hook");
    vbox_scripts.pack_start(&pre_hook, false, false, 5);

    let source_view = SourceView::new();

    let view = View::new_with_buffer(&source_view.buffer);
    view.has_focus();
    view.grab_focus();
    view.set_show_line_numbers(true);
    view.set_auto_indent(true);
    vbox_scripts.pack_start(&view, true, true, 5);

    let post_hook = gtk::Entry::new();
    post_hook.set_placeholder_text("Post hook");
    vbox_scripts.pack_start(&post_hook, false, false, 5);

    hbox.pack_start(&vbox_scripts, true, true, 1);

    let vbox_options = gtk::Box::new(gtk::Orientation::Vertical, 3);

    let language_chooser = LanguageChooser::new();

    language_chooser.prepare();
    language_chooser.fill();

    language_chooser.connect_change(source_view);

    vbox_options.pack_start(&language_chooser.combo, false, false, 5);

    hbox.pack_start(&vbox_options, true, true, 1);

    window.add(&hbox);

    window.show_all();
}

fn main() {
    let application = gtk::Application::new("com.task_remote",
                                            gio::ApplicationFlags::empty())
                                       .expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
