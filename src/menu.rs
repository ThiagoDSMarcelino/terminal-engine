use crossterm::{
    cursor,
    event::{Event, KeyCode},
    terminal, ExecutableCommand,
};

use std::io::{self, Result};

pub struct MenuOption {
    name: String,
    action: fn() -> Result<()>,
}

impl MenuOption {
    pub fn new(name: &str, action: fn() -> Result<()>) -> Self {
        Self {
            name: name.to_string(),
            action,
        }
    }
}

pub struct Menu {
    options: Vec<MenuOption>,
    selected: usize,
    first: bool,
}

impl Menu {
    pub fn new(options: Vec<MenuOption>) -> Self {
        Self {
            options,
            selected: 0,
            first: true,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;

        loop {
            self.print()?;
            if let Event::Key(event) = crossterm::event::read()? {
                self.handle_event(Event::Key(event))?;
            }

            if self.first {
                self.first = false;
            }
        }
    }

    fn print(&self) -> Result<()> {
        let mut stdout = io::stdout();
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        stdout.execute(cursor::MoveTo(0, 0))?;

        for (index, option) in self.options.iter().enumerate() {
            if index == self.selected {
                println!("> {}", option.name);
            } else {
                println!("  {}", option.name);
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> Result<()> {
        match event {
            Event::Key(event) => match event.code {
                KeyCode::Up => {
                    if self.selected > 0 {
                        self.selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.selected < self.options.len() - 1 {
                        self.selected += 1;
                    }
                }
                KeyCode::Enter => {
                    if self.first {
                        return Ok(());
                    }

                    let action = self.options[self.selected].action;
                    action()?;
                }
                _ => {}
            },
            _ => {}
        }

        Ok(())
    }
}
