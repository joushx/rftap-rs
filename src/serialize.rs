use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};

use crate::{RFTapPacket, *};

impl<'a> RFTapPacket<'a> {

    /// serializes a RFTap packet into bytes
    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = Vec::with_capacity(100 + self.payload.len());

        buffer.extend(vec![
            b'R', b'F', b't', b'a', // magic
            0, 0, // placeholder for size
            0, 0, // placeholder for flags
        ]);

        let mut length_32: u16 = 2;
        let mut flags: u16 = 0;

        if let Some(dlt) = self.dlt {
            length_32 += 1;
            flags |= 1 << DLT;
            buffer
                .write_u32::<LittleEndian>(dlt)
                .map_err(|e| {
                    std::io::Error::new(
                        e.kind(),
                        format!("failed to write dlt field: {e}")
                    )
                })?;
        }

        if let Some(freq) = self.freq {
            length_32 += 2;
            flags |= 1 << FREQ;
            buffer
                .write_f64::<LittleEndian>(freq)
                .map_err(|e| {
                    std::io::Error::new(
                        e.kind(),
                        format!("failed to write freq field: {e}")
                    )
                })?;
        }

        if let Some(nomfreq) = self.nomfreq {
            length_32 += 2;
            flags |= 1 << NOMFREQ;
            buffer
                .write_f64::<LittleEndian>(nomfreq)
                .map_err(|e| {
                    std::io::Error::new(
                        e.kind(),
                        format!("failed to write nomfreq field: {e}")
                    )
                })?;
        }

        if let Some(freqofs) = self.freqofs {
            length_32 += 2;
            flags |= 1 << FREQOFS;
            buffer
                .write_f64::<LittleEndian>(freqofs)
                .map_err(|e| {
                    std::io::Error::new(
                        e.kind(),
                        format!("failed to write freqofs field: {e}")
                    )
                })?;
        }

        if self.isdbm {
            flags |= 1 << ISDBM;
        }

        if let Some(power) = self.power {
            length_32 += 1;
            flags |= 1 << POWER;
            buffer
                .write_f32::<LittleEndian>(power)
                .map_err(|e| {
                    std::io::Error::new(
                        e.kind(),
                        format!("failed to write power field: {e}")
                    )
                })?;
        }

        if let Some(noise) = self.noise {
            length_32 += 1;
            flags |= 1 << NOISE;
            buffer
                .write_f32::<LittleEndian>(noise)
                .map_err(|e| {
                    std::io::Error::new(
                        e.kind(),
                        format!("failed to write noise field: {e}")
                    )
                })?;
        }

        if let Some(snr) = self.snr {
            length_32 += 1;
            flags |= 1 << SNR;
            buffer
                .write_f32::<LittleEndian>(snr)
                .map_err(|e| {
                    std::io::Error::new(
                        e.kind(),
                        format!("failed to write snr field: {e}")
                    )
                })?;
        }

        if self.isunixtime {
            flags |= 1 << ISUNIXTIME;
        }

        if let Some(qual) = self.qual {
            length_32 += 1;
            flags |= 1 << QUAL;
            buffer
                .write_f32::<LittleEndian>(qual)
                .map_err(|e| {
                    std::io::Error::new(
                        e.kind(),
                        format!("failed to write qual field: {e}")
                    )
                })?;
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
                        format!("failed to write time.int field: {e}")
                    )
                })?;

            buffer
                .write_f64::<LittleEndian>(frac_time)
                .map_err(|e| {
                    std::io::Error::new(
                        e.kind(),
                        format!("failed to write time.frac field: {e}")
                    )
                })?;
        }

        if let Some(duration) = self.duration {
            length_32 += 2;
            flags |= 1 << DURATION;
            buffer
                .write_f64::<LittleEndian>(duration)
                .map_err(|e| {
                    std::io::Error::new(
                        e.kind(),
                        format!("failed to write duration field: {e}")
                    )
                })?;
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