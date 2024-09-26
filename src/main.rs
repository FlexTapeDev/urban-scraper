use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::error::Error;
use std::io;

mod crates;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://www.rust-lang.org/"; // currently static url will change later
    let response = get(url)?.text()?; // get response from site that is being scraped
    let document = Html::parse_document(&response); // parse the data to be able to use it
    let selector = Selector::parse("h1, p").unwrap(); // which tags should be scraped

    let mut results = Vec::new(); // initilizing a vector to put tag and text in

    for element in document.select(&selector) { // for loop searching for tags in the elements
        let tag = element.value().name().to_string();
        let text = element.text().collect::<Vec<_>>().concat(); // takes the different sets of words and puts them together to a string
        
        results.push(crates::json_download::ResultItem { tag, text }); // pushes the data into the public struct in save_to_json
    }

    crates::json_download::save_to_json(&results, "results.json")?; // saves the scraped data to results.json

    println!("Results saved at results.json");

    println!("Press Enter to continue... ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}


/* 
TODO
- Make it go on other links and scrape them too
- Make it parse the text in the json nicer
- Add more features
*/