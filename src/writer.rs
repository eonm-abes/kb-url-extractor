use tokio::fs::File;
use tokio::io::{AsyncWriteExt, Stdout};

#[derive(Debug)]
pub enum DataWriter {
    Stdout(Stdout),
    File(File),
}

impl DataWriter {
    pub fn file(file: File) -> Self {
        DataWriter::File(file)
    }

    pub fn stdout(stdout: Stdout) -> Self {
        DataWriter::Stdout(stdout)
    }

    pub async fn write(&mut self, bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            DataWriter::Stdout(ref mut stdout) => stdout.write_all(bytes).await?,
            DataWriter::File(ref mut file) => file.write_all(bytes).await?,
        }

        Ok(())
    }
}
