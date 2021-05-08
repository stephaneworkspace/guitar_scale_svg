use std::io;
use std::fs;
use guitar_scale::note::{Note, TraitGenerate};
use std::str::FromStr;
use std::path::Path;

/// Copy assets from renoise output folder to ./temp audio
fn main() -> io::Result<()> {
    let vec_note = ["C","C#","D","D#","E","F","F#","G","G#","A","A#","B",];
    let mut idx: u8 = 0 as u8;
    for pitch in 0 as u8..3 as u8 {
        for x in vec_note.to_vec().iter().filter(|&&x_str| if pitch == 0 {
            if x_str == "C" || x_str == "C#" || x_str == "D" || x_str == "D#" {
                false
            } else {
                true
            }
        } else {
            true
        } ) {
            idx += 1;
            copy(*x, pitch, idx)?;
        }
    }
    Ok(())
}

/// Copy assets from renoise output folder to ./temp_audio
fn copy(note: &str, pitch: u8, i: u8) -> io::Result<()> {
    let note= match Note::from_str(&note) {
        Ok(ok) => ok,
        Err(err) => {
            panic!("{}", err)
        }
    };
    let origin_string: String = String::from(format!("./renoise_guitar_split/output/guitar_split_Seq{:02}.wav", i));
    let origin = Path::new(&origin_string);
    let dest_string: String = String::from(format!("./temp_audio/guitar_{}_{}.wav",note.data_name(), pitch));
    let dest = Path::new(&dest_string);
    fs::copy(origin, dest)?;
    Ok(())
}