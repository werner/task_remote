extern crate gio;
extern crate gtk;
extern crate sourceview;

use gio::prelude::*;
use gtk::prelude::*;
use sourceview::{Buffer, LanguageManager, LanguageManagerExt, StyleSchemeManager, StyleSchemeManagerExt, BufferExt, View};

use std::env::args;

// make moving clones into closures more convenient
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

pub fn configure_sourceview(buff: &Buffer) {
    LanguageManager::new()
        .get_language("ruby")
        .map(|markdown| buff.set_language(&markdown));

    let manager = StyleSchemeManager::new();
    manager
        .get_scheme("classic")
        .map(|theme| buff.set_style_scheme(&theme));
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

    let hbox = gtk::Box::new(gtk::Orientation::Vertical, 4);

    let pre_hook = gtk::Entry::new();
    hbox.pack_start(&pre_hook, false, false, 5);

    let buffer = sourceview::Buffer::new(None);
    configure_sourceview(&buffer);

    let view = View::new_with_buffer(&buffer);
    hbox.pack_start(&view, true, true, 5);

    let post_hook = gtk::Entry::new();
    hbox.pack_start(&post_hook, false, false, 5);

    let button = gtk::Button::new_with_label("Choose language");
    hbox.pack_start(&button, false, false, 5);

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
