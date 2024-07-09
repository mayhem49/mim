use super::terminal::Size;
use std::io::Error;
pub trait UIComponent {
    fn mark_redraw(&mut self, redraw: bool);
    fn needs_redraw(&self) -> bool;

    fn set_size(&mut self, size: Size);
    fn draw(&self, y_position: usize) -> Result<(), Error>;

    fn resize(&mut self, size: Size) {
        self.set_size(size);
        self.mark_redraw(true);
    }

    fn render(&mut self, start_y: usize) {
        if self.needs_redraw() {
            match self.draw(start_y) {
                Ok(()) => self.mark_redraw(false),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    panic!("Error when rendering component {err:?}")
                }
            }
        }
    }
}
