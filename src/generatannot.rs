use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::error::Error;

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-8-18
*/

pub async fn getrsid(rsidsnp: &str) -> Result<String, Box<dyn Error>> {
    let formatstring = format!("{}/{}", "https://www.ncbi.nlm.nih.gov/snp/", rsidsnp);
    let response = get(formatstring).expect("string not found");
    let document = Html::parse_document(&response.text().expect("message not present"));
    let snpselect = Selector::parse(".sect_heading").expect("method failed");
    for element in document.select(&snpselect) {
        let vector_1 = element
            .text()
            .collect::<Vec<_>>()
            .join("-")
            .replace(" ", "")
            .replace("\n", "")
            .replace("Genomecontext", "")
            .replace("Selectflanklength", "")
            .replace(":", "");
        println!("{:?}", vector_1);
    }
    Ok("The string has been written".to_string())
}
