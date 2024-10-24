use clap::{Arg, Command};

#[derive(Debug)]
struct Cli {
    urls: Vec<String>,
    output_format: String,
}

fn main() {
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
    let cli = Cli { urls, output_format };



}