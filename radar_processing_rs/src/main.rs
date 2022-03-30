extern crate core;

use std::fs::File;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use anyhow::Result;
use clap::Parser;
use scilib::math::complex::Complex;
use poloto::prelude::*;
use std::io::Write;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
struct Measurement {
    ch1: i64,
    ch2: i64,
}

fn read_data_file(path: &Path) -> Result<Vec<Measurement>> {
    let mut rdr = csv::Reader::from_reader(File::open(path)?);
    rdr.deserialize::<Measurement>().map(|r| r.map_err(Into::into)).collect()
}

fn split_channels(data: Vec<Measurement>) -> (Vec<i64>, Vec<i64>) {
    data.into_iter().map(|m| (m.ch1, m.ch2)).unzip()
}

fn process(dir: &Path, prefix: &str) -> Result<(Vec<f64>, Vec<f64>)> {
    let (ch1, ch2): (Vec<Vec<i64>>, Vec<Vec<i64>>) = (0..16).into_iter().map(|i| -> Result<_> {
        Ok(split_channels(read_data_file(&dir.join(format!("{prefix}_{i}.csv")))?))
    }).collect::<Result<Vec<_>>>()?.into_iter().unzip();
    Ok((process_channel(ch1), process_channel(ch2)))
}

fn process_channel(data: Vec<Vec<i64>>) -> Vec<f64> {
    let averaged = average(data);
    let comp = to_complex(averaged);
    scilib::signal::fft(&comp).into_iter().map(|c| c.modulus()).take(1601).map(|x| 20.0 * (x/25.0).log10()).collect()
}

fn to_complex<T: Into<f64>>(data: impl IntoIterator<Item=T>) -> Vec<Complex> {
    data.into_iter().map(|p: T| Complex { re: p.into(), im: 0.0 }).collect()
}

fn average(data: Vec<Vec<i64>>) -> Vec<f64> {
    let mut result = vec![0; 3202];
    for ch in data {
        for (i, v) in ch.into_iter().enumerate() {
            result[i] += v;
        }
    }
    result.into_iter().map(|v| v as f64).collect()
}

fn subtract(left: &[f64], right: &[f64]) -> Vec<f64> {
    left.iter().zip(right).map(|(a, b)| a - b).collect()
}

fn plot_channels(data: &(Vec<f64>, Vec<f64>), output: &str) -> Result<()> {
    const LIMIT: usize = 100;
    let (ch1, ch2) = data;

    const SPEED_OF_LIGHT: f64 = 299792458.0; // [m / s]
    const SLOPE: f64 = 1950037684072.2034; // [Hz / s]
    const FS: f64 = (6250000 / 2) as f64;
    const HALF: usize = 1601;
    let frequencies = poloto::range_iter([0.0, FS], HALF);
    let distance: Vec<f64> = frequencies.map(|x: f64| (SPEED_OF_LIGHT * x) / (2.0 * SLOPE)).take(LIMIT).collect();

    use poloto::build::line;
    let l1 = line("ch1", distance.iter().copied().zip(ch1.iter().copied()));
    let l2 = line("ch2", distance.iter().copied().zip(ch2.iter().copied()));
    let m = poloto::build::origin();
    let data = plots!(l1, l2, m);

    let p = simple_fmt!(data, "distances", "x", "y");

    let mut file = File::create(format!("{output}.svg"))?;
    write!(file, "{}", poloto::disp(|w| p.simple_theme(w)))?;
    Ok(())
}

#[derive(Parser)]
struct Cli {
    dir: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let with_object = process(&cli.dir, "donnees")?;
    let without_object = process(&cli.dir, "donnees_vide")?;
    plot_channels(&with_object, "with_object")?;
    plot_channels(&without_object, "without_object")?;

    let (with_ch1, with_ch2) = with_object;
    let (without_ch1, without_ch2) = without_object;
    let subtracted = (subtract(&with_ch1, &without_ch1), subtract(&with_ch2, &without_ch2));
    plot_channels(&subtracted, "subtraction")?;

    Ok(())
}

