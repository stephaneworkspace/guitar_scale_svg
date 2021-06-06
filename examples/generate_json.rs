use std::str::FromStr;
use std::fs::File;
use guitar_scale::tuning::Tuning;
use strum::IntoEnumIterator;
use std::io::Write;
use guitar_scale::note::{Note, TraitGenerate};
use guitar_scale::scale::Scale;
use guitar_scale::scale::ScaleType;
use guitar_scale::svg_draw::{DrawScale, Theme};
extern crate serde;
#[macro_use]
extern crate serde_derive;
//use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct GuitarScale {
    name: String,
    tonic: String,
    display: Display,
    theme: ThemeIos,
    svg: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum Display {
    Vertical,
    Horizontal
}

#[derive(Serialize, Deserialize, Debug)]
enum ThemeIos {
    Light,
    Dark
}

/// Generate scales svg
fn main() -> std::io::Result<()> {
    let vec_note = ["C","C#","D","D#","E","F","F#","G","G#","A","A#","B"];
    //let serialized: Vec<String> = vec_note.to_vec().iter().map(|x| serde_json::to_string(&note(*x))).collect();
    let mut v: Vec<GuitarScale> = Vec::new();
    for x in vec_note.to_vec().iter() {
        v = GuitarScale::note(*x).into_iter().chain(v).collect()
    }
    let serialized = serde_json::to_string(&v).unwrap();
    let mut buffer = File::create("export.json")?;
    buffer.write_all(&serialized.as_bytes())?;
    Ok(())
}

impl GuitarScale {
    /// Generate scales svg for a specified tonic
    fn note(note: &str) -> Vec<Self> {
        let tonic = match Note::from_str(&note) {
            Ok(ok) => ok,
            Err(err) => {
                panic!("{}", err)
            }
        };
        let vec_scale: Vec<Scale> = ScaleType::iter().map(|x| {
            Scale {
                scale_type: x,
                tuning: Tuning::E,
                tonic
            }
        }).collect();
        let mut vec_guitar_current: Vec<GuitarScale> = Vec::new();
        for v_s in vec_scale {
            let draw: DrawScale = DrawScale::new(v_s, Theme::Light);
            let svg = format!("{}",draw.draw_base_vertical());
            vec_guitar_current.push(Self {
               name: v_s.scale_type.get_name_short(),
                tonic: tonic.data_name().to_string(),
                display: Display::Vertical,
                theme: ThemeIos::Light,
                svg
            });
            let svg = format!("{}",draw.draw_base());
            vec_guitar_current.push(Self {
                name: v_s.scale_type.get_name_short(),
                tonic: tonic.data_name().to_string(),
                display: Display::Horizontal,
                theme: ThemeIos::Light,
                svg
            });
            let draw: DrawScale = DrawScale::new(v_s, Theme::Dark);
            let svg = format!("{}",draw.draw_base_vertical());
            vec_guitar_current.push(Self {
                name: v_s.scale_type.get_name_short(),
                tonic: tonic.data_name().to_string(),
                display: Display::Vertical,
                theme: ThemeIos::Dark,
                svg
            });
            let svg = format!("{}",draw.draw_base());
            vec_guitar_current.push(Self {
                name: v_s.scale_type.get_name_short(),
                tonic: tonic.data_name().to_string(),
                display: Display::Horizontal,
                theme: ThemeIos::Dark,
                svg
            });
        }
        vec_guitar_current.into_iter().collect()
    }
}