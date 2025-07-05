use anyhow::Result;

use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};

use crate::{RFTapPacket, *};

impl<'a> RFTapPacket<'a> {
    pub fn serialize(&self) -> Result<Vec<u8>> {
        let mut buffer: Vec<u8> = vec![
            b'R', b'F', b't', b'a', // magic
            0, 0, // placeholder for size
            0, 0, // placeholder for flags
        ];

        let mut length_32: u16 = 2;
        let mut flags: u16 = 0;

        if let Some(dlt) = self.dlt {
            length_32 += 1;
            flags |= 1 << DLT;
            buffer.write_u32::<LittleEndian>(dlt)?;
        }

        if let Some(freq) = self.freq {
            length_32 += 2;
            flags |= 1 << FREQ;
            buffer.write_f64::<LittleEndian>(freq)?;
        }

        if let Some(nomfreq) = self.nomfreq{
            length_32 += 2;
            flags |= 1 << NOMFREQ;
            buffer.write_f64::<LittleEndian>(nomfreq)?;
        }

        if let Some(freqofs) = self.freqofs{
            length_32 += 2;
            flags |= 1 << FREQOFS;
            buffer.write_f64::<LittleEndian>(freqofs)?;
        }

        if self.isdbm {
            flags |= 1 << ISDBM;
        }

        if let Some(power) = self.power {
            length_32 += 1;
            flags |= 1 << POWER;
            buffer.write_f32::<LittleEndian>(power)?;
        }

        if let Some(noise) = self.noise {
            length_32 += 1;
            flags |= 1 << NOISE;
            buffer.write_f32::<LittleEndian>(noise)?;
        }

        if let Some(snr) = self.snr {
            length_32 += 1;
            flags |= 1 << SNR;
            buffer.write_f32::<LittleEndian>(snr)?;
        }

        if self.isunixtime {
            flags |= 1 << ISUNIXTIME;
        }

        if let Some(qual) = self.qual {
            length_32 += 1;
            flags |= 1 << QUAL;
            buffer.write_f32::<LittleEndian>(qual)?;
        }

        if let Some(time) = self.time {
            let (int_time, frac_time) = nanoseconds_to_seconds_parts(time);
            length_32 += 4;
            flags |= 1 << TIME;
            buffer.write_f64::<LittleEndian>(int_time)?;
            buffer.write_f64::<LittleEndian>(frac_time)?;
        }

        if let Some(duration) = self.duration {
            length_32 += 2;
            flags |= 1 << DURATION;
            buffer.write_f64::<LittleEndian>(duration)?;
        }

        if let Some((lat, lon, alt)) = self.location {
            length_32 += 6;
            flags |= 1 << LOCATION;
            buffer.write_f64::<LittleEndian>(lat)?;
            buffer.write_f64::<LittleEndian>(lon)?;
            buffer.write_f64::<LittleEndian>(alt)?;
        }

        LittleEndian::write_u16(&mut buffer[4..6], length_32);
        LittleEndian::write_u16(&mut buffer[6..8], flags);

        buffer.extend(self.payload);

        Ok(buffer)
    }
}

fn nanoseconds_to_seconds_parts(nanoseconds: u128) -> (f64, f64) {
    let total_seconds = nanoseconds as f64 / 1_000_000_000.0;
    let int_part = total_seconds.floor();
    let frac_part = total_seconds - int_part;
    (int_part, frac_part)
}