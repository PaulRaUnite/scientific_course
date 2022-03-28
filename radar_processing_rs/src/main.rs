use std::fs::File;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use anyhow::Result;
use clap::Parser;


#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
struct Measurement {
    ch1: i64,
    ch2: i64,
}

fn read_data_file(path: &Path) -> Result<Vec<Measurement>> {
    let mut rdr = csv::Reader::from_reader(File::open(path)?);
    rdr.deserialize::<Measurement>().map(|r|r.map_err(Into::into)).collect()
}

fn split_channels(data: Vec<Measurement>) -> (Vec<i64>, Vec<i64>) {
    data.into_iter().map(|m|(m.ch1, m.ch2)).unzip()

}

#[derive(Parser)]
struct Cli {
    dir: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let (ch1, ch2) = split_channels(read_data_file(&cli.dir.join("donnees_0.csv"))?);

    println!("{:?}", ch1);
    println!("{:?}", ch2);

    Ok(())
}

