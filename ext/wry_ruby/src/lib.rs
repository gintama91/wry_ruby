use magnus::{ define_global_function,function, Error};
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

fn hello_wry() {

        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_title("Hello World")
            .build(&event_loop)
            .unwrap();
        let _webview = WebViewBuilder::new(window)
            .unwrap()
            .with_url("https://tauri.studio")
            .unwrap()
            .build()
            .unwrap();
    

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
});
}



#[magnus::init]
fn init() -> Result<(), Error> {
    define_global_function("hello_wry", function!(hello_wry, 0));
    Ok(())
}
