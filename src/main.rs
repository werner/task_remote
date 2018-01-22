extern crate gio;
extern crate gtk;
extern crate sourceview;

#[macro_use]
mod utils;

use gio::prelude::*;
use gtk::prelude::*;
use sourceview::{Buffer, LanguageManager, LanguageManagerExt, StyleSchemeManager, StyleSchemeManagerExt, BufferExt, View};

use std::env::args;

pub fn configure_sourceview(buff: &Buffer, language: &str) {
    LanguageManager::new()
        .get_language(language)
        .map(|markdown| buff.set_language(&markdown));

    let manager = StyleSchemeManager::new();
    manager
        .get_scheme("classic")
        .map(|theme| buff.set_style_scheme(&theme));
}

pub fn add_text_row(store: &gtk::ListStore,
                    col1: &str, col2: &str) -> gtk::TreeIter {
    store.insert_with_values(None, &[0, 1],
                             &[&String::from(col1), &String::from(col2)])
}

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
    let vbox_scripts = gtk::Box::new(gtk::Orientation::Vertical, 3);

    let pre_hook = gtk::Entry::new();
    vbox_scripts.pack_start(&pre_hook, false, false, 5);

    let buffer = sourceview::Buffer::new(None);

    let view = View::new_with_buffer(&buffer);
    vbox_scripts.pack_start(&view, true, true, 5);

    let post_hook = gtk::Entry::new();
    vbox_scripts.pack_start(&post_hook, false, false, 5);

    hbox.pack_start(&vbox_scripts, true, true, 1);

    let vbox_options = gtk::Box::new(gtk::Orientation::Vertical, 3);

    let language_chooser = gtk::ComboBox::new();
    let model_store = gtk::ListStore::new(&[gtk::Type::String, gtk::Type::String]);

    language_chooser.set_model(Some(&model_store));
    language_chooser.set_id_column(0);
    language_chooser.set_entry_text_column(1);

    add_text_row(&model_store, "ruby", "Ruby");
    add_text_row(&model_store, "python", "Python");
    add_text_row(&model_store, "perl", "Perl");

    let cell = gtk::CellRendererText::new();
    language_chooser.pack_start(&cell, true);
    language_chooser.add_attribute(&cell, "text", 1);

    language_chooser.connect_changed(move |combo| {
        if let Some(id) = combo.get_active_id() {
            configure_sourceview(&buffer, &id);
        }
    });

    vbox_options.pack_start(&language_chooser, false, false, 5);

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
