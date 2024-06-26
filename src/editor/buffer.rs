//#[derive(Default)]
pub struct Buffer {
    pub data: Vec<String>,
}
impl Default for Buffer {
    fn default() -> Self {
        Buffer {
            data: vec![
                String::from("Hello, World"),
                String::from("This is second line"),
            ],
        }
    }
}
impl Buffer {}
