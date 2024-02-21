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
    prelude::{CrosstermBackend, Terminal, Style, Frame},
    widgets::Paragraph, widgets::{Block, Borders},
};
use std::io::{stdout, Result};

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
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Twitch Stream TUI");
    f.render_widget(
        // createing a new paragraph
        Paragraph::new(format!("Current count: {}", app.counter))
            .block(block)
            .style(Style::new().fg(ratatui::style::Color::Black).bg(app.background_color)),
        // paragraph size
        {
            let mut r = f.size();
            r.height = r.height/2;
            r
        }
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
        background_color: ratatui::style::Color::White,
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
