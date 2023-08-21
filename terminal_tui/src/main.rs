
mod terminal_lib {
    pub(crate) mod element;
}

use crate::terminal_lib::{element::InputMode, element::App};

use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Text, Spans},
    widgets::{Block, Borders, Clear, Paragraph, ListItem, List},
    Frame, Terminal,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use unicode_width::UnicodeWidthStr;


fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::default();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if *app.get_popup() {
                match key.code {
                    KeyCode::Enter => {
                        //app.messages.push(app.input.drain(..).collect());
                        app.command_clear();
                    }
                    KeyCode::Char(c) => {
                        app.command_push(c);
                    }
                    KeyCode::Backspace => {
                        app.command_pop();
                    }
                    KeyCode::Esc => {
                        app.command_clear();
                        app.set_popup(false);
                    }
                    _ => {}
                }
            }
            else{
                match app.get_input_mode() {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('i') => {
                            app.set_input_mode(InputMode::Editing);
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        },
                        KeyCode::Char('p') => {
                            app.set_popup(true);
                        },
                        KeyCode::Down => app.item_next(),
                        KeyCode::Up => app.item_previous(),
                        KeyCode::Delete => app.item_clear(),
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            //app.messages.push(app.input.drain(..).collect());
                        }
                        KeyCode::Char(c) => {
                            app.input_push(c);
                        }
                        KeyCode::Backspace => {
                            app.input_pop();
                        }
                        KeyCode::Esc => {
                            app.set_input_mode(InputMode::Normal);
                            app.input_clear();
                        }
                        _ => {}
                    },
                };
            }
            

        }
    }
}


fn ui <B: Backend>(f: &mut Frame<B>,app: &mut App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(30), Constraint::Percentage(30), Constraint::Percentage(30)].as_ref())
        .split(chunks[0]);

    let (msg, style) = match app.get_input_mode() {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, left_chunks[0]);

    match app.get_input_mode() {
        InputMode::Normal =>{
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
        },
        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                left_chunks[1].x + app.get_input().width() as u16 + 1 ,
                // Move one line down, from the border to the input line
                left_chunks[1].y + 1,
            )
        }
    }

    let input = Paragraph::new(app.get_input().as_ref())
        .style(match app.get_input_mode() {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Search Bar"));
    f.render_widget(input, left_chunks[1]);
    

    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|i| {
            let mut lines = vec![];
            lines.push(Spans::from(Span::styled(
                *i,
                Style::default().add_modifier(Modifier::ITALIC),
            )));

            ListItem::new(lines).style(Style::default())
        })
        .collect();
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Result"))
        .highlight_style(Style::default().bg(Color::LightGreen).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");
    f.render_stateful_widget(items, left_chunks[2], &mut app.items.state);
    let block = Block::default().title("System Info").borders(Borders::ALL);
    f.render_widget(block, left_chunks[3]);


    let right_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(chunks[1]);
    let block = Block::default().title("File info").borders(Borders::ALL);
    f.render_widget(block, right_chunk[0]);
    
    if *app.get_popup() {
        let input = Paragraph::new(app.get_command().as_ref())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Change to Directory"));
        //let block = Block::default().title("Popup").borders(Borders::ALL);
        let area = centered_rect(60, 20, size);
        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(input, area);
    }
}


fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}