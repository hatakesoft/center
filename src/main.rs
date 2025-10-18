//! # CONTENT
//! main program

mod cmd;
mod display;
mod file;

use {
    std:: {
        env,
        io,
        time,
    },
    crossterm:: {
        cursor,
        event,
        execute,
        terminal,
    },
};

/// # CONTENT
/// main program
/// # ARGUMENT
/// none
/// # RETURN VALUE
/// Ok(()): ok
/// Err(...): error
fn main() -> io::Result<()> {
    // init variable
    let mut cmd = cmd::Cmd::new();
    let mut display_handle = display::Display::new();
    display_handle.resize()?;
    let args: Vec<String> = env::args().collect();
    let mut file_handle = file::File::new(args.get(1).cloned());
    file_handle.read()?;

    // init display
    terminal::enable_raw_mode()?;
    execute!(
        io::stdout(),
        cursor::Hide,
        terminal::EnterAlternateScreen,
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All),
    )?;

    loop {
        match event::read()? {
            event::Event::Key(key) => {
                match key.code {
                    event::KeyCode::Char(kc) => {
                        if cmd.key(kc, &mut display_handle, &file_handle) == false {
                            break;
                        }
                    }
                    event::KeyCode::Backspace => {
                        if cmd.key('\x08', &mut display_handle, &file_handle) == false {
                            break;
                        }
                    }
                    _ => {}
                }
            }
            event::Event::Resize(_, _) => {
                display_handle.resize()?;
            }
            _ => {}
        }
        display_handle.print(&cmd, &file_handle)?;
    }

    // post-processing
    terminal::disable_raw_mode()?;
    execute!(
        io::stdout(),
        cursor::Show,
        terminal::LeaveAlternateScreen,
    )?;

    Ok(())
}
