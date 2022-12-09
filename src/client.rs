pub mod cli
{
    use termion::{clear, terminal_size};

    pub struct Terminal {
        pub width: usize,
        pub height: usize,
    }
    
    impl Terminal {
        pub fn new() -> Terminal {
            let size = terminal_size().expect("ERROR GETTING SIZE");
            Terminal {
                width: usize::from(size.0),
                height: usize::from(size.1),
            }
        }
    }

    pub fn clear() {
        println!("{}", clear::All);
    }

    pub fn goto(x: u16, y: u16) {
        println!("{}", termion::cursor::Goto(x, y));
    }
}