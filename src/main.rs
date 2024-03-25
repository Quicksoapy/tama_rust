use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    layout::Rect, prelude::{CrosstermBackend, Stylize, Terminal}, widgets::Paragraph
};
use std::io::{stdout, Result};

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
                // Calculate the height required for each process line
                let line_height = 1;
                let available_height = area.height as usize;
                let num_lines = data.len().min(available_height / line_height);
                let header_line = "PID Parent_PID Filename State Nice Starttime";
                let vertical_scroll = 0; // from app state

                for (i, process) in data.iter().take(num_lines).enumerate() {   
                    let line = format!(
                        "{} {} {} {} {} {}",
                        process.pid, process.parent_pid, process.filename, 
                        process.state, process.nice, process.starttime
                    );
                frame.render_widget(Paragraph::new(items.clone())
                .scroll((vertical_scroll as u16, 0))
                .block(Block::new().borders(Borders::RIGHT)), frame.size());
                frame.render_widget(Paragraph::new(header_line).white().on_black(), Rect::new(area.left(), area.top(), area.width, line_height as u16));
                
                // Calculate the vertical position for each line
                let y_position = area.top() + i as u16 * line_height as u16;
                frame.render_widget(Paragraph::new(line).white().on_blue(), Rect::new(area.left(), y_position, area.width, line_height as u16));
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
