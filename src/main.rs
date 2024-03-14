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
mod visualizer;

fn main() {
        //TODO https://ratatui.rs/tutorials/counter-app/basic-app/

        loop {
            let data = data_reader::read_data();
            visualizer::visualize(data);

            let seconds = time::Duration::from_secs(5);
            sleep(seconds)
        };
    }
