use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

pub fn open_browser(title: &str) {
    let el = EventLoop::new();
    let wb = create_window(title);

    let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    println!(
        "Pixel format of the window's GL context: {:?}",
        windowed_context.get_pixel_format()
    );

    // let gl = support::load(&windowed_context.context());

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                // gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}

fn create_window(title: &str) -> WindowBuilder {
    WindowBuilder::new().with_resizable(true).with_title(title)
}
