use gtk::*;

use source_view::{SourceView};
use sourceview::{View};
use language_chooser::{LanguageChooser};
use models::{MutTask};

pub struct Form {
    pub title: Entry,
    pub pre_hook: Entry,
    pub post_hook: Entry,
    pub source_view: SourceView,
    pub language_chooser: LanguageChooser
}

impl Form {
  pub fn new() -> Form {
    Form {
      title: Entry::new(),
      pre_hook: Entry::new(),
      post_hook: Entry::new(),
      source_view: SourceView::new(),
      language_chooser: LanguageChooser::new()
    }
  }

  pub fn load(&self, view: View) -> MutTask {
    MutTask::new(self.title.get_text().unwrap_or(String::new()),
                 self.pre_hook.get_text(),
                 view.get_buffer().unwrap().get_text(&view.get_buffer().unwrap().get_start_iter(),
                                                     &view.get_buffer().unwrap().get_end_iter(),
                                                     true).unwrap_or(String::new()),
                 self.post_hook.get_text(),
                 self.language_chooser.chooser.combo.get_active_id())
  }
}