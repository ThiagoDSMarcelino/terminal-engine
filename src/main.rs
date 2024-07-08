use std::{
    io::{self, Result, Write},
    process::exit,
    thread,
    time::Duration,
};

use crossterm::{cursor, terminal, ExecutableCommand};
use menu::{Menu, MenuOption};

mod menu;

fn main() {
    let menu_options = vec![
        MenuOption::new("Start", start_animation),
        MenuOption::new("Exit", || exit(0)),
    ];

    let mut menu = Menu::new(menu_options);

    match menu.run() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}

fn start_animation() -> Result<()> {
    let mut stdout = io::stdout();

    let (cols, rows) = terminal::size()?;
    let (mut x, mut y) = (0, 0);
    let direction = (1, 1);

    loop {
        stdout.execute(cursor::MoveTo(x, y))?;
        print!("█");
        stdout.flush()?;

        thread::sleep(Duration::from_millis(100));

        stdout.execute(cursor::MoveTo(x, y))?;
        print!(" ");

        x = (x as i16 + direction.0) as u16;
        y = (y as i16 + direction.1) as u16;

        if x == cols - 1 || x == 0 {
            break;
        }
        if y == rows - 1 || y == 0 {
            break;
        }
    }

    Ok(())
}
