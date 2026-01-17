use anyhow::{bail, Result};
use byteorder::{ByteOrder, LittleEndian};

use crate::{RFTapPacket, *};

impl<'a> RFTapPacket<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self> {
        if input.len() < 8 {
            bail!("Input too short")
        }

        let mut result = Self {
            ..Default::default()
        };

        if input[0..4] != vec![b'R', b'F', b't', b'a'] {
            bail!("Cannot find magic header")
        }

        let header_length: usize = (LittleEndian::read_u16(&input[4..6]) * 4) as usize;
        if input.len() < header_length {
            bail!("Input is shorter than indicated")
        }

        let flags: u16 = LittleEndian::read_u16(&input[6..8]);

        let mut current_position: usize = 8;

        if (flags >> DLT) & 0b1 == 1 {
            result.dlt = Some(LittleEndian::read_u32(&input[current_position..current_position+4]));
            current_position += 4
        }

        if (flags >> FREQ) & 0b1 == 1 {
            result.freq = Some(LittleEndian::read_f64(&input[current_position..current_position+8]));
            current_position += 8
        }

        if (flags >> NOMFREQ) & 0b1 == 1 {
            result.nomfreq = Some(LittleEndian::read_f64(&input[current_position..current_position+8]));
            current_position += 8
        }

        if (flags >> FREQOFS) & 0b1 == 1 {
            result.freqofs = Some(LittleEndian::read_f64(&input[current_position..current_position+8]));
            current_position += 8
        }

        result.isdbm = (flags >> ISDBM) & 0b1 == 1;

        if (flags >> POWER) & 0b1 == 1 {
            result.power = Some(LittleEndian::read_f32(&input[current_position..current_position+4]));
            current_position += 4
        }

        if (flags >> NOISE) & 0b1 == 1 {
            result.noise = Some(LittleEndian::read_f32(&input[current_position..current_position+4]));
            current_position += 4
        }

        if (flags >> SNR) & 0b1 == 1 {
            result.snr = Some(LittleEndian::read_f32(&input[current_position..current_position+4]));
            current_position += 4
        }

        if (flags >> QUAL) & 0b1 == 1 {
            result.qual = Some(LittleEndian::read_f32(&input[current_position..current_position+4]));
            current_position += 4
        }

        result.isunixtime = (flags >> ISUNIXTIME) & 0b1 == 1;

        if (flags >> TIME) & 0b1 == 1 {
            let int_part = LittleEndian::read_f64(&input[current_position..current_position+8]);
            let frac_part = LittleEndian::read_f64(&input[current_position+8..current_position+16]);
            result.time = Some(seconds_to_nanoseconds(int_part, frac_part));
            current_position += 16;
        }

        if (flags >> DURATION) & 0b1 == 1 {
            result.duration = Some(LittleEndian::read_f64(&input[current_position..current_position+8]));
            current_position += 8
        }

        if (flags >> LOCATION) & 0b1 == 1 {
            result.location = Some((
                LittleEndian::read_f64(&input[current_position..current_position+8]),
                LittleEndian::read_f64(&input[current_position+8..current_position+16]),
                LittleEndian::read_f64(&input[current_position+16..current_position+24])
            ));
            current_position += 24
        }


        // tags
        while current_position + 4 <= header_length {
            let tag_id = LittleEndian::read_u16(&input[current_position..current_position+2]);
            let tag_len = input[current_position+2];
            let tag_flags = input[current_position+3];
            current_position += 4;

            if current_position + tag_len as usize > header_length {
                bail!("Tag length extends beyond header");
            }

            let tag_value = input[current_position..current_position+tag_len as usize].to_vec();
            current_position += tag_len as usize;

            result.tags.push(Tag {
                id: tag_id,
                length: tag_len,
                flags: tag_flags,
                value: tag_value,
            });
        }

        result.payload = &input[header_length..];

        Ok(result)
    }
}

fn seconds_to_nanoseconds(int_part: f64, frac_part: f64) -> u128 {
    let total_seconds = int_part + frac_part;
    (total_seconds * 1_000_000_000.0) as u128
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

        let data = hex::decode( "524674610a00250694000000000000004368a341000000000000808e77e7d8410000000079d6d03f").unwrap();
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
        assert_eq!(packet.time.unwrap(), 1671290426263090432);
        assert!(packet.duration.is_none());
        assert!(packet.location.is_none());
        assert_eq!(packet.payload.len(), 0)
    }
}