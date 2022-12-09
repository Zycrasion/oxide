mod client;
pub mod file;
pub mod utilities;
use crate::client::cli;
use crate::utilities::utils;
use crate::utilities::utils::interface::centre_with;
use client::cli::Terminal;
use file::file_handler::Document;
use termion::event::Key;
use termion::input::{Keys, TermRead};
use termion::raw::IntoRawMode;
use termion::style::Reset;

use std::io::{stdin, stdout, Write};
use std::os::unix::process::CommandExt;
use std::process::{Command, exit};
use std::thread;
use std::time::Duration;
use termion::color;

struct Oxide
{
    pub doc : Document,
    pub interface : Terminal,
    pub line_offset : usize,
    pub cursor_y : u16,
}

impl Oxide 
{
    pub fn new(doc : Document, term : Terminal) -> Oxide
    {
        println!("{}", termion::cursor::Hide);
        Oxide 
        {
            doc : doc,
            interface : term,
            line_offset : 0,
            cursor_y : 1
        }
    }

    pub fn print(&mut self, text : &String)
    {
        let mut padding : usize = 0;
        if text.len() < self.interface.width
        {
           padding = self.interface.width - text.len();
        }
        let padding = String::from(" ").repeat(padding);
        cli::goto(1, self.cursor_y);
        println!("{}{}",text,padding);
        self.cursor_y += 1;
    }

    pub fn flush(&mut self)
    {
        self.cursor_y = 1;
    }

    pub fn render(&mut self)
    {
        cli::goto(1, 1);
        let title = format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        let header = centre_with(" ", &title.as_str(), &self.interface);
    
        self.print(&utils::interface::create_bar(&self.interface, &header, &color::White, &color::Black));
        let mut y = 2;
        for index in 3..self.interface.height {
            cli::goto(1,y);
            y += 1;
            let mut content = String::new();
            {
                let line = self.doc.content.get(index + self.line_offset);
                match line 
                {
                    Some(x) => {content = String::clone(x)},
                    None => {content = String::from(" ")}
                }
            }
            self.print(&content);
        }
    
        let footer_text = String::from(format!(
            "FILE : {}, LINES : {}-{}/{}",
            self.doc.title,
            self.line_offset,
            self.interface.height - 3 + usize::from(self.line_offset),
            self.doc.length,
        ));
        self.print(&utils::interface::create_bar(&self.interface, &footer_text, &color::White, &color::Black));
        self.flush();
    }

    pub fn run(&mut self)
    {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let stdin = stdin();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Ctrl('c') => {
                    Command::new("reset").exec();
                    std::process::exit(0);
                }
                Key::Down => {
                    self.line_offset += 1;
                    self.render();
                }

                Key::Up => {
                    if self.line_offset > 0
                    {
                        self.line_offset -= 1;
                    }
                    self.render();
                }
                _ => {
                    self.render();
                }
            }
            stdout.flush().unwrap();
        }
    }
}

fn wait(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}

fn main() {
    cli::clear();
    let interface = cli::Terminal::new();

    let args : Vec<String> = std::env::args().collect();
    if args.len() < 2
    {
        println!("Please specify a file");
        exit(1);
    }

    if !std::path::Path::new(&args[1]).exists()
    {
        println!("Path doesn't exist!");
        exit(1);
    }

    let file = Document::new(&args[1]);
    let mut app = Oxide::new(file, interface);
    app.render();
    app.run();
}