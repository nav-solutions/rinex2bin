use binex::prelude::Meta;
use clap::{Arg, ArgAction, ArgMatches, ColorChoice, Command};
use std::path::{Path, PathBuf};

pub enum Proto {
    #[cfg(feature = "gps")]
    GPS,
    #[cfg(feature = "qzss")]
    QZSS,
    #[cfg(feature = "rtcm")]
    RTCM,
    #[cfg(feature = "ubx")]
    UBX,
    #[cfg(feature = "binex")]
    BINEX,
}

pub struct Cli {
    /// arguments passed by user
    pub matches: ArgMatches,
}

impl Cli {
    pub fn new() -> Self {
        Self {
            matches: {
                Command::new("rinex2bin")
                    .author("Guillaume W. Bres <guillaume.bressaix@gmail.com>")
                    .version(env!("CARGO_PKG_VERSION"))
                    .about("RINEX to Binary streamer")
                    .arg_required_else_help(true)
                    .color(ColorChoice::Always)
                    .next_help_heading("RINEX (Input)")
                    .arg(
                        Arg::new("filepath")
                            .short('f')
                            .long("file")
                            .help("Input RINEX file (supports: Clock, Meteo, Navigation, Observations, CRINEX).
Accepts gzip compressed file, as long as they are terminated with '.gz'.
Use as many times as needed.")
                            .value_name("FILE")
                            .action(ArgAction::Append)
                            .required(true),
                    )
                    .next_help_heading("Serialization: applies to any proto")
                    .arg(
                        Arg::new("skip-header")
                            .long("skip-header")
                            .action(ArgAction::SetTrue)
                            .help("Do not serialize the RINEX Header, jump to data serie.
This has no effect to raw protocols like GPS, QZSS..")
                    )
                    .arg(
                        Arg::new("gzip")
                            .long("gzip")
                            .action(ArgAction::SetTrue)
                            .help("Gzip compress the output stream (whatever its proto).")
                    )
                    .next_help_heading("RTCM (protocol)")
                    .arg(
                        Arg::new("rtcm")
                            .long("rtcm")
                            .action(ArgAction::SetTrue)
                            .required_unless_present_any(&["ubx", "binex", "gps", "qzss"])
                            .help("Select RTCM protocol forging.
You may forge RTCM to binary file, or serve it over --udp/--tcp, which gives a so called ntrip server."))
                    .next_help_heading("UBX (protocol)")
                    .arg(
                        Arg::new("ubx")
                            .long("ubx")
                            .action(ArgAction::SetTrue)
                            .required_unless_present_any(&["rtcm", "binex", "gps", "qzss"])
                            .help("Select UBX protocol forging.
You may forge UBX to binary file, or serve it over --udp/--tcp."))
                    .next_help_heading("GPS (protocol)")
                    .arg(
                        Arg::new("gps")
                            .long("gps")
                            .action(ArgAction::SetTrue)
                            .required_unless_present_any(&["rtcm", "binex", "ubx", "qzss"])
                            .help("Select GPS protocol forging.
You may forge GPS to binary file, or serve it over --udp/--tcp."))
                    .arg(
                        Arg::new("qzss")
                            .long("qzss")
                            .action(ArgAction::SetTrue)
                            .required_unless_present_any(&["rtcm", "binex", "ubx", "gps"])
                            .help("Select QZSS protocol forging.
You may forge QZSS to binary file, or serve it over --udp/--tcp."))
                    .next_help_heading("BINEX (protocol)")
                    .arg(
                        Arg::new("binex")
                            .long("binex")
                            .action(ArgAction::SetTrue)
                            .required_unless_present_any(&["ubx", "rtcm", "gps", "qzss"])
                            .help("Select BINEX protocol forging.
BINEX is the ''Binary'' equivalent to the RINEX format and is fully open-source.")
                    )
                    .arg(
                        Arg::new("little")
                            .short('l')
                            .long("little")
                            .action(ArgAction::SetTrue)
                            .help("Encoded stream uses Little endianness. Big endiannes is the default")
                    )
                    .arg(
                        Arg::new("crc")
                            .short('c')
                            .long("crc")
                            .action(ArgAction::SetTrue)
                            .help("Encoded stream uses enhanced CRC technique (for very robust messaging).")
                        )
                    .arg(
                        Arg::new("reversed")
                            .short('r')
                            .long("rev")
                            .action(ArgAction::SetTrue)
                            .help("Forge a Reversed BINEX Stream.")
                    )
                    .next_help_heading("Output Interface")
                    .arg(
                        Arg::new("output")
                            .short('o')
                            .long("output")
                            .value_name("FILE")
                            .action(ArgAction::Set)
                            .conflicts_with("stream")
                            .required(false)
                            .help("Define output BIN file name. Otherwise, BIN file name is guessed fron RINEX content.")
                    )
                    .arg(
                        Arg::new("short")
                            .short('s')
                            .long("short")
                            .action(ArgAction::SetTrue)
                            .help("Prefer V2 (short) file name when auto guessing the BIN file name.")
                    )
                    .next_help_heading("Streaming")
                    .arg(
                        Arg::new("streaming")
                            .long("stream")
                            .action(ArgAction::Set)
                            .conflicts_with("output")
                            .value_name("writable interface")
                            .required(false)
                            .help("Stream on custom I/O interface, instead of forging a BIN file.")
                    )
                    .get_matches()
            },
        }
    }

    pub fn input_path(&self) -> PathBuf {
        Path::new(self.matches.get_one::<String>("filepath").unwrap()).to_path_buf()
    }

    pub fn custom_bin_name(&self) -> Option<&String> {
        self.matches.get_one::<String>("output")
    }

    pub fn short_bin_name(&self) -> bool {
        self.matches.get_flag("short")
    }

    pub fn streaming(&self) -> Option<PathBuf> {
        let stream = self.matches.get_one::<String>("streaming")?;
        Some(Path::new(stream).to_path_buf())
    }

    pub fn skip_header(&self) -> bool {
        self.matches.get_flag("skip-header")
    }

    pub fn gzip(&self) -> bool {
        self.matches.get_flag("gzip")
    }

    pub fn binex_meta(&self) -> Meta {
        Meta {
            reversed: self.matches.get_flag("reversed"),
            enhanced_crc: self.matches.get_flag("crc"),
            big_endian: !self.matches.get_flag("little"),
        }
    }

    pub fn proto(&self) -> Proto {
        #[cfg(not(feature = "gps"))]
        if self.matches.get_flag("gps") {
            panic!("--gps required GPS compilation option");
        }
        #[cfg(feature = "gps")]
        if self.matches.get_flag("gps") {
            return Proto::GPS;
        }

        #[cfg(not(feature = "qzss"))]
        if self.matches.get_flag("qzss") {
            panic!("--qzss required QZSS compilation option");
        }
        #[cfg(feature = "qzss")]
        if self.matches.get_flag("qzss") {
            return Proto::QZSS;
        }

        #[cfg(not(feature = "binex"))]
        if self.matches.get_flag("binex") {
            panic!("--binex required BINEX compilation option");
        }
        #[cfg(feature = "binex")]
        if self.matches.get_flag("binex") {
            return Proto::BINEX;
        }

        #[cfg(not(feature = "rtcm"))]
        if self.matches.get_flag("rtcm") {
            panic!("--rtcm required RTCM compilation option");
        }
        #[cfg(feature = "rtcm")]
        if self.matches.get_flag("rtcm") {
            return Proto::RTCM;
        }

        #[cfg(not(feature = "ubx"))]
        if self.matches.get_flag("ubx") {
            panic!("--ubx required UBX compilation option");
        }
        #[cfg(feature = "ubx")]
        if self.matches.get_flag("ubx") {
            return Proto::UBX;
        }

        panic!("must select at least one protocol!");
    }
}
