pub use button::Button;
pub use label::Label;
pub use window::Window;

mod button;
mod label;
mod window;

pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error>;

    /// Draw the widget on standard output.
    fn draw(&self) -> Result<(), std::fmt::Error> {
        let mut buffer = String::new();
        self.draw_into(&mut buffer)?;
        println!("{buffer}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::gui::{Button, Label, Widget, Window};

    #[test]
    fn main() -> Result<(), std::fmt::Error> {
        let mut window = Window::new("Rust GUI Demo 1.23");
        window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
        window.add_widget(Box::new(Button::new("Click me!")));
        window.draw()?;
        Ok(())
    }
}
