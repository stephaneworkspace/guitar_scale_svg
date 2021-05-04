use guitar_scale_svg::note::{Note, TraitGenerate};
use guitar_scale_svg::scale::Scale;
use guitar_scale_svg::scale::ScaleType;
use std::str::FromStr;
use guitar_scale_svg::svg_draw::{Theme, DrawScale};
use std::fs::File;
use guitar_scale_svg::tuning::Tuning;
use strum::IntoEnumIterator;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let vec_note = ["C","C#","D","D#","E","F","F#","G","G#","A","A#","B"];
    for x in vec_note.to_vec().iter() {
        note(*x)?;
    }
    Ok(())
}

fn note(note: &str) -> std::io::Result<()> {
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
    for v_s in vec_scale {
        let draw: DrawScale = DrawScale::new(v_s, Theme::Light);
        let svg = format!("{}",draw.draw_base());
        let mut buffer = File::create(format!("temp/scale_light_{}_{}.svg", v_s.scale_type.get_name_short(), &tonic.data_name()))?;
        buffer.write_all(&svg.as_bytes())?;
        let draw: DrawScale = DrawScale::new(v_s, Theme::Dark);
        let svg = format!("{}",draw.draw_base());
        let mut buffer = File::create(format!("temp/scale_dark_{}_{}.svg", v_s.scale_type.get_name_short(), &tonic.data_name()))?;
        buffer.write_all(&svg.as_bytes())?;
    }
    Ok(())
}