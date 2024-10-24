use scraper::{Html, Selector};
use std::error::Error;

// Define an enum for different scraping types
pub enum ScrapeType {
    Title,
    ProductName,
    Price,
    Custom(String), // Allows users to pass custom CSS Selectors
}

/// Function to extract the title from the html
pub fn scrape(html_content: &str, scrape_type: ScrapeType) -> Result<String, Box<dyn Error>> {
    // Parse the html content
    let document = Html::parse_document(html_content);

    // Create a selector to fin the title tag
    let selector = match scrape_type {
        ScrapeType::Title => Selector::parse("title").unwrap(),
        ScrapeType::ProductName => Selector::parse(".product-name").unwrap(),
        ScrapeType::Price => Selector::parse(".price").unwrap(),
        ScrapeType::Custom(custom_selector) => Selector::parse(&custom_selector).unwrap(),
    };

    // Find the first title element and extract its text
    let element = document
        .select(&selector)
        .next()
        .ok_or("No title Found")?
        .text()
        .collect::<Vec<_>>()
        .join("");

    Ok(element)
}
