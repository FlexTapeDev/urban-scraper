use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://example.com";
    let response = get(url)?.text()?;
    let document = Html::parse_document(&response);
    let selector = Selector::parse("h1").unwrap();
    for element in document.select(&selector) {
        let title = element.text().collect::<Vec<_>>().concat();
        println!("Title: {}", title);
    }

    Ok(())
}
