use color_eyre::Result;
use ratatui::{prelude::*, widgets::*, text::Span, layout::Rect, style::{Color, Modifier, Style, Stylize},};
use tokio::sync::mpsc::UnboundedSender;

use super::{tab::FocusedTab, Component};
use crate::{action::Action, config::Config};

#[derive(Default)]
struct Signal {
    x: f64,
    interval: f64,
    period: f64,
    scale: f64,
}

impl Signal {
    const fn new(interval: f64, period: f64, scale: f64) -> Self {
        Self {
            x: 0.0,
            interval,
            period,
            scale
        }
    }
}

impl Iterator for Signal {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        let point = (self.x, (self.x * 1.0 / self.period).sin() * self.scale);
        self.x += self.interval;
        Some(point)
    }
}

#[derive(Default)]
pub struct SamplerWidgit {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    active: bool,
    signal1: Signal,
    signal2: Signal,
    data1: Vec<(f64, f64)>,
    data2: Vec<(f64, f64)>,
    window: [f64; 2],
}

impl SamplerWidgit {
    pub fn new() -> Self {
        let mut signal1 = Signal::new(0.2, 3.0, 18.0);
        let mut signal2 = Signal::new(0.1, 2.0, 10.0);
        let data1 = signal1.by_ref().take(200).collect::<Vec<(f64, f64)>>();
        let data2 = signal2.by_ref().take(200).collect::<Vec<(f64, f64)>>();
        Self {
            signal1,
            signal2,
            data1,
            data2,
            window: [0.0, 20.0],
            ..Default::default()
        }
    }
}

impl Component for SamplerWidgit {
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
                self.data1.drain(0..5);
                self.data1.extend(self.signal1.by_ref().take(5));

                self.data2.drain(0..10);
                self.data2.extend(self.signal2.by_ref().take(10));

                self.window[0] += 1.0;
                self.window[1] += 1.0;
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            Action::TabFocus(focus) => {
                self.active = focus == FocusedTab::Sampler;
            },
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        if self.active {
            frame.render_widget(Paragraph::new("Sampler"), area);

            /*let x_lables = vec![
                Span::styled(
                    format!("{}", self.window[0]),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw(format!("{}", (self.window[0] + self.window[1]) / 2.0)),
                Span::styled(
                    format!("{}", self.window[1]),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ];*/

            let datasets = vec![
                Dataset::default()
                    .name("data1")
                    .marker(symbols::Marker::Dot)
                    .style(Style::default().fg(Color::Cyan))
                    .data(&self.data1),
                Dataset::default()
                    .name("data2")
                    .marker(symbols::Marker::Braille)
                    .style(Style::default().fg(Color::Yellow))
                    .data(&self.data2),
            ];

            let chart = Chart::new(datasets)
                .block(Block::bordered())
                .x_axis(
                    Axis::default()
                        //.title("X Axis")
                        //.style(Style::default().fg(Color::Gray))
                        //.labels(x_lables)
                        .bounds(self.window),
                )
                .y_axis(
                    Axis::default()
                        //.title("Y Axis")
                        //.style(Style::default().fg(Color::Gray))
                        //.labels(["-20".bold(), "0".into(), "20".bold()])
                        .bounds([-20.0, 20.0]),
                );

            frame.render_widget(chart, area);
        }
        Ok(())
    }
}
