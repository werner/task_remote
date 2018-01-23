use sourceview::{Buffer, LanguageManager, LanguageManagerExt, StyleSchemeManager, StyleSchemeManagerExt, BufferExt};

#[derive(Clone)]
pub struct SourceView {
  pub buffer: Buffer
}

impl SourceView {
  pub fn new() -> SourceView {
    SourceView { 
      buffer: Buffer::new(None)
    }
  }

  pub fn configure_sourceview(&self, language: &str) {
      LanguageManager::new()
          .get_language(language)
          .map(|markdown| self.buffer.set_language(&markdown));

      let manager = StyleSchemeManager::new();
      manager
          .get_scheme("classic")
          .map(|theme| self.buffer.set_style_scheme(&theme));
  }

}