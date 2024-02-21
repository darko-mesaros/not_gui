// TODO:
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand
};
use ratatui::{
    prelude::*,
    widgets::{Paragraph, Gauge}, widgets::{Block, Borders, BorderType},
};
use std::io::stdout;
use anyhow::Result;

struct App {
    counter: u32,
    should_quit: bool,
    background_color: ratatui::style::Color,
}


fn startup() -> Result<()> {
    enable_raw_mode()?; 
    stdout().execute(EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn ui(app: &App, f: &mut Frame) {

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(5),
            Constraint::Percentage(70),
        ])
        .split(f.size());


    // top
    let top_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Twitch Stream TUI");
    
    f.render_widget(
        Paragraph::new(format!("Current count: {}", app.counter))
            .block(top_block)
            .style(Style::new().fg(ratatui::style::Color::Black).bg(app.background_color)),
        layout[0],
    );

    // mid
    let mid_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Progress");
    f.render_widget(
        Gauge::default()
            .block(mid_block)
            .gauge_style(
                Style::default()
                    .fg(Color::Yellow)
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            )
            .percent(37),
        layout[1]
    );

    // bottom
    let bottom_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Main");
    f.render_widget(
        Paragraph::new("There is some text here too!")
            .block(bottom_block)
            .alignment(Alignment::Center)
            .style(Style::new().fg(ratatui::style::Color::LightBlue).bg(ratatui::style::Color::Black)),
        layout[2],
    );


}

fn update(app: &mut App) -> Result<()> {
    app.counter += 1;
    if event::poll(std::time::Duration::from_millis(25))? {
        // if there is a key event
        if let event::Event::Key(key) = event::read()? {
            // and if the key event is Key being pressed
            if key.kind == KeyEventKind::Press {
                match key.code  {
                    KeyCode::Char('q') => app.should_quit = true, 
                    KeyCode::Char('1') => app.background_color = ratatui::style::Color::Blue,
                    KeyCode::Char('2') => app.background_color = ratatui::style::Color::Red,
                    KeyCode::Char('3') => app.background_color = ratatui::style::Color::Green,
                    KeyCode::Char('4') => app.background_color = ratatui::style::Color::Magenta,
                    KeyCode::Char('5') => app.background_color = ratatui::style::Color::Yellow,
                    KeyCode::Char('r') => app.background_color = ratatui::style::Color::Rgb(236,114,17),
                    _=> {},
                };
            }
        }
    }

    Ok(())
}

fn run() ->  Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut app = App {
        counter: 0,
        should_quit: false,
        background_color: ratatui::style::Color::DarkGray,
    };

    loop {
        update(&mut app)?;
        terminal.draw(|f|{
            ui(&app, f);
        })?;
        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn main() -> Result<()>{
    startup()?;
    let _result = run();
    shutdown()?;
    Ok(())
}
