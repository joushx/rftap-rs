use byteorder::{ByteOrder, LittleEndian};
use crate::{RFTapPacket, *};

macro_rules! read_field {
    (
        $input:expr,
        $pos:expr,
        $size:expr,
        $reader:ident,
        $name:literal
    ) => {{
        let value = LittleEndian::$reader(
            $input
                .get($pos..$pos + $size)
                .ok_or_else(|| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("input to short to read {} field", $name),
                    )
                })?
        );

        $pos += $size;

        value
    }};
}

impl<'a> RFTapPacket<'a> {

    /// parses a RFTap header from a byte slice
    pub fn parse(input: &'a [u8]) -> Result<Self, std::io::Error> {
        if input.len() < 8 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Input too short"));
        }

        let mut result = Self {
            dlt: None,
            freq: None,
            nomfreq: None,
            payload: &[],
            freqofs: None,
            power: None,
            isdbm: false,
            noise: None,
            snr: None,
            isunixtime: false,
            qual: None,
            time: None,
            duration: None,
            location: None
        };

        if input.get(0..4) != Some(b"RFta") {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Cannot find magic header"));
        }

        let mut current_position: usize = 4;

        let header_length_u32 = read_field!(input, current_position, 2, read_u16, "header size");
        let header_length: usize = (header_length_u32 as usize) * 4;

        if input.len() < header_length {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Input is shorter than indicated"));
        }

        let flags: u16 = read_field!(input, current_position, 2, read_u16, "flags");

        if (flags >> DLT) & 0b1 == 1 {
            result.dlt = Some(read_field!(input, current_position, 4, read_u32, "dlt"));
        }

        if (flags >> FREQ) & 0b1 == 1 {
            result.freq = Some(read_field!(input, current_position, 8, read_f64, "freq"));
        }

        if (flags >> NOMFREQ) & 0b1 == 1 {
            result.nomfreq = Some(read_field!(input, current_position, 8, read_f64, "nomfreq"));
        }

        if (flags >> FREQOFS) & 0b1 == 1 {
            result.freqofs = Some(read_field!(input, current_position, 8, read_f64, "freqofs"));
        }

        result.isdbm = (flags >> ISDBM) & 0b1 == 1;

        if (flags >> POWER) & 0b1 == 1 {
            result.power= Some(read_field!(input, current_position, 4, read_f32, "power"));
        }

        if (flags >> NOISE) & 0b1 == 1 {
            result.noise = Some(read_field!(input, current_position, 4, read_f32, "noise"));
        }

        if (flags >> SNR) & 0b1 == 1 {
            result.snr = Some(read_field!(input, current_position, 4, read_f32, "snr"));
        }

        if (flags >> QUAL) & 0b1 == 1 {
            result.qual = Some(read_field!(input, current_position, 4, read_f32, "qual"));
        }

        result.isunixtime = (flags >> ISUNIXTIME) & 0b1 == 1;

        if (flags >> TIME) & 0b1 == 1 {
            let int_part = read_field!(input, current_position, 8, read_f64, "timeint");
            let frac_part = read_field!(input, current_position, 8, read_f64, "timefrac");
            result.time = Some((int_part, frac_part));
        }

        if (flags >> DURATION) & 0b1 == 1 {
            result.duration = Some(read_field!(input, current_position, 8, read_f64, "timeint"));
        }

        if (flags >> LOCATION) & 0b1 == 1 {
            result.location = Some((
                read_field!(input, current_position, 8, read_f64, "lat"),
                read_field!(input, current_position, 8, read_f64, "lon"),
                read_field!(input, current_position, 8, read_f64, "alt")
            ));
        }

        result.payload = &input[current_position..];

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_against_wireshark() {
        // RFtap Protocol (40 bytes)
        //     RFtap Fixed header
        //     Data Link Type (DLT): 148
        //     Nominal Frequency: 162800000Hz
        //     Signal Power: 0.00 dB
        //     Time (integer part): 1671290426 seconds
        //     Time (fractional part): 0.263090372 seconds
        //     Time: 1671290426.263090 seconds

        let data = hex::decode("524674610a00250694000000000000004368a341000000000000808e77e7d8410000000079d6d03f").unwrap();
        let packet = RFTapPacket::parse(&data).unwrap();

        assert_eq!(packet.dlt.unwrap(), 148);
        assert!(packet.freq.is_none());
        assert_eq!(packet.nomfreq.unwrap(), 162800000.0);
        assert!(packet.freqofs.is_none());
        assert_eq!(packet.isdbm, false);
        assert_eq!(packet.power.unwrap(), 0.0);
        assert!(packet.noise.is_none());
        assert!(packet.snr.is_none());
        assert_eq!(packet.isunixtime, true);
        let (time_int, time_frac) = packet.time.unwrap();
        assert_eq!(time_int, 1671290426.0);
        assert!((time_frac - 1671290426.0) < 0.000001);
        assert!(packet.duration.is_none());
        assert!(packet.location.is_none());
        assert_eq!(packet.payload.len(), 0)
    }

    proptest! {
        #[test]
        fn test_does_not_panic(
            payload in prop::collection::vec(any::<u8>(), 0..100)
        ) {
            let mut buf = Vec::new();
            buf.extend(vec![b'R', b'F', b't', b'a']);
            buf.extend(payload);

            let _ = RFTapPacket::parse(&buf);
        }
    }

    #[test]
    fn test_invalid_magic_header() {
        let data = vec![0u8; 16]; // no "RFta"
        let err = RFTapPacket::parse(&data).unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    }

    #[test]
    fn test_truncated_header_length() {
        let mut data = b"RFta".to_vec();
        data.extend_from_slice(&[10, 0]); // size too large
        data.extend_from_slice(&[0, 0]);

        let err = RFTapPacket::parse(&data).unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    }

    #[test]
    fn test_minimal_valid_packet() {
        let mut data = vec![];
        data.extend_from_slice(b"RFta");
        data.extend_from_slice(&2u16.to_le_bytes());
        data.extend_from_slice(&0u16.to_le_bytes());

        let pkt = RFTapPacket::parse(&data).unwrap();

        assert!(pkt.dlt.is_none());
        assert!(pkt.freq.is_none());
        assert!(pkt.nomfreq.is_none());
        assert_eq!(pkt.payload.len(), 0);
    }
}