use magnus::Error;

mod application;

#[magnus::init]
fn init() -> Result<(), Error> {
    application::init()
}