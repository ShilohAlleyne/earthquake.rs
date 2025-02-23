use crate::{analysis::Risk, data::Asset};

use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Bar, BarChart, BarGroup, Block, Borders, Cell, Row, Table},
    DefaultTerminal, Frame,
};

// Data presented to client
pub struct App {
    client_data: Vec<(Asset, Risk)>,
    eq_occ_by_state: Vec<(String, usize)>,
    av_mag_by_state: Vec<(String, f32)>,
    exit: bool,
}

impl App {
    // Load app
    // Shortcut:
    // By preloading our data, and then starting the app,
    // we can sidestep the complexities of Async rust event loops and such
    pub fn new(
        client_data: Vec<(Asset, Risk)>,
        eq_occ_by_state: Vec<(String, usize)>,
        av_mag_by_state: Vec<(String, f32)>
    ) -> Self {
        Self {
            client_data,
            eq_occ_by_state,
            av_mag_by_state,
            exit: false,
        }
    }
    
    // Run the app rendering loop
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    // Quit App
    fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                self.exit = true;
            }
        }
        Ok(())
    }

    // Draw the dashboard tui
    pub fn draw(&self, frame: &mut Frame) {
        let [eq_stats, client_data] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Fill(1), // Divide the screen in two
        ])
        .spacing(1)
        .areas(frame.area());

        // Make to areas for the different eq stats
        let [occ, mag] = Layout::horizontal([
            Constraint::Percentage(60),
            Constraint::Fill(1),
        ])
        .areas(eq_stats);

        frame.render_widget(draw_occ_barchart(&self.eq_occ_by_state), occ);
        frame.render_widget(draw_mag_barchart(&self.av_mag_by_state), mag);
        frame.render_widget(client_data_portfolio(&self.client_data), client_data);
    }
}

// Function for darwing BarCharts
// BarChart has the same lifetime as the app
pub fn draw_occ_barchart<'a>(data: &[(String, usize)]) -> BarChart<'a> {
    let bars: Vec<Bar> = data.iter()
        .map(|(label, data)| vertical_bar(label, data.to_owned() as u64))
        .collect();

    let title = Line::from("Top 10 States by Earthquake Occurance").centered();
    BarChart::default()
        .block(Block::new().title(title).borders(Borders::ALL))
        .data(BarGroup::default().bars(&bars))
        .bar_width(12)
        .bar_gap(2)
        .direction(Direction::Vertical)
}

pub fn draw_mag_barchart<'a>(data: &[(String, f32)]) -> BarChart<'a> {
    let bars: Vec<Bar> = data.iter()
        .map(|(label, data)| vertical_bar(label, data.to_owned() as u64))
        .collect();

    let title = Line::from("Top 10 States by Average Earthquake Maginitude").centered();
    BarChart::default()
        .block(Block::new().title(title).borders(Borders::ALL))
        .data(BarGroup::default().bars(&bars))
        .bar_width(2)
        .bar_gap(1)
        .direction(Direction::Horizontal)
}

// Draws client data as a table
fn client_data_portfolio<'a>(data: &[(Asset, Risk)]) -> Table<'a> {
    let title = Line::from(" Current Protfolio Risk, Red indicates high risk, Yellow medium and Green low ").centered();
    let header_style = Style::default()
        .fg(Color::Gray);

    let header = ["Building Name", "Location", "Full Address"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(1);

    let rows = data.iter()
        .map(|(asset, risk)| {
            let colour = match risk {
                Risk::High => Color::Red,
                Risk::Medium => Color::Yellow,
                Risk::Low => Color::Green,
            };
            let item: [&str; 3] = [&asset.building_name, &asset.location, &asset.full_address];
            item.into_iter()
                .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
                .collect::<Row>()
                .style(Style::new().bg(colour))
                .height(2)
        });

    Table::new(
        rows,
[
            // + 1 is for padding
            // Shortcut hard code the table sizes
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Min(54),
        ]
    )
    .block(Block::new().title(title).borders(Borders::ALL).title_bottom("Press 'q' to quit"))
    .header(header)
    // .block(Borders::ALL)
}

// Generates a vertical Bar
fn vertical_bar<'a>(label: &String, value: u64) -> Bar<'a> {
    Bar::default()
        .value(value)
        .label(Line::from(label.to_string()))
        .text_value(format!("{value}"))
        .style(Color::Blue)
        .value_style(Style::default().fg(Color::White).bg(Color::Blue))
}
