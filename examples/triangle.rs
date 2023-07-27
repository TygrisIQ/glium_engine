use glium_engine::{
    fs,
    window::{self, window::Gwindow},
};

fn main() {
    let vs = fs::ReadFile::read_shader_to_string("glsl/vs_triangle.glsl").unwrap();
    let fs = fs::ReadFile::read_shader_to_string("glsl/fs_triangle.glsl").unwrap();

    let w = Gwindow::new("triangle example");
    Gwindow::run(w, Some(vs.as_str()), Some(fs.as_str()));
}
