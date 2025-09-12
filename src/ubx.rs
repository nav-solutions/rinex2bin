use log::{debug, error, info};
use rinex::prelude::Rinex;

use crate::cli::Cli;

use std::io::{BufWriter, Read, Write};

use crate::Error;

pub fn stream<'a, W: Write>(
    rinex: &'a Rinex,
    cli: &Cli,
    writer: &mut BufWriter<W>,
) -> Result<(), Error> {
    const BUF_SIZE: usize = 4096;

    let mut buf = [0; BUF_SIZE];

    let version_major = rinex.header.version.major;

    let mut streamer = rinex.rnx2ubx();

    if cli.skip_header() {}

    loop {
        match streamer.read(&mut buf) {
            Ok(0) => {
                debug!("all frames consumed");
                return Ok(());
            },
            Err(e) => {
                error!("streaming error: {}", e);
            },
            Ok(_) => {
                match writer.write(&buf) {
                    Ok(_) => {},
                    Err(e) => {
                        error!("streaming error: {}", e);
                    },
                }

                buf = [0; BUF_SIZE];
            },
        }
    }

    Ok(())
}
