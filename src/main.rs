use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Result};

use core::time;
use std::thread::sleep;

mod data_reader;

fn main() -> Result<()> {
        //TODO https://ratatui.rs/tutorials/counter-app/basic-app/

        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;

        loop {
            let data: Vec<data_reader::ProcessData> = data_reader::read_data();    

            terminal.draw(|frame|{
                let area = frame.size();
                for process in data {
                    let line = 
                    process.pid.to_string() + " " + &process.parent_pid.to_string() + " " + &process.filename + " " + &process.state.to_string() + &process.nice.to_string() + &process.starttime.to_string();
                    frame.render_widget(Paragraph::new(line).white().on_blue(), area);
                }
            })?;


            //handle events here too:
            if event::poll(std::time::Duration::from_millis(4))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press
                        && key.code == KeyCode::Char('q')
                    {
                        break;
                    }
                }
            }
        };

        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }
