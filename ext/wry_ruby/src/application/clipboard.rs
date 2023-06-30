// https://docs.rs/wry/latest/wry/application/clipboard/struct.Clipboard.html

use std::cell::RefCell;

use gdk;
use gtk;

use magnus::{function, method, Error, Module, Object};

use wry::application::clipboard::Clipboard as ClipboardImpl;

#[derive(Clone, Debug)]
#[magnus::wrap(class = "Clip")]
pub struct Clipboard {
    inner: RefCell<ClipboardImpl>,
}

impl Clipboard {
    pub fn new() -> Result<Self, Error> {
        let inner = ClipboardImpl::new();

        Ok(Self {
            inner: RefCell::new(inner),
        })
    }

    pub fn get(&self) -> std::cell::Ref<ClipboardImpl> {
        self.inner.borrow()
    }

    pub fn get_mut(&self) -> std::cell::RefMut<ClipboardImpl> {
        self.inner.borrow_mut()
    }

    /// Writes text to the clipboard.
    ///
    /// # Arguments
    ///
    /// * `s` - A string containing the text to be written.
    ///
    /// # Equivalent Ruby Method
    ///
    /// `clip.write_text("Hello World")`
    pub fn write_text(&self, s: String) {
        let text: &str = s.as_ref();
        self.get_mut().write_text(text);
    }

    /// Reads text from the clipboard.
    ///
    /// # Equivalent Ruby Method
    ///
    /// `clip.read_text`
    pub fn read_text(&self) -> Option<String> {
        self.get().read_text()
    }
}

/// Initializes the clipboard module.
///
/// # Equivalent Ruby Methods
///
/// - `clip = Clip.new`
/// - `clip.write_text("Hello World")`
/// - `clip.read_text`
pub fn init() -> Result<(), Error> {
    gdk::init();
    let _ = gtk::init();

    let class = magnus::define_class("Clip", Default::default())?;

    class.define_singleton_method("new", function!(Clipboard::new, 0))?;
    class.define_method("write_text", method!(Clipboard::write_text, 1))?;
    class.define_method("read_text", method!(Clipboard::read_text, 0))?;

    Ok(())
}
