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

    let meta = cli.binex_meta();

    let version_major = rinex.header.version.major;

    let mut streamer = rinex
        .rnx2bin(meta)
        .unwrap_or_else(|| panic!("Failed to deploy BINEX streamer"));

    if let Some(constellation) = rinex.header.constellation {
        streamer.custom_announce = Some(format!(
            "rinex2bin v{} from V{} {} {}",
            env!("CARGO_PKG_VERSION"),
            version_major,
            constellation,
            rinex.header.rinex_type
        ));
    } else {
        streamer.custom_announce = Some(format!(
            "rinex2bin v{} from V{} {}",
            env!("CARGO_PKG_VERSION"),
            version_major,
            rinex.header.rinex_type
        ));
    }

    if cli.skip_header() {
        streamer.skip_header = true;
    }

    loop {
        match streamer.next() {
            Some(msg) => {
                debug!("Streaming: {:?}", msg);
                msg.encode(&mut buf, BUF_SIZE)
                    .unwrap_or_else(|e| panic!("BINEX encoding error: {:?}", e));

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
