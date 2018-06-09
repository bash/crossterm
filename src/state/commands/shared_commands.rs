//! This module contains the commands that can be used for both unix and windows systems.
use { Context, Terminal };
use super::{ ICommand, IContextCommand };
use std::io;
use std::io::Write;

/// This command is used for switching to alternate screen and back to main screen.
#[derive(Clone, Copy)]
pub struct ToAlternateScreenBufferCommand;

impl ICommand for ToAlternateScreenBufferCommand
{
    fn new() -> Box<ToAlternateScreenBufferCommand> {
        Box::from(ToAlternateScreenBufferCommand {})
    }

    fn execute(&mut self, terminal: &Terminal) -> bool
    {
        let mut screen = terminal.screen_manager.lock().unwrap();
        {
            screen.write_ansi_str(csi!("?1049h"));
            return true;
        }
    }

    fn undo(&mut self, terminal: &Terminal) -> bool
    {
        let mut screen = terminal.screen_manager.lock().unwrap();
        {
            screen.write_ansi_str(csi!("?1049l"));
            return true;
        }
    }
}