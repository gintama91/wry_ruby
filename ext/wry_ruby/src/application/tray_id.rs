use magnus::{function, Error, ExceptionClass, Module, Object, RString};
use wry::application::TrayId as TrayIdImpl;

#[derive(Clone, Debug)]
#[magnus::wrap(class = "TrayId")]
pub struct TrayId {
    inner: TrayIdImpl,
}

impl TrayId {
    pub const EMPTY: TrayId = TrayId {
        inner: TrayIdImpl::EMPTY,
    };

    pub fn new(unique_string: RString) -> Result<Self, Error> {
        if unique_string.is_empty() {
            return Err(Error::new(
                ExceptionClass::default(),
                "Unique string cannot be empty",
            ));
        }

        let s = match unique_string.as_interned_str() {
            Some(str_ref) => str_ref.as_str().unwrap(),
            None => {
                println!("ok why is it none, hm check freeze or no");
                return Err(Error::new(
                    ExceptionClass::default(),
                    "Interned string not available",
                ));
            }
        };

        Ok(Self {
            inner: TrayIdImpl::new(s),
        })
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

pub fn init() -> Result<(), Error> {
    let class = magnus::define_class("TrayId", Default::default())?;

    class.define_singleton_method("new", function!(TrayId::new, 1))?;
    class.define_method("is_empty", magnus::method!(TrayId::is_empty, 0))?;

    Ok(())
}
