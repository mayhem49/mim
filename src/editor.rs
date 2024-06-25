use crossterm::{
    event::{read, Event, KeyCode::Char, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }

    pub fn run(&mut self) {
        if let Err(err) = self.repl() {
            panic!("{err:?}");
        }
        println!("end\r\n");
    }

    pub fn repl(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        loop {
            if let Event::Key(
                ref key_event @ KeyEvent {
                    code, modifiers, ..
                },
            ) = read()?
            {
                println!("key_event: {key_event:?}\r");
                match code {
                    //Char('r') => print!("\x1b[J"),
                    //Char('c') => print!("\x1b[31red\n"),
                    //Char('0') => print!("\x1b[0J"),
                    //Char('1') => print!("\x1b[1J"),
                    //Char('2') => print!("\x1b[2J"),
                    Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.should_quit = true;
                    }
                    _ => (),
                }
            }
            if self.should_quit == true {
                break;
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}
