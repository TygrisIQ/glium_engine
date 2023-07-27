use glium::{
    glutin::{self},
    Surface,
};

pub struct Gwindow {
    events: glutin::event_loop::EventLoop<()>,
    display: glium::Display,
}

impl Gwindow {
    pub fn new(window_title: &str) -> Gwindow {
        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new().with_title(window_title);
        let cb = glutin::ContextBuilder::new();
        let facade = glium::Display::new(wb, cb, &event_loop).unwrap();

        return Gwindow {
            events: event_loop,
            display: facade,
        };
    }
    pub fn run(s: Self, vertex_shader: Option<&str>, fragment_shader: Option<&str>) {
        s.events.run(move |ev, _, control_flow| {
            let mut target = s.display.draw();
            match ev {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => control_flow.set_exit(),
                    _ => (),
                },
                _ => (),
            }
            target.clear_color(0.0, 0.5, 0.3, 1.0);
            target.finish().unwrap();
        })
    }
}
