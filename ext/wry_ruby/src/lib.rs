use magnus::Error;

mod application;

mod webview;

#[magnus::init]
fn init() -> Result<(), Error> {
    application::init()
    // webview::init()

}
