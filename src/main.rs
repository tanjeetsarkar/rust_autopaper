use std::io::Write;

fn main() {
    match get_newspaper() {
        Ok(_) => {}
        Err(e) => println!("error: {}", e),
    }
}

#[tokio::main]
async fn get_newspaper() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(
        "https://epaper.telegraphindia.com/epaperimages////22032023////22032023-md-hr-5ll.png",
    )
    .await?;

    if resp.status().is_success() {
        if let Some(content_type) = resp.headers().get(reqwest::header::CONTENT_TYPE) {
            if content_type.to_str()?.contains("text/html") {
                println!("End of Page here")
            } else {
                println!("Content type {:#?}", content_type);
                let mut file = std::fs::File::create("image.png").unwrap();
                let content = resp.bytes().await?;
                file.write_all(&content)?;
            }
        }
    } else {
        println!("error in response {}", resp.status())
    }

    Ok(())
}
