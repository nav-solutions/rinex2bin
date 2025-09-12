use log::{debug, info};
use rinex::prelude::Rinex;

use crate::cli::Cli;

use std::io::{BufWriter, Write};

use crate::Error;

pub fn stream<'a, W: Write>(
    rinex: &'a Rinex,
    cli: &Cli,
    w: &mut BufWriter<W>,
) -> Result<(), Error> {
    const BUF_SIZE: usize = 4096;

    let mut buf = [0; BUF_SIZE];

    let version_major = rinex.header.version.major;

    let mut streamer = rinex
        .rnx2rtcm()
        .unwrap_or_else(|| panic!("Failed to deploy RTCM streamer"));

    if cli.skip_header() {}

    loop {
        match streamer.next() {
            Some(msg) => {
                debug!("Streaming: {:?}", msg);

                msg.encode(&mut buf, BUF_SIZE)
                    .unwrap_or_else(|e| panic!("RTCM encoding error: {:?}", e));

                w.write(&buf).unwrap_or_else(|e| panic!("I/O error: {}", e));

                buf = [0; BUF_SIZE];
            },
            None => {
                break;
            },
        }
    }

    debug!("all frames consumed");
    Ok(())
}
