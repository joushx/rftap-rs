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

#[derive(Debug, PartialEq)]
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
    pub time: Option<(f64, f64)>,
    pub duration: Option<f64>,
    pub location: Option<(f64, f64, f64)>,
    pub payload: &'a [u8]
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use proptest::option;

    proptest! {
        #[test]
        fn test_all_fields_end_to_end(
            dlt in option::of(0u32..std::u32::MAX),
            freq in option::of(0f64..std::f64::MAX),
            nomfreq in option::of(0f64..std::f64::MAX),
            freqofs in option::of(0f64..std::f64::MAX),
            isdbm in any::<bool>(),
            power in option::of(0f32..std::f32::MAX),
            noise in option::of(0f32..std::f32::MAX),
            snr in option::of(0f32..std::f32::MAX),
            isunixtime in any::<bool>(),
            qual in option::of(0f32..std::f32::MAX),
            time in option::of((0f64..std::f64::MAX, 0f64..std::f64::MAX)),
            duration in option::of(0f64..std::f64::MAX),
            location in option::of((-90.0f64..90.0, -180.0f64..180.0, -1000.0f64..10000.)),
            payload in prop::collection::vec(any::<u8>(), 0..256)
        ) {
            let packet = RFTapPacket {
                dlt,
                freq,
                nomfreq,
                freqofs,
                isdbm,
                power,
                noise,
                snr,
                isunixtime,
                qual,
                time,
                duration,
                location,
                payload: payload.as_slice(),
            };

            let serialized = packet.serialize().unwrap();

            let restored = RFTapPacket::parse(&serialized).unwrap();

            assert_eq!(packet, restored)
        }
    }
}