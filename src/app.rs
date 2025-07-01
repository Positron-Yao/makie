use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, Paragraph, Widget};

pub struct App {
    pub output: String,
    pub window_height: u16,
}

impl App {
    pub fn new(output: &String) -> Self {
        App {
            output: output.to_owned(),
            window_height: output.lines().count() as u16 + 2,
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for _ in 0..self.window_height-2 {
            println!();
        }

        let bottom = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(100),
                Constraint::Length(self.window_height),
            ])
            .split(area);

        Paragraph::new(self.output.clone())
            .block(Block::default()
                .borders(Borders::ALL)
                .style(Style::default()))
            .render(bottom[1], buf);
    }
}
