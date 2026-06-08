use color_eyre::Result;
use crossterm::event::KeyCode;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::{tab::FocusedTab, Component};
use crate::{action::Action, config::Config};

#[derive(Default)]
pub struct FilesWidgit {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    active: bool,
}

impl FilesWidgit {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for FilesWidgit {
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
            match key.code {
                KeyCode::Char('h') => {}
                KeyCode::Char('j') => {}
                KeyCode::Char('k') => {}
                KeyCode::Char('l') => {}
                _ => {}
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
                self.active = focus == FocusedTab::Files;
            },
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        if self.active {
            frame.render_widget(Paragraph::new("Files"), area);
        }
        Ok(())
    }
}
