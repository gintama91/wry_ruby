use magnus::Error;

mod tray_id;

mod window;

mod clipboard;

mod event_loop;

pub fn init() -> Result<(), Error> {
    tray_id::init()?;
    // window::init()?;
    clipboard::init()?;
    event_loop::init()?;
    Ok(())
}
