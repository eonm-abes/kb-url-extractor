#![feature(drain_filter)]

mod cli;
use cli::Cli;

mod config;
use config::DataProviders;

mod link_extractor;
use link_extractor::LinkExtractor;

mod link_filter;
use link_filter::*;

mod writer;
use url::Url;
use writer::DataWriter;

use std::path::PathBuf;
use std::sync::Arc;

use clap::Parser;

use futures::StreamExt;

use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};
use tokio::sync::Mutex;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let nb_workers = cli.worker;
    let config = read_config(cli.config).await?;

    let writer: DataWriter = match cli.output {
        Some(output) => DataWriter::file(File::create(output).await?),
        None => DataWriter::stdout(io::stdout()),
    };

    let writer = Arc::new(Mutex::new(writer));

    let fetches = futures::stream::iter(config.data_provider.into_iter().map(|(url, dp)| {
        let writer = Arc::clone(&writer);
        async move {
            if let Ok(resp) = reqwest::get(url.clone()).await {
                if let Ok(response) = resp.text().await {
                    let html = scraper::Html::parse_document(&response);

                    let mut links: Vec<Url> =
                        LinkExtractor::new(url).extract(&html).into_iter().collect();

                    let whithe_list = ControlList::new(dp.white_list);
                    let black_list = ControlList::new(dp.black_list);

                    let link_filter = LinkFilter::new(whithe_list.clone(), black_list.clone());
                    link_filter.filter::<Url, MatchStartUrlPath>(&mut links);

                    let link_filter = LinkFilter::new(
                        ControlList::new(dp.allowed_extensions),
                        ControlList::default(),
                    );
                    link_filter.filter::<Url, MatchEndUrlPath>(&mut links);

                    for link in links {
                        let link = format!("{}\n", link);

                        let mut w = writer.lock().await;
                        w.write(link.as_bytes())
                            .await
                            .expect("Failed to write data");
                    }
                }
            }
        }
    }))
    .buffer_unordered(nb_workers)
    .collect::<Vec<()>>();

    fetches.await;

    Ok(())
}

pub async fn read_config(path: PathBuf) -> Result<DataProviders, Box<dyn std::error::Error>> {
    let mut config_file = File::open(path).await?;
    let mut config_content = String::new();
    config_file.read_to_string(&mut config_content).await?;

    Ok(toml::from_str(&config_content)?)
}
