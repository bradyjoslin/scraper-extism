use extism_pdk::*;
use s::{Html, Selector};
use scraper as s;
use serde::Deserialize;

#[derive(FromBytes, Deserialize, PartialEq, Debug)]
#[encoding(Json)]
struct Scraper {
    html: String,
    selector: String,
}

#[plugin_fn]
pub fn scraper(input: Scraper) -> FnResult<String> {
    let document = Html::parse_document(input.html.as_str());

    let selector = Selector::parse(&input.selector)
        .map_err(|e| WithReturnCode::new(anyhow::anyhow!("Selector error {}", e), 1))?;

    // Find the elements matching the selector
    let mut text_content = String::new();
    for element in document.select(&selector) {
        // Extract the text content of the element and concatenate it
        text_content.push_str(&element.text().collect::<Vec<_>>().concat());
    }

    Ok(text_content)
}
