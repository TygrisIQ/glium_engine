pub mod ReadFile {
    use std::io;

    pub fn read_shader_to_string(path: &str) -> Result<String, io::Error> {
        match std::fs::read_to_string(path) {
            Ok(content) => Ok(content),
            Err(error) => Err(error),
        }
    }
}
