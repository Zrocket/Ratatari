use color_eyre::{owo_colors::OwoColorize, Result};
use crossterm::event::KeyCode;
use ratatui::{prelude::*, widgets::{self, *}};
use tokio::sync::mpsc::UnboundedSender;

use super::{tab::FocusedTab, Component};
use crate::{action::Action, config::Config};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
enum Mode {
    #[default]
    Normal,
    Edit,
    Command,
}

pub struct Tile {
    note: String,
    vol: String,
    fx: String,
}

#[derive(Default)]
pub struct SequencerWidgit {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    active: bool,
    columns: usize,
    rows: usize,
    active_column: usize,
    active_row: usize,
    active_table: usize,
    mode: Mode,
    table_states: Vec<TableState>,
}

impl SequencerWidgit {
    pub fn new() -> Self {
        let mut ret = Self {
            columns: 5,
            rows: 5,
            active_table: 0,
            active_row: 1,
            active_column: 1,
            ..Default::default()
        };
        //ret.state.select_cell(Some((0, 0)));
        for _i in 0..ret.columns {
            ret.table_states.push(TableState::default());
        }
        ret
    }

    fn normal_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('h') | KeyCode::Left => {
                if self.active_column == 0 {
                    if self.active_table != 0 {
                        self.active_column = 2;
                        self.table_states[self.active_table].select_cell(None);
                        self.active_table -= 1;
                    }
                } else {
                    self.active_column -= 1;
                }
            }
            KeyCode::Char('j') | KeyCode::Down => {
                if self.active_row < self.rows - 1 {
                    self.active_row += 1
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.active_row != 0 {
                    self.active_row -= 1;
                }
            }
            KeyCode::Char('l') | KeyCode::Right => {
                self.active_column = self.active_column.wrapping_add(1);
                if self.active_column > 2  && self.active_table != self.columns - 1 {
                    self.active_column = 0;
                    self.table_states[self.active_table].select_cell(None);
                    self.active_table += 1;
                } else if self.active_column > 2 {
                    self.active_column = 2;
                }
            }
            KeyCode::Char('i') => {
                self.mode = Mode::Edit;
            }
            KeyCode::Char(':') => {
                self.mode = Mode::Command;
            }
            _ => {}
        }

        self.table_states[self.active_table].select_cell(Some((self.active_row, self.active_column)));

        Ok(())
    }

    fn edit_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
            }
            _ => {}
        }

        Ok(())
    }

    fn command_mode(&mut self, key: crossterm::event::KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
            }
            _ => {}
        }

        Ok(())
    }

    /// Calculate the layout of the UI elements.
    ///
    /// Returns a tuple of the title area and the main areas.
    fn calculate_layout(&mut self, area: Rect) -> (Rect, Vec<Vec<Rect>>) {
        let main_layout = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]);
        let block_layout = Layout::vertical([Constraint::Max(10); 9]);
        let [title_area, main_area] = main_layout.areas(area);
        let main_areas = block_layout
            .split(main_area)
            .iter()
            .map(|&area| {
                Layout::horizontal([Constraint::Length(11), Constraint::Length(11), Constraint::Length(11), Constraint::Length(11), Constraint::Length(11)])
                    .split(area)
                    .to_vec()
            })
            .collect();
        (title_area, main_areas)
    }
}

impl Component for SequencerWidgit {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<Option<Action>> {
        if self.active {
            match self.mode {
                Mode::Normal => {self.normal_mode(key)?}
                Mode::Edit => {self.edit_mode(key)?}
                Mode::Command => {self.command_mode(key)?}
            }
        }

        Ok(None)
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            Action::TabFocus(focus) => {
                self.active = focus == FocusedTab::Sequencer;
            },
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        if self.active {
            let (title_area, layout) = self.calculate_layout(frame.area());

            frame.render_widget(
                Paragraph::new(format!("Table {}: row {}: col {}", self.active_table, self.active_row, self.active_column))
                    .alignment(Alignment::Center),
                title_area
            );

            let mut rows = Vec::<Row>::new();
            let widths = [Constraint::Length(3), Constraint::Length(2), Constraint::Length(2)];
            for _i in 0..self.rows {
                let row: Vec<&str> = ["---", "--", "--"]
                    .into_iter()
                    .cycle()
                    .take(self.columns * 3)
                    .collect();
                rows.push(Row::new(row.clone()));
            }
            let table = Table::new(rows.clone(), widths)
                .block(
                    Block::default()
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                )
                .cell_highlight_style(Style::default().add_modifier(Modifier::REVERSED));
            frame.render_stateful_widget(table.clone(), layout[0][0], &mut self.table_states[0]);
            frame.render_stateful_widget(table.clone(), layout[0][1], &mut self.table_states[1]);
            frame.render_stateful_widget(table.clone(), layout[0][2], &mut self.table_states[2]);
            frame.render_stateful_widget(table.clone(), layout[0][3], &mut self.table_states[3]);
            frame.render_stateful_widget(table.clone(), layout[0][4], &mut self.table_states[4]);
        }
        Ok(())
    }
}
