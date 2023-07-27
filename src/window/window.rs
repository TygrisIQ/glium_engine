use crate::{indicies::GIndicies, window::indicies};
use glium::{
    glutin::{self},
    uniforms::Uniforms,
    Surface,
};
pub struct Gwindow {
    events: glutin::event_loop::EventLoop<()>,
    pub display: glium::Display,
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
    pub fn run_blank(s: Self) {
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
    pub fn run_no_indicies<T>(
        s: Self,
        vs_src: &str,
        fs_src: &str,
        shape: &[T],
        indicies_type: GIndicies,
    ) where
        T: glium::Vertex + 'static,
    {
        let program = glium::Program::from_source(&s.display, &vs_src, &fs_src, None).unwrap();
        let vertex_buffer = glium::VertexBuffer::new(&s.display, &shape).unwrap();
        let indicies_type = match indicies_type {
            GIndicies::TrianglesList(glium::index::PrimitiveType::TrianglesList) => {
                glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList)
            }
            _ => glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        };
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

            target
                .draw(
                    &vertex_buffer,
                    &indicies_type,
                    &program,
                    &glium::uniforms::EmptyUniforms,
                    &Default::default(),
                )
                .unwrap();

            target.finish().unwrap();
        })
    }
}
