# Project: Web Scr4per with Concurrency
## Project Overview:
Create a CLI tool in Rust that get data from multiple websites concurrently. 

The scraper will gather data like product information (title, price, and availability) from various e-commerce sites or gather articles from blog pages. 

The tool should handle multiple sites in parallel using Rust's asynchronous programming capabilities, and it will save the data into a local file (CSV or JSON) or database like SQLite.

## Core Features:
1. CLI Interface:
    - Accept user input for websites to scrape, the type of data to gather, and the number of sites.
    - Allow options like saving results in CSV or JSON format.
    - Display the scraping progress.

2. Asynchronous Scraping:
   - Use Rust's async and tokio or async-std for scraping multiple websites concurrently.
   - Implement concurrency limits to avoid overwhelming the target websites.

3. Data Parsing:
   - Extract specific data from HTML (e.g., product titles, prices, availability).
   - Use libraries like reqwest for HTTP requests and scraper or select for parsing HTML.

4. Error Handling and Retries:
   - Implement robust error handling for failed requests, with retry logic for timeouts or bad responses.

5. Saving Data:
   - Save scraped data to a CSV or JSON file.
   - Optionally store results in a SQLite database (build on your prior experience).

--------

## Instructions to run
1. Clone the repository
2. Run the following command to run the project:
```bash
cargo run -- --urls https://example.com https://rust-lang.org --format json
```
3. The above command will scrape the data from the given URLs and save the data in JSON format.
4. You can also use the `--format csv` option to save the data in CSV format.
5. Use the `--help` option to see all available options.
6. Enjoy scraping data concurrently with Rust!