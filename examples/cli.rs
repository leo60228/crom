use crom::*;

#[async_std::main]
async fn main() -> surf::Result<()> {
    let client = Client::new();
    let pages = client
        .search(
            "5000",
            Some(vec!["http://scp-wiki.wikidot.com".parse().unwrap()]),
        )
        .await?;

    for page in pages {
        println!("{:?}", page);
    }

    Ok(())
}
