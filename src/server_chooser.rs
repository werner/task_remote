use gtk::*;

use chooser::{Chooser};
use models::*;
use diesel::prelude::*;
use schema::servers::dsl::*;
use db_connection::*;
use models::{MutServer};

pub struct ServerChooser {
    pub chooser: Chooser,
    pub add_server_btn: Button
}

impl ServerChooser {

    pub fn new() -> ServerChooser {
        ServerChooser {
            chooser:
              Chooser {
                  combo: ComboBox::new(),
                  model_store: ListStore::new(&[Type::I32, Type::String]),
              },
            add_server_btn: Button::new_with_label("+")
        }
    }

    pub fn widget(&self, window: &ApplicationWindow) -> Box {
      let hbox = Box::new(Orientation::Horizontal, 2);

      self.add_server_btn.connect_clicked(clone!(window => move |_| {
        let dialog = Dialog::new_with_buttons(Some("Add a Server"), Some(&window), DialogFlags::empty(), &[("Save", 1), ("Cancel", 2)]);
        let content = dialog.get_content_area();

        let entry_user = Entry::new();
        let label_user = Label::new_with_mnemonic(Some("User:"));
        label_user.set_mnemonic_widget(Some(&entry_user));

        label_user.set_halign(Align::Start);
        content.pack_start(&label_user, false, false, 1);
        content.pack_start(&entry_user, false, false, 1);

        let entry_server_name = Entry::new();
        let label_server_name = Label::new_with_mnemonic(Some("Server Name:"));
        label_server_name.set_mnemonic_widget(Some(&entry_server_name));

        label_server_name.set_halign(Align::Start);
        content.pack_start(&label_server_name, false, false, 1);
        content.pack_start(&entry_server_name, false, false, 1);

        let response = {
          dialog.show_all();
          dialog.run()
        };

        if let 1 = response {
          let connection: SqliteConnection = establish_connection();
          let server = MutServer::new(entry_user.get_text().unwrap_or(String::new()), entry_server_name.get_text().unwrap_or(String::new()));
          server.create(&connection);
        }

        dialog.destroy();
      }));
      hbox.pack_start(&self.chooser.combo, true, true, 1);
      hbox.pack_start(&self.add_server_btn, false, false, 1);
      hbox
    }

    pub fn fill(&self) {
        let connection: SqliteConnection = establish_connection();

        self.chooser.add_db_row(&self.chooser.model_store, 0, "Choose a Server");
        let results = servers.load::<Server>(&connection).expect("Error loading servers");
        for server in results {
            self.chooser.add_db_row(&self.chooser.model_store,
                                    server.id,
                                    &format!("{}@{}", server.user, server.domain_name));
        }
        self.chooser.combo.set_active(0);
    }
}
