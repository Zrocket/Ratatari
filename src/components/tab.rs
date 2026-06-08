use color_eyre::Result;
use crossterm::event::KeyCode;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;
use serde::{Deserialize, Serialize};

use super::Component;
use crate::{action::Action, config::Config};

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum FocusedTab {
    Sequencer,
    Sampler,
    #[default]
    Test,
    Files,
}

#[derive(Default)]
pub struct TabHandler {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    focus: FocusedTab,
}

impl TabHandler {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for TabHandler {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Char('0') => {
                self.focus = FocusedTab::Test;
                if let Some(tx) = &self.command_tx {
                    tx.send(Action::TabFocus(FocusedTab::Test))?;
                }
            },
            KeyCode::Char('1') => {
                self.focus = FocusedTab::Sequencer;
                if let Some(tx) = &self.command_tx {
                    tx.send(Action::TabFocus(FocusedTab::Sequencer))?;
                }
            },
            KeyCode::Char('2') => {
                self.focus = FocusedTab::Sampler;
                if let Some(tx) = &self.command_tx {
                    tx.send(Action::TabFocus(FocusedTab::Sampler))?;
                }
            },
            KeyCode::Char('3') => {
                self.focus = FocusedTab::Files;
                if let Some(tx) = &self.command_tx {
                    tx.send(Action::TabFocus(FocusedTab::Files))?;
                }
            },
            _ => {}
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
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        Ok(())
    }
}
