#[derive(Debug)]
pub struct Cli {
    urls: Vec<String>,
    output_format: String,
}

impl Cli {
    pub fn new(urls: Vec<String>, output_format: String) -> Self {
        Self { urls, output_format }
    }
}