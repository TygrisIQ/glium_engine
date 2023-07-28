use glium;

pub enum UniformOperationType {
    //uniform type, after a unifrom value is passed to the eventloop, use this enum to specify what to do with the uniform
    nothing,
    add(f32),
    minus(f32),
    multiply(f32),
    divide(f32),
}
