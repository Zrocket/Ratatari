use color_eyre::{eyre::Ok, Result};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, audio::sound_test, config::Config};

#[derive(Default)]
pub struct AudioTest {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

impl AudioTest {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for AudioTest {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
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
        //frame.render_widget(Paragraph::new("hello world"), area);
        Ok(())
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Char('a') => {
                sound_test();
                Ok(None)
            },
            _ =>Ok(None)
        }
    }
}
