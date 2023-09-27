use magnus::{
    function, Error, Object,
};

use std::marker::PhantomData;
use std::sync::atomic::AtomicPtr;

use wry::application::event_loop::EventLoop as EventLoopImpl;

#[magnus::wrap(class = "EventLoop")]
pub struct EventLoop {
    inner: SafeWrapper<EventLoopImpl<()>>,
}

pub struct SafeWrapper<T> {
    event_loop: AtomicPtr<T>,
    _marker: PhantomData<*mut T>,
}

unsafe impl<T> Send for SafeWrapper<T> {}

impl EventLoop {
    pub fn new() -> Self {
        let event_loop_impl = EventLoopImpl::new();
        let inner = SafeWrapper {
            event_loop: AtomicPtr::new(Box::into_raw(Box::new(event_loop_impl))),
            _marker: PhantomData,
        };
        EventLoop { inner }
    }
}

pub fn init() -> Result<(), Error> {
    let class = magnus::define_class("WindowBuilder", Default::default())?;
    class.define_singleton_method("new", function!(EventLoop::new, 0))?;
    Ok(())
}
