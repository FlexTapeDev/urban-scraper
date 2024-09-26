use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::error::Error;
use std::io;

mod crates;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://www.rust-lang.org/";
    let response = get(url)?.text()?;
    let document = Html::parse_document(&response);
    let selector = Selector::parse("h1, p").unwrap();

    let mut results = Vec::new();

    for element in document.select(&selector) {
        let tag = element.value().name().to_string();
        let text = element.text().collect::<Vec<_>>().concat();
        
        results.push(crates::json_download::ResultItem { tag, text });
    }

    crates::json_download::save_to_json(&results, "results.json")?;

    println!("Results saved at results.json");

    println!("Press Enter to continue... ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}
