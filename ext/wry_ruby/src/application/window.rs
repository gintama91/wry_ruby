use magnus::{Error, define_global_function, function};
use wry::application::dpi::LogicalSize;
use wry::application::window::WindowBuilder;
use wry::application::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use std::time::{Duration, Instant};

pub fn WindoWnew(title: String, width: u32, height: u32, resizable: bool,timeout:u64) {
    let event_loop = EventLoop::new();
    let mut window_builder = WindowBuilder::new();
    window_builder = window_builder.with_title(title.clone());
    window_builder = window_builder.with_inner_size(LogicalSize::new(width, height));
    window_builder = window_builder.with_resizable(resizable);

    let window = window_builder.build(&event_loop).expect("Failed to create window");

    // Get initial properties
    let current_title = window.title();
    let current_size = window.inner_size();
    let resizable = window.is_resizable();

    println!("Initial Title: {}", current_title);
    println!("Initial Size: {:?}", current_size);
    println!("Resizable: {}", resizable);

    // ******************************************************* for tests timeout ************************************************************************
    let timeout_duration = Duration::from_secs(timeout);
    let start_time = Instant::now();

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
        let elapsed = Instant::now() - start_time;
        if elapsed >= timeout_duration {
            *control_flow = ControlFlow::Exit
        }

    });
}

pub fn init() -> Result<(), Error> {
    println!("inside init");
    define_global_function("new_window", function!(WindoWnew, 5));

    Ok(())
}