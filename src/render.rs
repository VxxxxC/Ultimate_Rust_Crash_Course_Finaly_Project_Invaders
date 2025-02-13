use crossterm::{
    style::{Color, SetBackgroundColor},
    QueueableCommand,
};

use crate::frame::Frame;
use std::io::Stdout;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
    }
}
