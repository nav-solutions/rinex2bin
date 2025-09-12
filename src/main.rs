mod cli;
use cli::{Cli, Proto};

extern crate log;

use env_logger::{Builder, Target};
use log::debug;

use rinex::prelude::{FormattingError, ParsingError, Rinex};

use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};

use flate2::{write::GzEncoder, Compression};
use thiserror::Error;

#[cfg(feature = "gps")]
mod gps;

#[cfg(feature = "qzss")]
mod qzss;

#[cfg(feature = "rtcm")]
mod rtcm;

#[cfg(feature = "binex")]
mod binex;

#[cfg(feature = "binex")]
use binex::stream as binex_stream;

#[cfg(feature = "ubx")]
mod ubx;

#[cfg(feature = "ubx")]
use ubx::stream as ubx_stream;

#[derive(Debug, Error)]
enum Error {
    #[error("parsing error")]
    ParsingError(#[from] ParsingError),
    #[error("formatting error")]
    FormattingError(#[from] FormattingError),
}

fn stream<'a, W: Write>(
    rinex: &'a Rinex,
    cli: &Cli,
    writer: &mut BufWriter<W>,
) -> Result<(), Error> {
    match cli.proto() {
        #[cfg(feature = "gps")]
        Proto::GPS => gps_stream(rinex, cli, writer),
        #[cfg(feature = "qzss")]
        Proto::QZSS => qzss_stream(rinex, cli, writer),
        #[cfg(feature = "rtcm")]
        Proto::RTCM => rtcm_stream(rinex, cli, writer),
        #[cfg(feature = "binex")]
        Proto::BINEX => binex_stream(rinex, cli, writer),
        #[cfg(feature = "ubx")]
        Proto::UBX => ubx_stream(rinex, cli, writer),
    }
}

fn main() -> Result<(), Error> {
    let mut builder = Builder::from_default_env();

    builder
        .target(Target::Stdout)
        .format_timestamp_secs()
        .format_module_path(false)
        .init();

    let cli = Cli::new();

    let input_path = cli.input_path();
    let input_path_str = input_path.to_string_lossy().to_string();
    let gzip_input = input_path_str.ends_with(".gz");

    let gzip_output = cli.gzip();
    let forced_short_v2 = cli.short_bin_name();

    let rinex = if gzip_input {
        Rinex::from_gzip_file(input_path)
    } else {
        Rinex::from_file(input_path)
    };

    let rinex = rinex.unwrap_or_else(|e| panic!("RINEX parsing error: {}", e));

    let short_v2 = forced_short_v2 || rinex.header.version.major < 3;

    let output_path = if let Some(custom) = cli.custom_bin_name() {
        custom.to_string()
    } else {
        if gzip_output {
            rinex.standard_filename(short_v2, Some("bin.gz"), None)
        } else {
            rinex.standard_filename(short_v2, Some(".bin"), None)
        }
    };

    let fd = if let Some(stream) = cli.streaming() {
        OpenOptions::new()
            .write(true)
            .open(&stream)
            .unwrap_or_else(|e| panic!("Failed to open output stream {}: {}", stream.display(), e))
    } else {
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(&output_path)
            .unwrap_or_else(|e| panic!("Failed to create output file {}: {}", output_path, e))
    };

    if cli.custom_bin_name().is_some() {
        if output_path.ends_with(".gz") {
            let compression = Compression::new(5);
            let mut writer = BufWriter::new(GzEncoder::new(fd, compression));
            stream(&rinex, &cli, &mut writer);
        } else {
            let mut writer = BufWriter::new(fd);
            stream(&rinex, &cli, &mut writer);
        }
    } else {
        let mut writer = BufWriter::new(fd);
        stream(&rinex, &cli, &mut writer);
    }

    Ok(())
}
