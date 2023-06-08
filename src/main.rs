mod cli;
use cli::Cli;

mod config;
use config::DataProviders;

mod link_extractor;
use link_extractor::LinkExtractor;

mod writer;
use writer::DataWriter;


use std::path::PathBuf;
use std::sync::Arc;

use clap::Parser;

use futures::StreamExt;

use tokio::sync::Mutex;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};


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

                            let mut w = writer.lock().await;
                            w.write(link.as_bytes()).await.expect("Failed to write data");
                        }
                    }
                    Err(_) => (),
                },
                Err(_) => (),
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
