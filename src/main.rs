use color_eyre::eyre::Ok;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use git2::Repository;
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, List, ListItem, Paragraph, Widget},
};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    branches: Vec<String>,
    selected_branch_index: Option<usize>,
    selector_cursor_index: usize,
}

impl App {
    pub fn init(&mut self) -> color_eyre::Result<()> {
        color_eyre::install()?;
        self.branches = Vec::new();
        self.selected_branch_index = None;
        let repo = Repository::open_from_env()?;
        for branch_and_type in repo.branches(None)? {
            let (branch, _type) = branch_and_type?;
            let name = branch.name()?.unwrap();
            self.branches.push(name.to_string());
        }
        Ok(())
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        while !self.exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
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
            KeyCode::Char('j') => self.select_next(),
            KeyCode::Char('k') => self.select_previous(),
            KeyCode::Enter => self.select_current(),
            _ => {}
        }
    }

    fn select_next(&mut self) {
        let mut cur = self.selector_cursor_index;
        let len = self.branches.len();
        cur += 1;
        if cur >= len {
            self.selector_cursor_index = 0;
            return;
        }
        self.selector_cursor_index = cur;
    }

    fn select_previous(&mut self) {
        let mut cur = self.selector_cursor_index;
        let len = self.branches.len();
        if len == 0 {
            return;
        }
        if cur == 0 {
            self.selector_cursor_index = len - 1;
            return;
        }
        cur -= 1;

        self.selector_cursor_index = cur;
    }

    fn select_current(&mut self) {
        self.selected_branch_index = Some(self.selector_cursor_index);
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn layout(area: Rect) -> (Rect, Rect, Rect) {
        let col_constraints = (0..3).map(|_| Constraint::Length(50));
        let row_constraints = (0..1).map(|_| Constraint::Length(100));
        let horizontal = Layout::horizontal(col_constraints).spacing(1);
        let vertical = Layout::vertical(row_constraints).spacing(1);

        let rows = vertical.split(area);
        let cells: Vec<Rect> = rows
            .iter()
            .flat_map(|&row| horizontal.split(row).to_vec())
            .collect();
        (cells[0], cells[1], cells[2])
    }

    fn render_branches(&mut self, area: Rect, buf: &mut Buffer) {
        let mut branch_items: Vec<ListItem> = Vec::new();
        for (i, branch_name) in self.branches.iter().enumerate() {
            let mut is_selected = false;
            if let Some(branch_index) = self.selected_branch_index {
                is_selected = branch_index == i;
            }
            branch_items.push(Self::render_branch(
                is_selected,
                branch_name.to_string(),
                self.selector_cursor_index == i,
            ));
        }

        List::new(branch_items)
            .block(
                Block::bordered()
                    .title(Line::from("Branches").centered())
                    .border_set(border::DOUBLE),
            )
            .render(area, buf);
    }

    fn render_branch(
        is_selected: bool,
        branch_name: String,
        is_pointed: bool,
    ) -> ListItem<'static> {
        let mut display_name = branch_name;
        if is_selected {
            display_name = format!("► {}", display_name);
        } else {
            display_name = format!("  {}", display_name);
        }
        if is_pointed {
            return ListItem::new(display_name).blue();
        }
        return ListItem::new(display_name);
    }

    fn render_commits(&mut self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Commits content placeholder")
            .block(
                Block::bordered()
                    .title(Line::from("Commits").centered())
                    .border_set(border::DOUBLE),
            )
            .render(area, buf);
    }

    fn render_commit_info(&mut self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Commit info placeholder")
            .block(
                Block::bordered()
                    .title(Line::from("Commit info").centered())
                    .border_set(border::DOUBLE),
            )
            .render(area, buf);
    }
}

impl Widget for &mut App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let (branches_area, commits_area, commit_info_area) = App::layout(area);

        self.render_branches(branches_area, buf);
        self.render_commits(commits_area, buf);
        self.render_commit_info(commit_info_area, buf);
    }
}

fn main() -> color_eyre::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::default();
    let _init_result = app.init();
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}
