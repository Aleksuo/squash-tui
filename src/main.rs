use std::io;

use color_eyre::{eyre::Ok, owo_colors::OwoColorize};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use git2::Repository;
use ratatui::{
    DefaultTerminal, Frame, border,
    layout::{Constraint, Layout, Rect},
    symbols::border,
    text::Line,
    widgets::{Block, Cell, Paragraph, Widget},
};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    current_branch_name: String,
}

impl App {
    pub fn init(&mut self) -> color_eyre::Result<()> {
        color_eyre::install()?;
        let repo = Repository::open_from_env()?;
        for branch_and_type in repo.branches(None)? {
            let (branch, _type) = branch_and_type?;
            let name = branch.name()?.unwrap();
            print!("{name}");
        }
        Ok(())
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let col_constraints = (0..3).map(|_| Constraint::Length(50));
        let row_constraints = (0..1).map(|_| Constraint::Length(100));
        let horizontal = Layout::horizontal(col_constraints).spacing(1);
        let vertical = Layout::vertical(row_constraints).spacing(1);

        let rows = vertical.split(area);
        let cells: Vec<Rect> = rows
            .iter()
            .flat_map(|&row| horizontal.split(row).to_vec())
            .collect();

        Paragraph::new("Branches content placeholder")
            .block(
                Block::bordered()
                    .title(Line::from("Branches").centered())
                    .border_set(border::DOUBLE),
            )
            .render(cells[0], buf);
        Paragraph::new("Commits content placeholder")
            .block(
                Block::bordered()
                    .title(Line::from("Commits").centered())
                    .border_set(border::DOUBLE),
            )
            .render(cells[1], buf);
        Paragraph::new("Commit info placeholder")
            .block(
                Block::bordered()
                    .title(Line::from("Commit info").centered())
                    .border_set(border::DOUBLE),
            )
            .render(cells[2], buf);
    }
}

fn main() -> color_eyre::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::default();
    let init_result = app.init();
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}
