use gtk::*;

use source_view::{SourceView};
use sourceview::{View};
use language_chooser::{LanguageChooser};
use models::{Task, MutTask};

enum ViewText<'a> {
  ViewSource(&'a View),
  GtkViewText(&'a TextView)
}

#[derive(Clone)]
pub struct Form {
    pub title: Entry,
    pub command: Entry,
    pub output: TextView,
    pub source_view: SourceView,
    pub language_chooser: LanguageChooser
}

impl Form {
  pub fn new() -> Form {
    Form {
      title: Entry::new(),
      command: Entry::new(),
      output: TextView::new(),
      source_view: SourceView::new(),
      language_chooser: LanguageChooser::new()
    }
  }

  pub fn get_code(&self) -> String {
    self.get_text_from_view(ViewText::GtkViewText(&self.output))
  }

  pub fn set_output(&self, text: &str) {
    self.output.get_buffer().unwrap().set_text(text);
  }

  pub fn load(&self, view: &View) -> MutTask {
    MutTask::new(self.title.get_text().unwrap_or(String::new()),
                 self.command.get_text(),
                 self.get_text_from_view(ViewText::ViewSource(view)),
                 Some(self.get_code()),
                 self.language_chooser.chooser.combo.get_active_id())
  }

  pub fn set_values(&self, task: Task) {
    self.title.set_text(&task.title);
    self.command.set_text(&task.command.unwrap_or(String::new()));
    self.source_view.buffer.set_text(&task.code);
    self.set_output(&task.output.unwrap_or(String::new()));
    self.language_chooser.chooser.combo.set_active_id(Some(task.language.unwrap().as_str()));
  }

  // waiting for https://github.com/rust-lang/rfcs/pull/2175
  fn get_text_from_view(&self, view_text: ViewText) -> String {
    if let ViewText::ViewSource(view) = view_text {
      view.get_buffer().unwrap().get_text(&view.get_buffer().unwrap().get_start_iter(),
                                          &view.get_buffer().unwrap().get_end_iter(),
                                          true).unwrap_or(String::new())
    } else if let ViewText::GtkViewText(view) = view_text {
      view.get_buffer().unwrap().get_text(&view.get_buffer().unwrap().get_start_iter(),
                                          &view.get_buffer().unwrap().get_end_iter(),
                                          true).unwrap_or(String::new())
    } else {
      panic!("Wut")
    }
  }
}