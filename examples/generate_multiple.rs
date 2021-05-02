use guitar_scale_svg::note::Note;
use guitar_scale_svg::scale::Scale;
use guitar_scale_svg::scale::ScaleType;
use std::str::FromStr;
use guitar_scale_svg::svg_draw::{Theme, DrawScale};
use std::fs::File;
use guitar_scale_svg::tuning::Tuning;
use strum::IntoEnumIterator;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let tonic = match Note::from_str("E") {
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
        let mut buffer = File::create(format!("temp/scale_light_{}_E.svg", v_s.scale_type.get_name_short()))?;
        buffer.write_all(&svg.as_bytes())?;
        let draw: DrawScale = DrawScale::new(v_s, Theme::Dark);
        let svg = format!("{}",draw.draw_base());
        let mut buffer = File::create(format!("temp/scale_dark_{}_E.svg", v_s.scale_type.get_name_short()))?;
        buffer.write_all(&svg.as_bytes())?;
    }
    Ok(())
}