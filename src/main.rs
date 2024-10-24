mod cli;
mod fetch;
mod models;
mod scraper;

use cli::Cli;
use scraper::ScrapeType;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut cli = Cli::parse_args();

    // Printing the input values for confirmation
    println!("Scraping the following URLs: {:?}", cli.get_urls());
    println!("Data will be saved in {} format", cli.get_output_format());
    println!("Scrape output format: {}", cli.get_scrape_option());

    let urls = cli.get_urls().clone();
    let scrape_option = cli.get_scrape_option().clone();

    for url in urls.iter() {
        match fetch::fetch_url(url).await {
            Ok(content) => {
                println!("Successfully fetched content from {}", url);

                // Example: Scrape title or product name based on user input
                let scrape_type = if scrape_option.eq("title") {
                    ScrapeType::Title
                } else if scrape_option.eq("product-name") {
                    ScrapeType::ProductName
                } else if scrape_option == "price" {
                    ScrapeType::Price
                } else {
                    ScrapeType::Custom(cli.get_scrape_option().clone())  // Custom selector
                };

                match scraper::scrape(&content, scrape_type) {
                    Ok(data) => {
                        println!("Scraped data from {}: {}", url, data);
                    }
                    Err(err) => {
                        eprintln!("Failed to scrape content from {}: {}", url, err);
                    }
                }
            }
            Err(err) => {
                eprintln!("Failed to fetch content from {}: {}", url, err);
            }
        }
    }
    Ok(())
}