mod config;
use config::DataProviders;
use futures::StreamExt;
use tokio::io::{self, AsyncWriteExt};

mod link_extractor;
use link_extractor::LinkExtractor;

use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config_file = File::open("data-providers.toml").await?;
    let mut config_content = String::new();

    config_file.read_to_string(&mut config_content).await?;

    let config: DataProviders = toml::from_str(&config_content)?;

    let fetches = futures::stream::iter(config.data_provider.into_iter().map(
        |(url, dp)| async move {
            match reqwest::get(url.clone()).await {
                Ok(resp) => match resp.text().await {
                    Ok(response) => {
                        let html = scraper::Html::parse_document(&response);

                        let links = LinkExtractor::new(
                            &url.to_string(),
                            dp.white_list.iter().map(AsRef::as_ref).collect(),
                            dp.black_list.iter().map(AsRef::as_ref).collect(),
                        )
                        .extract(&html);

                        for link in links {
                            let link = format!("{}\n", link);
                            let mut stdout = io::stdout();

                            let _ = stdout.write_all(link.as_bytes()).await;
                        }
                    }
                    Err(_) => (),
                },
                Err(_) => (),
            }
        },
    ))
    .buffer_unordered(8)
    .collect::<Vec<()>>();

    fetches.await;

    Ok(())
}
