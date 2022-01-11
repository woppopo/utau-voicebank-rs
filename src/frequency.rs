use std::{io::Read, path::Path};

#[repr(C, packed)]
struct RawFrequencyHeader {
    id: [u8; 8],
    sample_interval: u32,
    key_frequency: f64,
    _padding: [u8; 16],
    count_data: u32,
}

#[repr(C, packed)]
struct RawFrequencyData {
    frequency: f64,
    amplitude: f64,
}

#[derive(Clone, Debug)]
pub struct FrequencyData {
    pub frequency: f64,
    pub amplitude: f64,
}

#[derive(Debug, Clone)]
pub struct Frequency {
    pub id: [u8; 8],
    pub sample_interval: u32,
    pub key_frequency: f64,
    pub data: Vec<FrequencyData>,
}

impl Frequency {
    pub fn load(path: impl AsRef<Path>) -> Self {
        let mut file = std::fs::File::open(path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let header = unsafe { &*buffer.as_ptr().cast::<RawFrequencyHeader>() };

        let id = header.id;
        let sample_interval = header.sample_interval;
        let key_frequency = header.key_frequency;
        let count_data = header.count_data;

        let mut data = Vec::new();
        for i in 0..count_data {
            let raw = unsafe {
                let ptr = buffer.as_ptr().cast::<RawFrequencyData>().add(i as usize);
                &*ptr
            };

            data.push(FrequencyData {
                frequency: raw.frequency,
                amplitude: raw.amplitude,
            })
        }

        Self {
            id,
            sample_interval,
            key_frequency,
            data,
        }
    }
}
