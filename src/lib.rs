mod serialize;
mod deserialize;

// 0 Data Link Type (DLT) field is present
const DLT: usize = 0;
// 1 Frequency field is present
const FREQ: usize = 1;
// 2 Nominal frequency field is present
const NOMFREQ: usize = 2;
// 3 Frequency offset field is present
const FREQOFS: usize = 3;
// 4 The power units are dBm (boolean)
const ISDBM: usize = 4;
// 5 Signal power field is present
const POWER: usize = 5;
// 6 Noise power field is present
const NOISE: usize = 6;
// 7 SNR field is present
const SNR: usize = 7;
// 8 Signal quality field is present
const QUAL: usize = 8;
// 9 The time standard is UNIX time (boolean)
const ISUNIXTIME: usize = 9;
// 10 Time field is present
const TIME: usize = 10;
// 11 Duration of packet field is present
const DURATION: usize = 11;
// 12 Location field is present
const LOCATION: usize = 12;
// 13 Reserved, must be 0
// 14 Reserved, must be 0
// 15 Reserved, must be 0

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagId {
    /// Name of sub-dissector used for packet data (alternative to DLT field)
    /// only used in Wireshark and flags must be 0xff
    DissectorName = 16,
}

#[derive(Debug, PartialEq, Default)]
pub struct RFTapPacket<'a> {
    pub dlt: Option<u32>,
    pub freq: Option<f64>,
    pub nomfreq: Option<f64>,
    pub freqofs: Option<f64>,
    pub isdbm: bool,
    pub power: Option<f32>,
    pub noise: Option<f32>,
    pub snr: Option<f32>,
    pub isunixtime: bool,
    pub qual: Option<f32>,
    pub time: Option<u128>,
    pub duration: Option<f64>,
    pub location: Option<(f64, f64, f64)>,
    pub tags: Vec<Tag>,
    pub payload: &'a [u8]
}

#[derive(Debug, PartialEq)]
pub struct Tag {
    pub id: u16,
    pub length: u8,
    pub flags: u8,
    pub value: Vec<u8>
}

impl Tag {
    pub fn new(id: u16, flags: u8, value: Vec<u8>) -> Self {
        let length = value.len() as u8;
        Tag {
            id,
            length,
            flags,
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_fields_end_to_end() {
        let packet = RFTapPacket {
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
            tags: vec![
                Tag::new(TagId::DissectorName as u16, 255, b"r09".to_vec())
            ],
            payload: &vec![0xff, 0xff, 0xff]
        };

        let serialized = packet.serialize().unwrap();

        let restored = RFTapPacket::parse(&serialized).unwrap();

        assert_eq!(packet, restored)
    }
}
