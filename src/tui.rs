use core::prelude;
use ratatui::prelude::{Line, Stylize};
use std::io;

pub fn tui_main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App { exit: false };
    let app_result = app.run(&mut terminal);

    ratatui::restore();
    app_result
}

pub struct App {
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut ratatui::Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl ratatui::widgets::Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Line::from("Yoo guiys").bold().render(area, buf);
    }
}
