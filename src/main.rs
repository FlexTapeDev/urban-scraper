use reqwest::blocking::get;
use reqwest::StatusCode;
use scraper::{Html, Selector};
use std::error::Error;
use std::io::{self, Write};

mod crates;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Link to scrape: ");
    let mut url = String::new(); // currently static url will change later
    io::stdout().flush()?;
    io::stdin().read_line(&mut url)?;
    let mut url = url.trim().to_string();

    if !url.starts_with("https://") && !url.starts_with("http://") {
        url = format!("https://{}", url);
        println!("Added https://");
    }

    let response = get(url)?.text()?; // get response from site that is being scraped
    let document = Html::parse_document(&response); // parse the data to be able to use it
    let selector = Selector::parse("h1, p").unwrap(); 
    let mut results = Vec::new(); // initilizing a vector to put tag and text in

    for element in document.select(&selector) { // for loop searching for tags in the elements
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
