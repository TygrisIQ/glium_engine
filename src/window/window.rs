pub mod window {

    use crate::indicies::GIndicies;

    use glium::{
        glutin::{self},
        uniform, Surface,
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
        //run with a primative shape with no indicies, currently indicies are onganized as a triangle list only
        pub fn run_no_indicies<T>(
            s: Self,
            program: glium::Program,
            shape: &[T],
            indicies_type: GIndicies,
            uni: f32,
        ) where
            T: glium::Vertex + 'static,
        {
            let mut u = uni;
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
                u -= 0.0002;
                let s = u.cos() * -0.5;
                let uniform = uniform! { s : s};
                target
                    .draw(
                        &vertex_buffer,
                        &indicies_type,
                        &program,
                        &uniform,
                        &Default::default(),
                    )
                    .unwrap();

                target.finish().unwrap();
            })
        }
    }
}
