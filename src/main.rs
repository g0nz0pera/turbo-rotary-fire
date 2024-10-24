mod cli;
mod scraper;
mod fetch;

use clap::{Arg, Command};
use cli::Cli;

#[tokio::main]
async fn main() {
    let matches = Command::new("Multi Scrapper")
        .author("g0ng0n")
        .version("0.1")
        .about("Scraping multiples sites and stores data")
        .arg(
            Arg::new("urls")
                .short('u')
                .long("urls")
                .value_name("URLS")
                .help("A list of URLS to scrape")
                .num_args(1..)
                .required(true),
        ).arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .value_name("FORMAT")
                .help("The output format: csv or Json")
                .required(true)
                .value_parser(["json", "csv"]),
    ).get_matches();

    // Get the URLs and output format
    let urls: Vec<String> = matches.get_many::<String>("urls")
        .expect("A List of URLS is required.")
        .cloned()
        .collect();

    let output_format = matches.get_one::<String>("format")
        .expect("An output format is required. (CSV or JSON)")
        .to_string();

    // Printing the input values for confirmation
    println!("Scraping the following URLs: {:?}", urls);
    println!("Data will be saved in {} format", output_format);

    // You can store these in a struct for later use
    //let cli = Cli::new(urls, output_format);

    for url in &urls {
        match fetch::fetch_url(url).await {
            Ok(content) => {
            println!("Successfully fetched content from {}", url);

                match scraper::scrape_title(&content){
                    Ok(title) => {
                        println!("Title of {}: {}", url, title);
                    }
                    Err(err) => {
                        eprintln!("Failed to scrape title from {}: {}", url, err);
                    }
                }
            }
            Err(err) => {
                eprintln!("Failed to fetch content from {}: {}", url, err);
            }
        }
    }


}