use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use crate::{RFTapPacket, *};

macro_rules! write_field {
    (
        $buffer:expr,
        $method:ident,
        $value:expr,
        $flags:expr,
        $flag:expr,
        $name:literal
    ) => {{
        $flags |= 1 << $flag;

        $buffer
            .$method::<LittleEndian>($value)
            .map_err(|e| {
                std::io::Error::new(
                    e.kind(),
                    format!("failed to write {} field: {}", $name, e),
                )
            })?;
    }};
}

impl<'a> RFTapPacket<'a> {

    /// serializes a RFTap packet into bytes
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = Vec::with_capacity(100 + self.payload.len());

        buffer.extend_from_slice(&[
            b'R', b'F', b't', b'a', // magic
            0, 0, // placeholder for size
            0, 0, // placeholder for flags
        ]);

        let mut length_32: u16 = 2;
        let mut flags: u16 = 0;

        if let Some(dlt) = self.dlt {
            length_32 += 1;
            write_field!(buffer, write_u32, dlt, flags, DLT, "dlt");
        }

        if let Some(freq) = self.freq {
            length_32 += 2;
            write_field!(buffer, write_f64, freq, flags, FREQ, "freq");
        }

        if let Some(nomfreq) = self.nomfreq {
            length_32 += 2;
            write_field!(buffer, write_f64, nomfreq, flags, NOMFREQ, "nomfreq");
        }

        if let Some(freqofs) = self.freqofs {
            length_32 += 2;
            write_field!(buffer, write_f64, freqofs, flags, FREQOFS, "freqofs");
        }

        if self.isdbm {
            flags |= 1 << ISDBM;
        }

        if let Some(power) = self.power {
            length_32 += 1;
            write_field!(buffer, write_f32, power, flags, POWER, "power");
        }

        if let Some(noise) = self.noise {
            length_32 += 1;
            write_field!(buffer, write_f32, noise, flags, NOISE, "noise");
        }

        if let Some(snr) = self.snr {
            length_32 += 1;
            write_field!(buffer, write_f32, snr, flags, SNR, "snr");
        }

        if self.isunixtime {
            flags |= 1 << ISUNIXTIME;
        }

        if let Some(qual) = self.qual {
            length_32 += 1;
            write_field!(buffer, write_f32, qual, flags, QUAL, "qual");
        }

        if let Some(time) = self.time {
            let (int_time, frac_time) = time;
            length_32 += 4;
            flags |= 1 << TIME;

            buffer
                .write_f64::<LittleEndian>(int_time)
                .map_err(|e| {
                    std::io::Error::new(
                        e.kind(),
                        format!("failed to write time_int field: {e}")
                    )
                })?;

            buffer
                .write_f64::<LittleEndian>(frac_time)
                .map_err(|e| {
                    std::io::Error::new(
                        e.kind(),
                        format!("failed to write time_frac field: {e}")
                    )
                })?;
        }

        if let Some(duration) = self.duration {
            length_32 += 2;
            write_field!(buffer, write_f64, duration, flags, DURATION, "duration");
        }

        if let Some((lat, lon, alt)) = self.location {
            length_32 += 6;
            flags |= 1 << LOCATION;

            buffer
                .write_f64::<LittleEndian>(lat)
                .map_err(|e| {
                    std::io::Error::new(
                        e.kind(),
                        format!("failed to write location.lat field: {e}")
                    )
                })?;

            buffer
                .write_f64::<LittleEndian>(lon)
                .map_err(|e| {
                    std::io::Error::new(
                        e.kind(),
                        format!("failed to write location.lon field: {e}")
                    )
                })?;

            buffer
                .write_f64::<LittleEndian>(alt)
                .map_err(|e| {
                    std::io::Error::new(
                        e.kind(),
                        format!("failed to write location.alt field: {e}")
                    )
                })?;
        }

        LittleEndian::write_u16(&mut buffer[4..6], length_32);
        LittleEndian::write_u16(&mut buffer[6..8], flags);

        buffer.extend(self.payload);

        Ok(buffer)
    }
}