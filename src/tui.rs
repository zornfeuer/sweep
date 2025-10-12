use crate::types::SweepItem;
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, style::palette::tailwind, widgets::*};
use std::io::{stdout, Stdout};

pub struct App {
    items: Vec<SweepItem>,
    selected: Vec<bool>,
    cursor: usize,
    should_quit: bool,
    dry_run: bool,
}

impl App {
    pub fn new(items: Vec<SweepItem>, dry_run: bool) -> Self {
        let selected = vec![false; items.len()];
        Self {
            items,
            selected,
            cursor: 0,
            should_quit: false,
            dry_run,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut terminal = init_terminal()?;
        terminal.clear()?;

        loop {
            terminal.draw(|frame| self.render(frame))?;

            if self.should_quit {
                break;
            }

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                        KeyCode::Down => self.cursor = (self.cursor + 1).min(self.items.len().saturating_sub(1)),
                        KeyCode::Up => self.cursor = self.cursor.saturating_sub(1),
                        KeyCode::Char(' ') => {
                            if !self.items.is_empty() {
                                self.selected[self.cursor] = !self.selected[self.cursor];
                            }
                        },
                        KeyCode::Enter => {
                            self.confirm_and_remove()?;
                            self.should_quit = true;
                        },
                        _ => {},
                    }
                }
            }
        }

        restore_terminal()?;
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let palette = tailwind::BLUE;

        let title = if self.dry_run {
            "🧹 sweep (DRY RUN — nothing will be deleted)"
        } else {
            "🧹 sweep — USE ARROWS, SPACE, ENTER"
        };

        let block = Block::bordered()
            .title(title)
            .title_alignment(Alignment::Center)
            .border_style(Style::new().fg(palette.c400));

        let items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let prefix = if self.selected[i] { "✓ " } else { "  " };
                let line = Line::from(format!("{}{}", prefix, item));
                ListItem::new(line).style(if i == self.cursor { Style::new().bg(palette.c900) } else { Style::new() })
            })
            .collect();

        let list = List::new(items).block(block).highlight_symbol(">> ");
        frame.render_widget(list, area);
    }

    fn confirm_and_remove(&self) -> Result<()> {
        if self.dry_run {
            println!("\n✅ DRY RUN: would remove:");
            for (i, item) in self.items.iter().enumerate() {
                if self.selected[i] {
                    println!("  - {}", item)
                }
            }
        } else {
            // TODO: Удаление
            println!("\n⚠️  Real removal not implemented yet.");
        }
        Ok(())
    }
}

fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
