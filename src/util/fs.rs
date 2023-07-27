pub mod read_file {
    use std::fs::File;
    use std::io::{self, Read};

    pub fn read_shader_to_string(path: &str) -> Result<String, io::Error> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        return Ok(content);
    }
}
