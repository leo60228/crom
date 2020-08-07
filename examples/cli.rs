use crom::*;

#[async_std::main]
async fn main() -> surf::Result<()> {
    let client = Client::new();
    let pages = client.search("5000").await?;

    for page in pages {
        match page {
            Page {
                url,
                wikidot_info:
                    Some(WikidotInfo {
                        title: Some(title),
                        rating: Some(rating),
                        ..
                    }),
                ..
            } => println!("{} ({}) - {}", title, rating, url),
            Page {
                url,
                wikidot_info:
                    Some(WikidotInfo {
                        title: Some(title), ..
                    }),
                ..
            } => println!("{} - {}", title, url),
            Page {
                url,
                wikidot_info:
                    Some(WikidotInfo {
                        rating: Some(rating),
                        ..
                    }),
                ..
            } => println!("{} ({})", url, rating),
            Page { url, .. } => println!("{}", url),
        }
    }

    Ok(())
}
