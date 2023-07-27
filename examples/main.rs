use glium_engine::fs;

fn main() {
    let file = fs::read_file::read_shader_to_string("../test.txt").unwrap();
    println!("{}", file.as_str());
}
