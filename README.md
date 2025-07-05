# rftap

rftap is a library for parsing and serializing the [RFTap protocol](https://rftap.github.io/), which is used in recordings of radio frequency (RF) data transmission and is also supported in Wireshark.

## Parse

```rust
let serialized: Vec<u8> = hex::decode( "524674610a00250694000000000000004368a341000000000000808e77e7d8410000000079d6d03f")?;
let restored = RfTapPacket::parse(&serialized)?;
```

## Serialize

```rust
let packet = RfTapPacket {
    dlt: Some(123),
    freq: Some(1234566.0),
    nomfreq: Some(67456745.0),
    freqofs: Some(0.000003),
    isdbm: false,
    power: Some(12.3),
    noise: Some(0.4),
    snr: Some(99.9),
    isunixtime: true,
    qual: Some(100.0),
    time: Some(1751558597250044416),
    duration: Some(33.0),
    location: Some((48.0, 14.0, 440.0)),
    payload: &vec![0xff, 0xff, 0xff]
};

let serialized: Vec<u8> = packet.serialize()?;
```