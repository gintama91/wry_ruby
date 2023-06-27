use magnus:: Error;

mod tray_id;

pub fn init() -> Result<(), Error> {
    tray_id::init()?;
    Ok(())
}
