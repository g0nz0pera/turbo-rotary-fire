use clap::{Arg, Command};

#[derive(Debug)]
pub struct Cli {
    urls: Vec<String>,
    output_format: String,
    scrape_option: String,
}

impl Cli {
    pub fn new(urls: Vec<String>, output_format: String, scrape_option: String) -> Self {
        Self {
            urls,
            output_format,
            scrape_option,
        }
    }
    pub fn get_urls(&mut self) -> &Vec<String> {
        return &self.urls;
    }

    pub fn get_output_format(&mut self) -> &String {
        return &self.output_format;
    }

    pub fn get_scrape_option(&mut self) -> &String {
        return &self.scrape_option;
    }

    pub fn parse_args() -> Cli {
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
            )
            .arg(
                Arg::new("format")
                    .short('f')
                    .long("format")
                    .value_name("FORMAT")
                    .help("The output format: csv or Json")
                    .required(true)
                    .value_parser(["json", "csv"]),
            )
            .arg(
                Arg::new("scrape-option")
                    .short('s')
                    .long("scrape-option")
                    .value_name("OPTION")
                    .help("What to scrape: title, product-name, price, or a custom CSS selector")
                    .required(false),
            )
            .get_matches();

        // Get the URLs and output format
        let urls: Vec<String> = matches
            .get_many::<String>("urls")
            .expect("A List of URLS is required.")
            .cloned()
            .collect();

        let output_format = matches
            .get_one::<String>("format")
            .expect("An output format is required. (CSV or JSON)")
            .to_string();

        let scrape_option = matches
            .get_one::<String>("scrape-option")
            .expect("A scrape option is required. (title, product-name, price, or a custom CSS selector)")
            .to_string();

        Cli {
            urls,
            output_format,
            scrape_option,
        }
    }
}
