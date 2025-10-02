mod cmd;

use {
    std:: {
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

fn main() -> io::Result<()> {
    let mut cmd = cmd::Cmd::new();
    terminal::enable_raw_mode()?;
    execute!(
        io::stdout(),
        cursor::Hide,
        terminal::EnterAlternateScreen,
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All),
    )?;

    loop {
        if event::poll(time::Duration::from_millis(100))? {
            match event::read()? {
                event::Event::Key(key) => {
                    match key.code {
                        event::KeyCode::Char(kc) => {
                            if cmd.key(kc) == false {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    terminal::disable_raw_mode()?;
    execute!(
        io::stdout(),
        cursor::Show,
        terminal::LeaveAlternateScreen,
    )?;
    Ok(())
}
