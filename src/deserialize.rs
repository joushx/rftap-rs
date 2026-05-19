use byteorder::{ByteOrder, LittleEndian};

use crate::{RFTapPacket, *};

impl<'a> RFTapPacket<'a> {
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

        let header_length: usize = (LittleEndian::read_u16(
            input.get(4..6).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Input too short for header length"))?
        ) * 4) as usize;

        if input.len() < header_length {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Input is shorter than indicated"));
        }

        let flags: u16 = LittleEndian::read_u16(
            input.get(6..8).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Input too short for flags"))?
        );

        let mut current_position: usize = 8;

        if (flags >> DLT) & 0b1 == 1 {
            result.dlt = Some(LittleEndian::read_u32(
                input.get(current_position..current_position+4)
                    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Input too short for dlt"))?
            ));
            current_position += 4
        }

        if (flags >> FREQ) & 0b1 == 1 {
            result.freq = Some(LittleEndian::read_f64(
                input.get(current_position..current_position+8)
                    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Input too short for freq"))?
            ));
            current_position += 8
        }

        if (flags >> NOMFREQ) & 0b1 == 1 {
            result.nomfreq = Some(LittleEndian::read_f64(
                input.get(current_position..current_position+8)
                    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Input too short for nomfreq"))?
            ));
            current_position += 8
        }

        if (flags >> FREQOFS) & 0b1 == 1 {
            result.freqofs = Some(LittleEndian::read_f64(
                input.get(current_position..current_position+8)
                    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Input too short for freqofs"))?
            ));
            current_position += 8
        }

        result.isdbm = (flags >> ISDBM) & 0b1 == 1;

        if (flags >> POWER) & 0b1 == 1 {
            result.power = Some(LittleEndian::read_f32(
                input.get(current_position..current_position+4)
                    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Input too short for power"))?
            ));
            current_position += 4
        }

        if (flags >> NOISE) & 0b1 == 1 {
            result.noise = Some(LittleEndian::read_f32(
                input.get(current_position..current_position+4)
                    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Input too short for noise"))?
            ));
            current_position += 4
        }

        if (flags >> SNR) & 0b1 == 1 {
            result.snr = Some(LittleEndian::read_f32(
                input.get(current_position..current_position+4)
                    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Input too short for snr"))?
            ));
            current_position += 4
        }

        if (flags >> QUAL) & 0b1 == 1 {
            result.qual = Some(LittleEndian::read_f32(
                input.get(current_position..current_position+4)
                    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Input too short for qual"))?
            ));
            current_position += 4
        }

        result.isunixtime = (flags >> ISUNIXTIME) & 0b1 == 1;

        if (flags >> TIME) & 0b1 == 1 {
            let int_part = LittleEndian::read_f64(
                input.get(current_position..current_position+8)
                    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Input too short for time.int"))?
            );
            let frac_part = LittleEndian::read_f64(
                input.get(current_position+8..current_position+16)
                    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Input too short for time.frac"))?
            );
            result.time = Some((int_part, frac_part));
            current_position += 16;
        }

        if (flags >> DURATION) & 0b1 == 1 {
            result.duration = Some(LittleEndian::read_f64(
                input.get(current_position..current_position+8)
                    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Input too short for duration"))?
            ));
            current_position += 8
        }

        if (flags >> LOCATION) & 0b1 == 1 {
            result.location = Some((
                LittleEndian::read_f64(
                    input.get(current_position..current_position+8)
                        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Input too short for location.lat"))?
                ),
                LittleEndian::read_f64(
                    input.get(current_position+8..current_position+16)
                        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Input too short for location.lon"))?
                ),
                LittleEndian::read_f64(
                    input.get(current_position+16..current_position+24)
                        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Input too short for location.alt"))?
                )
            ));
            current_position += 24
        }

        result.payload = &input[current_position..];

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}