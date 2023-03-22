use chrono::prelude::*;
use regex::Regex;
use std::io::Write;
use std::path::Path;
use std::{env, fs};

const NEWSPAPERS: &[(&str, &str)] = &[
    (
        "telegraph",
        "https://epaper.telegraphindia.com/epaperimages",
    ),
    ("anandabazar", "https://epaper.anandabazar.com/epaperimages"),
];

const PAGES: u8 = 30;

fn main() {
    let arg: Vec<String> = env::args().collect();

    if arg.len() > 2 {
        eprintln!("Usage: rust_autopaper \n or \n rust_autopaper <ddmmYYYY>");
        return;
    }

    let today = Local::now().format("%d%m%Y").to_string();
    let mut date = &today;

    if arg.len() == 2 {
        let date_regex = Regex::new(r"^\d{2}\d{2}\d{4}$").unwrap();
        if !date_regex.is_match(&arg[1]) {
            eprintln!("Invalid date format.\nExpected format: ddmmyyyy");
            return;
        }
        date = &arg[1];
    }

    match get_newspaper(&date) {
        Ok(_) => {}
        Err(e) => println!("error: {}", e),
    }
}

#[tokio::main]
async fn get_newspaper(date: &str) -> Result<(), Box<dyn std::error::Error>> {
    for (key, value) in NEWSPAPERS {
        let dir_path = format!("{}-{}", key, date);
        if let Err(e) = fs::create_dir_all(&dir_path) {
            eprintln!("Error creating directory{} : {}", dir_path, e)
        }
        if Path::new(&dir_path).exists() {
            for page in 1..=PAGES {
                let url = format!("{}////{}////{}-md-hr-{}ll.png", value, date, date, page);

                let resp = reqwest::get(url).await?;

                if resp.status().is_success() {
                    if let Some(content_type) = resp.headers().get(reqwest::header::CONTENT_TYPE) {
                        if content_type.to_str()?.contains("text/html") {
                            println!("End of Pages here");
                            break;
                        } else {
                            println!("downloading {} page {} into: {}", key, page, dir_path);

                            let mut file = std::fs::File::create(format!(
                                "{}/{}-md-hr-{}ll.png",
                                dir_path, date, page
                            ))
                            .unwrap();
                            let content = resp.bytes().await?;
                            file.write_all(&content)?;
                        }
                    }
                } else {
                    println!("error in response {}", resp.status())
                }
            }
        }
    }

    Ok(())
}
