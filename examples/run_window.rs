fn main() {
    use glium_engine::Gwindow;
    let w = Gwindow::new("blank window");
    Gwindow::run(w, None, None);
}
