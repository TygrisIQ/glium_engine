use glium::{self, implement_vertex, program};
use glium_engine::{
    fs,
    window::{self, indicies::GIndicies, window::Gwindow},
};
#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}
fn main() {
    //loading the vertex & fragement shader source files

    implement_vertex!(Vertex, position);
    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [-0.5, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, 0.0],
    };
    let verticies = vec![vertex1, vertex2, vertex3];
    let vs = fs::read_file::read_shader_to_string("glsl/tri_vs.glsl").unwrap();
    let fs = fs::read_file::read_shader_to_string("glsl/tri_fs.glsl").unwrap();
    let w = Gwindow::new("triangle example");
    Gwindow::run_no_indicies(
        w,
        &vs,
        &fs,
        &verticies,
        GIndicies::TrianglesList(glium::index::PrimitiveType::TrianglesList),
    );
}
