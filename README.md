RINEX2BIN
=========

[![Rust](https://github.com/nav-solutions/rinex2bin/actions/workflows/rust.yml/badge.svg)](https://github.com/nav-solutions/rinex2bin/actions/workflows/rust.yml)
[![Rust](https://github.com/nav-solutions/rinex2bin/actions/workflows/daily.yml/badge.svg)](https://github.com/nav-solutions/rinex2bin/actions/workflows/daily.yml)
[![crates.io](https://img.shields.io/crates/v/rinex2bin.svg)](https://crates.io/crates/rinex2bin)

[![License](https://img.shields.io/badge/license-MPL_2.0-orange?style=for-the-badge&logo=mozilla)](https://github.com/nav-solutions/rinex2bin/blob/main/LICENSE)

`rinex2bin` is a command line utility to serialize your RINEX (Receiver Indepent EXchange) files
into binary formats. We cover many protosol

- BINEX: the RINEX ''binary'' equivalent (open source), on `binex` tool feature.
- GPS: the GPS (US) broadcast protocol, on `gps` tool feature.
- QZSS: the QZSS (Japan) broadcast protocol, on `qzss` tool feature.
- RTCM: ntrip server and RTK protocol, on `rtcm` tool feature.
- UBX: U-Blox receivers protocols, on `ubx` tool feature.

The application can serialize into a binary file (for example a UBX file, or GPS binary file),
or deploy as a TCP/UDP server. When streaming to RTCM protocol and deploying as TCP server,
this application is identical to a so called NTRIP server.

All protocols are feature dependent. By default this application only comes with BINEX and RTCM protocols.

## Download the tool

You can download the latest version from [the release portal](https://github.com/nav-solutions/rinex2bin/releases)

## Install from Cargo

You can directly install the tool from Cargo with internet access,
which supports all protocols:

```bash
cargo install rinex2bin
```

## Build from sources

Download the version you are interested in:

```bash
git clone https://github.com/nav-solutions/rinex2bin
```

And build it using cargo:

```bash
cargo build --all-features -r
```


Build the tool limited to GPS protocol (example):

```bash
cargo build --no-default-features --features gps
```

Deployment
==========

+ You need at least one input file (RINEX file, supported formats: Meteo, Navigation, Clock, Observations, CRINEX).
+ And you need to select a protocol

Example (1): NAV V3 to RTCM file. 

```bash
rinex2bin -f data/NAV/V3/NYA100NOR_S_20241240000_01D_CN.rnx.gz --rtcm
```

Example (2): NAV V3 gzip compressed to RTCM file:

```bash
rinex2bin -f data/NAV/V3/NYA100NOR_S_20241240000_01D_CN.rnx.gz --rtcm
```

Example (3): PPP context to BINEX file:

```bash
rinex2bin \
    -f data/NAV/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    -f data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    --binex
```

Example (4): PPP to NTRIP server (RTK compatible)

```bash
rinex2bin \
    -f data/NAV/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    -f data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    --binex \
    --tcp 127.0.0.1 \
    --port 1234
```

Read more on this on the TCP port chapter.

Example (5): RINEX to raw GPS frames.
In this case, we will stream navigation messages only, scoped to GPS satellites only.

```bash
rinex2bin \
    -f data/NAV/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    --gps
```


Example (6): RINEX to raw GPS frames over TCP:

```bash
rinex2bin \
    -f data/NAV/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    --gps \
    --tcp 127.0.0.1 \
    --port 1234
```

Logs
====

`rinex2bin` uses the standardized Rust logger to notify of potential local issues
during streaming. To activate the logs, 
simply define the `RUST_LOG` environment variable, you have many sensivity levels:

- info
- error
- debug
- trace

Getting started
===============

`rinex2bin` always requires an input RINEX that needs to be readable.
Any supported format may apply, refer to the [list of supported formats](https://github.com/georust/rinex).

By default, the tool will serialize into a standardized binary ("bin") file. The file name
is guessed from the actual RINEX content.

```bash
rinex2bin amel0010.21g
```

The BINEX stream will always start with the following key elements

- a BINEX Monument GEO comment announcing the start of a new stream,
including the name of this software package & revision
- a BINEX Monument GEO comment describing the software name & revision
of the underlying BINEX streamer
- Announcement of the serialized RINEX Header to follow, by means of a BINEX Monument GEO comment
- Any Comments found in your RINEX is serialized by means of a BINEX Monument GEO comment
- Any geodetic information, including Approximate coordinates of your geodetic marker, is encoded
by means of Moument Geo frames
- The end of the serialized header section is announced by a new Monument GEO comment
announcing the start of the file body
- The stream to follow is RINEX type dependent

```bash
RUST_LOG rinex2bin amel0010.21g

[2025-02-22T13:21:01Z DEBUG rinex2bin] Streaming started!
[2025-02-22T13:44:53Z DEBUG rinex2bin] Streaming: Message { meta: Meta { reversed: false, enhanced_crc: false, big_endian: true }, record: MonumentGeo(MonumentGeoRecord { epoch: 2020-12-31T23:45:00 UTC, meta: RNX2BIN, comments: ["nav-solutions/rinex2bin v0.0.1 from V2 Glonass NAVIGATION DATA", "Stream starting!"], frames: [GeoStringFrame { fid: SoftwareName, string: "geo-rust v0.17.0-beta" }] }) }
[2025-02-22T13:21:01Z DEBUG rinex2bin] Streaming: Message { meta: Meta { reversed: false, enhanced_crc: false, big_endian: true }, record: MonumentGeo(MonumentGeoRecord { epoch: 2020-12-31T23:45:00 UTC, meta: RNX2BIN, comments: [], frames: [GeoStringFrame { fid: SoftwareName, string: "geo-rust v0.17.0-beta" }] }) }
[2025-02-22T13:21:01Z DEBUG rinex2bin] Streaming: Message { meta: Meta { reversed: false, enhanced_crc: false, big_endian: true }, record: MonumentGeo(MonumentGeoRecord { epoch: 2020-12-31T23:45:00 UTC, meta: RNX2BIN, comments: ["RINEX Header comments following!"], frames: [GeoStringFrame { fid: SoftwareName, string: "geo-rust v0.17.0-beta" }] }) }
[2025-02-22T13:21:01Z DEBUG rinex2bin] Streaming: Message { meta: Meta { reversed: false, enhanced_crc: false, big_endian: true }, record: MonumentGeo(MonumentGeoRecord { epoch: 2020-12-31T23:45:00 UTC, meta: RNX2BIN, comments: ["Linux 2.4.21-27.ELsmp|Opteron|gcc|Linux 64|=+", "GN-RINEX 1.3        Geo++ GmbH          31-DEC-20 23:59", "gfzrnx-1.13-7761    FILE MERGE          20210101 010301 UTC"], frames: [GeoStringFrame { fid: SoftwareName, string: "geo-rust v0.17.0-beta" }] }) }
[2025-02-22T13:21:01Z DEBUG rinex2bin] Streaming: Message { meta: Meta { reversed: false, enhanced_crc: false, big_endian: true }, record: MonumentGeo(MonumentGeoRecord { epoch: 2020-12-31T23:45:00 UTC, meta: RNX2BIN, comments: ["RINEX Record starting!"], frames: [GeoStringFrame { fid: SoftwareName, string: "geo-rust v0.17.0-beta" }] }) }
[2025-02-22T13:21:01Z DEBUG rinex2bin] Streaming: Message { meta: Meta { reversed: false, enhanced_crc: false, big_endian: true }, record: EphemerisFrame(GLO(GLOEphemeris { slot: 0, day: 0, tod_s: 0, clock_offset_s: -4.20100986958e-5, clock_rel_freq_bias: 0.0, t_k_sec: 0, x_km: 18170.6850586, vel_x_km: 0.818475723267, acc_x_km: 2.79396772385e-9, y_km: 18170.6850586, vel_y_km: 0.75625038147, acc_y_km: 9.31322574615e-10, z_km: 18170.6850586, vel_z_km: -3.34566307068, acc_z_km: -2.79396772385e-9, sv_health: 0, freq_channel: 0, age_op_days: 0, leap_s: 0, tau_gps_s: 0.0, l1_l2_gd: 0.0 })) }
[2025-02-22T13:21:01Z DEBUG rinex2bin] Streaming: Message { meta: Meta { reversed: false, enhanced_crc: false, big_endian: true }, record: EphemerisFrame(GLO(GLOEphemeris { slot: 0, day: 0, tod_s: 0, clock_offset_s: 0.000461053103209, clock_rel_freq_bias: 1.81898940355e-12, t_k_sec: 0, x_km: -8955.04199219, vel_x_km: 1.43687725067, acc_x_km: -4.65661287308e-9, y_km: -8955.04199219, vel_y_km: 1.5303068161, acc_y_km: -9.31322574615e-10, z_km: -8955.04199219, vel_z_km: 2.66476726532, acc_z_km: 9.31322574615e-10, sv_health: 0, freq_channel: 0, age_op_days: 0, leap_s: 0, tau_gps_s: 0.0, l1_l2_gd: 0.0 })) }
[2025-02-22T13:21:01Z DEBUG rinex2bin] Streaming: Message { meta: Meta { reversed: false, enhanced_crc: false, big_endian: true }, record: EphemerisFrame(GLO(GLOEphemeris { slot: 0, day: 0, tod_s: 0, clock_offset_s: 2.83820554614e-5, clock_rel_freq_bias: 0.0, t_k_sec: 0, x_km: 15025.2294922, vel_x_km: 1.75297737122, acc_x_km: 9.31322574615e-10, y_km: 15025.2294922, vel_y_km: -0.947224617004, acc_y_km: -9.31322574615e-10, z_km: 15025.2294922, vel_z_km: -2.76998615265, acc_z_km: -2.79396772385e-9, sv_health: 0, freq_channel: 0, age_op_days: 0, leap_s: 0, tau_gps_s: 0.0, l1_l2_gd: 0.0 })) }
[...]
```

The input RINEX can be Gzip compressed, but its name needs to be terminated by `.gz`

```bash
RUST_LOG=trace rinex2bin GEOP092I.24o.gz

[2025-02-22T13:54:02Z DEBUG rinex2bin] Streaming started!
[2025-02-22T13:54:02Z DEBUG rinex2bin] Streaming: Message { meta: Meta { reversed: false, enhanced_crc: false, big_endian: true }, record: MonumentGeo(MonumentGeoRecord { epoch: 2024-04-01T08:30:58.442760200 UTC, meta: RNX2BIN, comments: ["nav-solutions/rinex2bin v0.0.1 from V3 MIXED OBS DATA", "Stream starting!"], frames: [GeoStringFrame { fid: SoftwareName, string: "geo-rust v0.17.0-beta" }] }) }
[2025-02-22T13:54:02Z DEBUG rinex2bin] Streaming: Message { meta: Meta { reversed: false, enhanced_crc: false, big_endian: true }, record: MonumentGeo(MonumentGeoRecord { epoch: 2024-04-01T08:30:58.442760200 UTC, meta: RNX2BIN, comments: [], frames: [GeoStringFrame { fid: SoftwareName, string: "geo-rust v0.17.0-beta" }, GeoStringFrame { fid: AgencyName, string: "Geo++" }, GeoStringFrame { fid: ObserverName, string: "Geo++" }, GeoStringFrame { fid: ReceiverType, string: "Xiaomi" }, GeoStringFrame { fid: ReceiverNumber, string: "unknown" }, GeoStringFrame { fid: ReceiverFirmwareVersion, string: "M2007J17G" }] }) }
[2025-02-22T13:54:02Z DEBUG rinex2bin] Streaming: Message { meta: Meta { reversed: false, enhanced_crc: false, big_endian: true }, record: MonumentGeo(MonumentGeoRecord { epoch: 2024-04-01T08:30:58.442760200 UTC, meta: RNX2BIN, comments: ["RINEX Header comments following!"], frames: [GeoStringFrame { fid: SoftwareName, string: "geo-rust v0.17.0-beta" }] }) }
[2025-02-22T13:54:02Z DEBUG rinex2bin] Streaming: Message { meta: Meta { reversed: false, enhanced_crc: false, big_endian: true }, record: MonumentGeo(MonumentGeoRecord { epoch: 2024-04-01T08:30:58.442760200 UTC, meta: RNX2BIN, comments: ["************************************************************", "This file was generated by the Geo++ RINEX Logger App", "for Android devices (Version 2.1.6). If you encounter", "any issues, please send an email to android@geopp.de", "Filtering Mode: BEST", "************************************************************"], frames: [GeoStringFrame { fid: SoftwareName, string: "geo-rust v0.17.0-beta" }] }) }
[2025-02-22T13:54:02Z DEBUG rinex2bin] Streaming: Message { meta: Meta { reversed: false, enhanced_crc: false, big_endian: true }, record: MonumentGeo(MonumentGeoRecord { epoch: 2024-04-01T08:30:58.442760200 UTC, meta: RNX2BIN, comments: ["RINEX Record starting!"], frames: [GeoStringFrame { fid: SoftwareName, string: "geo-rust v0.17.0-beta" }] }) }
```

## Auto-guessed name

When serializing RINEX3 in default mode, the output BIN file is automatically guessed from content to be streamed.  
By default, RINEX3 prefers long and precise filenames:

```bash
RUST_LOG=trace rinex2bin ESBC00DNK_R_20201770000_01D_MN.rnx.gz
ESBC00DNK_R_20201771950_01D_MN.rnx.bin as been collected
```

If you prefer to generate a shorter (V2) file name, simply use `-s`:

```bash
RUST_LOG=trace rinex2bin -s GEOP092I.24o.gz
[...]
[...]
ESBC1770.20N.bin has been collected
```

## BIN file name

By default, the BIN file name is auto-guessed from the actual RINEX content.
You may specify a custom name with `-o` instead:

```bash
RUST_LOG=trace rinex2bin GEOP092I.24o.gz -o GEOP092I.24o.bin
[...]
[...]
GEOP092I.24o.bin has been generated
```

The output filename is absolute:

```bash
RUST_LOG=trace rinex2bin GEOP092I.24o.gz -o /tmp/GEOP092I.24o.bin
[...]
[...]
/tmp/GEOP092I.24o.bin has been generated
```

Gzip compressed BINEX stream
============================

In both cases, the tool allows gzip compression of the BINEX stream, for ultimate storage efficiency.

When working with the auto-guesser (default option) specify you want to Gzip compress
the auto-named BIN file with `--gzip`:

```bash
RUST_LOG=trace rinex2bin GEOP092I.24o.gz --gzip
[...]
[...]
GEOP092I.24o.bin.gz has been compressed
```

If your output file name terminates with `.gz`:

```bash
RUST_LOG=trace rinex2bin GEOP092I.24o.gz -o /tmp/GEOP092I.24o.bin.gz
[...]
[...]
/tmp/GEOP092I.24o.bin.gz has been compressed
```

When streaming to an I/O, you need to add the `--gzip` flag to the streaming mode:

```bash
RUST_LOG=trace rinex2bin GEOP092I.24o.gz -s /dev/ttyUSB0 --gzip
[...]
```

## Licensing

This application is part of the [Nav-solutions framework](https://github.com/nav-solutions) which
is licensed under [Mozilla V2 Public](https://www.mozilla.org/en-US/MPL/2.0) license.
