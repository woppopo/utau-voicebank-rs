use std::{fs::File, io::Read, path::Path};

use encoding_rs::SHIFT_JIS;

pub mod character;
pub mod frequency;
pub mod oto;

pub(crate) fn read_to_string_sjis(path: impl AsRef<Path>) -> Option<String> {
    let mut file = File::open(path).unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).ok()?;

    let (cow_str, _, had_errors) = SHIFT_JIS.decode(&bytes);
    if had_errors {
        return None;
    }

    Some(cow_str.into_owned())
}
