pub mod file_handler
{

    pub struct Document {
        pub title: String,
        pub content: Vec<String>,
        pub content_full: String,
        pub length: usize,
    }
    
    impl Document {
        pub fn new(path: &str) -> Document {
            let content_full = String::from_utf8(std::fs::read(path).expect("ERROR READING FILE"))
                .expect("ERROR PARSING FILE");
            let content_str: Vec<&str> = content_full.split("\n").collect();
            let mut content: Vec<String> = Vec::new();
            for ele in content_str {
                content.push(String::from(ele));
            }
            let lines = content.len();
            Document {
                title: String::from(path),
                content: content,
                content_full: content_full,
                length: lines,
            }
        }
    }
}
