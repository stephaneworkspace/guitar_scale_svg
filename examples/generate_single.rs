use guitar_scale_svg_vertical::svg_draw::{DrawScale, Theme};
use guitar_scale_svg_vertical::scale::{Scale, ScaleType};
use std::str::FromStr;
use guitar_scale_svg_vertical::note::Note;
use guitar_scale_svg_vertical::tuning::Tuning;
use std::fs::File;
use std::fmt::format;

fn main() {
    let tonic = match Note::from_str("E") {
        Ok(ok) => ok,
        Err(err) => {
            panic!("{}", err)
        }
    };
    let scale = Scale {
      scale_type: ScaleType::Major,
        tuning: Tuning::E,
        tonic
    };
    let draw: DrawScale = DrawScale::new(scale, Theme::Light);
    //println!("Hello World !");
    let svg = draw.draw_base();
    println!("{}", svg);
}

}