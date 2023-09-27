// hmm i am tired of this errors where traits are not implemented for tht , this hmmmm i will look into this later.
// we need this mostly to eval js code


// **************************************************************************************************************************************************************************************

// use std::sync::{Arc, Mutex};
// use webkit2gtk::WebView;

// use magnus::{function, Error, ExceptionClass, Module, Object, RString, define_global_function, class};
// use wry::application::window::Window as WindowImpl;
// use wry::webview::WebView as WebViewImpl;

// pub struct WebViewWrapper {
//     inner: Arc<Mutex<WebViewImpl>>,
// }

// impl WebViewWrapper {
//     pub fn window(&self) -> Result<(), Error> {
//         let inner = self.inner.lock().unwrap();
//         WebViewImpl::window(&inner);
//         Ok(())
//     }

//     pub fn eval(&self, js: RString) -> Result<(), Error> {
//         let inner = self.inner.lock().unwrap();
//         WebViewImpl::evaluate_script(&inner, unsafe { js.as_str() })?;
//         Ok(())
//     }
// }

// pub fn init() -> Result<(), Error> {
//     // define_global_function("webview_eval", WebViewWrapper::eval);

//     class::string()
//            .define_private_method("eval",function!(WebViewWrapper::eval,2));
//     Ok(())
// }
//     // inner: Arc<Mutex<WebViewImpl>>,


