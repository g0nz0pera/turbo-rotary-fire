use scraper::{Html, Selector};
use std::error::Error;

/// Function to extract the title from the html
pub fn scrape_title(html_content: &str) -> Result<String, Box<dyn Error>> {

    // Parse the html content
    let document = Html::parse_document(html_content);

    // Create a selector to fin the title tag
    let selector = Selector::parse("title").unwrap();

    // Find the first title element and extract its text
    let title = document.select(&selector)
        .next()
        .ok_or("No title Found")?
        .text()
        .collect::<Vec<_>>()
        .join("");

    Ok(title)
}