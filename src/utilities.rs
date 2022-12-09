pub mod utils {
    pub mod interface {

        use crate::cli::Terminal;
        use termion::color;

        pub fn create_bar(
            interface: &Terminal,
            text: &String,
            bg_col: &dyn color::Color,
            fg_col: &dyn color::Color,
        ) -> String {
            let mut padding = String::new();
            if interface.width > text.len() {
                padding.push_str(&String::from(" ").repeat(interface.width - text.len()));
            }

            String::from(format!(
                "{}{}{}{}{}{}",
                color::Bg(bg_col),
                color::Fg(fg_col),
                text,
                padding,
                color::Fg(color::Reset),
                color::Bg(color::Reset)
            ))
        }

        pub fn centre_with(padding: &str, text: &str, interface: &Terminal) -> String {
            let width = interface.width;
            let text_len = text.len();

            if text_len > width {
                return String::from(text);
            }

            assert!(padding.len() == 1);

            let padding_size = (width - text_len) / 2;
            let padding_str = String::from(padding).repeat(padding_size);

            let mut finished = String::new();
            finished.push_str(&padding_str);
            finished.push_str(text);
            finished.push_str(&padding_str);
            if finished.len() != interface.width {
                finished.push_str(padding);
            }
            return finished;
        }
    }
}
