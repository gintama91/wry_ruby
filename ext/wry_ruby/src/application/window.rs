use magnus::{Error, define_global_function, function, define_class};
use wry::application::dpi::LogicalSize;
use wry::application::window::WindowBuilder;

use wry::{
    application::{
      event::{Event, StartCause, WindowEvent},
      event_loop::{ControlFlow, EventLoop},
    },
    webview::WebViewBuilder,
  };

pub fn WindoWnew(title: String, width: u32, height: u32) {
    let event_loop = wry::application::event_loop::EventLoop::new();
    let mut window_builder = WindowBuilder::new();
    window_builder = window_builder.with_title(title.clone());
    window_builder = window_builder.with_inner_size(LogicalSize::new(width, height)); 
    window_builder = window_builder.with_resizable(false);

    let window = window_builder.build(&event_loop).expect("Failed to create window");

    // Get initial properties
    let current_title = window.title();
    let current_size = window.inner_size();
    let resizable = window.is_resizable();

    println!("Initial Title: {}", current_title);
    println!("Initial Size: {:?}", current_size);
    println!("Resizable: {}", resizable);

  
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


pub fn init() -> Result<(), Error> {

     define_global_function("new_window", function!(WindoWnew, 3));
    // define_global_function("hello_wry", function!(new, 3));
    // application::init();
    Ok(())
}