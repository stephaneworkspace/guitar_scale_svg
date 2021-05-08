extern crate svg;
use std::str::FromStr;
use svg::node;
use svg::node::element::path::{Data, Number};
use svg::node::element::{Circle, Group, Path, Text};
use svg::Document;
use crate::scale::{Scale, DegreeAllStrings};
use crate::NUMBER_STRING;
use ukebox::PitchClass;
use crate::svg_draw::settings::Theme;
use crate::note::Note;
use crate::interval::Interval;

pub const WIDTH_LEFT: u16 = 50; // Place for root note (reverse A E C G)
pub const WIDTH_RIGHT: u16 = 25; // Blank
pub const WIDTH: u16 = 900 - WIDTH_LEFT - WIDTH_RIGHT;
pub const HEIGHT_TOP: u16 = 25;
pub const HEIGHT_BOTTOM: u16 = 45;
pub const HEIGHT: u16 = 250 - HEIGHT_TOP - HEIGHT_BOTTOM;
pub const NUMBER_POSITION: u16 = 19;
pub const OFFSET_TEXT_BOTTOM: Number = -1.0;
pub const THEME_BG_LIGHT: &str = "white";
pub const THEME_BG_DARK: &str = "#282c34";
pub const THEME_ITEM_LIGHT: &str = "black";
pub const THEME_ITEM_DARK: &str = "white";
pub const THEME_TEXT_COLOR_LIGHT: &str= "white";
pub const THEME_TEXT_COLOR_DARK: &str= "black";

pub const VER_HEIGHT_TOP: u16 = 50;
pub const VER_HEIGHT_BOTTOM: u16 = 25;
pub const VER_HEIGHT: u16 = 1100 - VER_HEIGHT_TOP - VER_HEIGHT_BOTTOM;
pub const VER_WIDTH_LEFT: u16 = 50;
pub const VER_WIDTH_RIGHT: u16 = 50;
pub const VER_WIDTH: u16 = 300 - VER_WIDTH_LEFT - VER_WIDTH_RIGHT;
pub const VER_OFFSET_WIDTH: f32 = 200.0 / 5.0;
pub const VER_FLUTTER_OFFSET_NOTE_NAME: Number = 4.75;

pub const SWIFT_WIDTH_OFFSET_NOTE_NAME: Number = -3.5;
pub const SWIFT_HEIGHT_OFFSET_NOTE_NAME: Number = 3.0;

pub struct DrawScale {
    pub scale: Scale,
    theme: Theme,
}

impl DrawScale {
    pub fn new(scale: Scale, theme: Theme) -> Self {
        Self { scale, theme }
    }

    /// Draw the svg
    pub fn draw_base(&self) -> Document {
        // Grid
        let style = match self.theme {
            Theme::Light => format!(
                "background: {}; stroke: {}; stroke-linecap: round;",
                THEME_BG_LIGHT, THEME_ITEM_LIGHT
            ),
            Theme::Dark => format!(
                "background: {}; stroke: {}; stroke-linecap: round;",
                THEME_BG_DARK, THEME_ITEM_DARK
            ),
        };
        let mut group_grid: Group =
            Group::new().set("style", style).set("class", "grid");
        let mut d: Data = Data::new();
        let width_pos: Number = WIDTH_LEFT as f32;
        let height_pos: Number = HEIGHT_TOP as f32;
        // All singles strings
        for n in 0..NUMBER_STRING {
            let h = (HEIGHT as Number / NUMBER_STRING as Number)
                * ((n + 1) as Number);
            d = d.move_to((width_pos, height_pos + h as Number)); // m
            d = d.horizontal_line_by((WIDTH, 0.0));
        }
        group_grid =
            group_grid.add(Path::new().set("class", "grid").set("d", d));
        d = Data::new();
        // Grid strings
        for n in 0..NUMBER_POSITION {
            let w: Number = (WIDTH as Number / NUMBER_POSITION as Number)
                * ((n + 1) as Number);
            let h = HEIGHT / NUMBER_STRING as u16;
            d = d.move_to((width_pos + w as Number, height_pos + h as Number)); // m
            d = d.vertical_line_by((HEIGHT - h, 0.0)); // v
        }
        group_grid =
            group_grid.add(Path::new().set("class", "grid").set("d", d));
        // Bottom point on fret
        let width_pos: Number = WIDTH_LEFT as f32;
        let height_pos: Number = HEIGHT_TOP as f32 + HEIGHT as f32;
        let mut group_text: Group = Group::new();
        let style = match self.theme {
            Theme::Light => {
                format!("font-family: Verdana; fill: {}", THEME_ITEM_LIGHT)
            },
            Theme::Dark => {
                format!("font-family: Verdana; fill: {}", THEME_ITEM_DARK)
            },
        };
        group_text = group_text
            .set("class", "text")
            .set("style", style.clone())
            .set("text-anchor", "middle");
        let vec_n = vec![1, 3, 5, 7, 10, 12, 15];
        for n in vec_n {
            let mut w: Number = WIDTH as f32 / NUMBER_POSITION as f32;
            w = w * (n as f32 + 0.5);
            let h = HEIGHT_BOTTOM / 2;
            if n.to_string().len() > 1 {
                group_text = group_text.add(
                    Text::new()
                        .set("class", "header")
                        .set("dominant-baseline", "middle")
                        .set("style", style.clone())
                        .set("x", width_pos + w as Number + SWIFT_WIDTH_OFFSET_NOTE_NAME)
                        .set("y", height_pos + h as Number + SWIFT_HEIGHT_OFFSET_NOTE_NAME)
                        .set("dx", OFFSET_TEXT_BOTTOM)
                        .add(node::Text::new(n.to_string())),
                );
            } else {
                group_text = group_text.add(
                    Text::new()
                        .set("class", "header")
                        .set("dominant-baseline", "middle")
                        .set("style", style.clone())
                        .set("x", width_pos + w as Number + SWIFT_WIDTH_OFFSET_NOTE_NAME)
                        .set("y", height_pos + h as Number + SWIFT_HEIGHT_OFFSET_NOTE_NAME)
                        .add(node::Text::new(n.to_string())),
                );
            }
        }
        // Circle note
        let mut group_circle_tonic: Group = Group::new();
        group_circle_tonic = group_circle_tonic
            .set("class", "circle_tonic")
            .set("fill", "coral");
        let mut group_circle_in_scale: Group = Group::new();
        let style = match self.theme {
            Theme::Light => THEME_ITEM_LIGHT,
            Theme::Dark => THEME_ITEM_DARK,
        };
        group_circle_in_scale = group_circle_in_scale
            .set("class", "circle_in_scale")
            .set("fill", style);
        let mut w: Number = WIDTH as f32 / NUMBER_POSITION as f32;
        let note_r: Number = w / 5.0;
        // Vector
        let vec_all_strings: Vec<DegreeAllStrings> =
            self.scale.get_string_combination();
        // Tonic
        for i in 0..NUMBER_STRING {
            let single_string = vec_all_strings[i as usize].clone();
            let height_pos: Number = HEIGHT_TOP as f32
                + ((HEIGHT as f32 / NUMBER_STRING as f32)
                * (self.guitar_string_convert(i) as f32 + 1.0));
            for j in 0..NUMBER_POSITION {
                for v in &single_string.degree_single_string {
                    if v.position == j as usize {
                        if v.sw_tonic.clone() {
                            let width_pos: Number = WIDTH_LEFT as f32;
                            w = WIDTH as f32 / NUMBER_POSITION as f32;
                            w = w * (j as f32 + 0.5);
                            group_circle_tonic = group_circle_tonic.add(
                                Circle::new()
                                    .set("cx", width_pos + w as Number)
                                    .set("cy", height_pos)
                                    .set("r", note_r),
                            );
                            break;
                        }
                    }
                }
            }
        }
        // In Scale
        for i in 0..NUMBER_STRING {
            let single_string = vec_all_strings[i as usize].clone();
            let height_pos: Number = HEIGHT_TOP as f32
                + ((HEIGHT as f32 / NUMBER_STRING as f32)
                * (self.guitar_string_convert(i) as f32 + 1.0));
            for j in 0..NUMBER_POSITION {
                for v in &single_string.degree_single_string {
                    if v.position == j as usize {
                        if !v.sw_tonic.clone() {
                            let width_pos: Number = WIDTH_LEFT as f32;
                            w = WIDTH as f32 / NUMBER_POSITION as f32;
                            w = w * (j as f32 + 0.5);
                            group_circle_in_scale = group_circle_in_scale.add(
                                Circle::new()
                                    .set("cx", width_pos + w as Number)
                                    .set("cy", height_pos)
                                    .set("r", note_r),
                            );
                            break;
                        }
                    }
                }
            }
        }
        // Text Tonic + In Scale
        let mut group_text_circle: Group = Group::new();
        /*let style = match self.theme {
            Theme::Light => THEME_BG_LIGHT,
            Theme::Dark => THEME_BG_DARK,
        };*/
        let style_color = match self.theme {
            Theme::Light => THEME_TEXT_COLOR_LIGHT,
            Theme::Dark => THEME_TEXT_COLOR_DARK,
        };
        group_text_circle = group_text_circle
            .set("class", "text_circle")
            .set("style", "font-family: Verdana;")
            .set("font-size", "9")
            .set("fill", style_color)
            .set("text-anchor", "middle");
        for i in 0..NUMBER_STRING {
            let single_string = vec_all_strings[i as usize].clone();
            let height_pos: Number = HEIGHT_TOP as f32
                + ((HEIGHT as f32 / NUMBER_STRING as f32)
                * (self.guitar_string_convert(i) as f32 + 1.0));
            for j in 0..NUMBER_POSITION {
                for v in &single_string.degree_single_string {
                    if v.position == j as usize {
                        let note = match v.note.pitch_class {
                            PitchClass::C => "C",
                            PitchClass::CSharp => "C#",
                            PitchClass::D => "D",
                            PitchClass::DSharp => "D#",
                            PitchClass::E => "E",
                            PitchClass::F => "F",
                            PitchClass::FSharp => "F#",
                            PitchClass::G => "G",
                            PitchClass::GSharp => "G#",
                            PitchClass::A => "A",
                            PitchClass::ASharp => "A#",
                            PitchClass::B => "B",
                        };
                        let width_pos: Number = WIDTH_LEFT as f32;
                        w = WIDTH as f32 / NUMBER_POSITION as f32;
                        w = w * (j as f32 + 0.5);
                        let style = match self.theme {
                            Theme::Light => {
                                format!("font-family: Verdana; fill: {}", THEME_TEXT_COLOR_LIGHT)
                            },
                            Theme::Dark => {
                                format!("font-family: Verdana; fill: {}", THEME_TEXT_COLOR_DARK)
                            },
                        };
                        if note.len() > 1 {
                            group_text_circle = group_text_circle.add(
                                Text::new()
                                    .set("dominant-baseline", "middle")
                                    .set("x", width_pos + w as Number + SWIFT_WIDTH_OFFSET_NOTE_NAME)
                                    .set("y", height_pos as Number + SWIFT_HEIGHT_OFFSET_NOTE_NAME)
                                    .set("dx", OFFSET_TEXT_BOTTOM)
                                    .set("style", style)
                                    .add(node::Text::new(note.to_string())),
                            );
                        } else {
                            group_text_circle = group_text_circle.add(
                                Text::new()
                                    .set("dominant-baseline", "middle")
                                    .set("x", width_pos + w as Number + SWIFT_WIDTH_OFFSET_NOTE_NAME)
                                    .set("y", height_pos as Number + SWIFT_HEIGHT_OFFSET_NOTE_NAME)
                                    .set("style", style)
                                    .add(node::Text::new(note.to_string())),
                            );
                        }
                    }
                }
            }
        }
        let mut group_text_left: Group = Group::new();
        let style = match self.theme {
            Theme::Light => {
                format!("font-family: Verdana; fill: {};", THEME_ITEM_LIGHT)
            },
            Theme::Dark => {
                format!("font-family: Verdana; fill: {};", THEME_ITEM_DARK)
            },
        };
        group_text_left = group_text_left
            .set("class", "text_circle")
            .set("style", style.clone())
            .set("text-anchor", "middle");
        let interval = self.scale.tuning.get_interval();
        let roots = [
            Note::from_str("E").unwrap() + interval,
            Note::from_str("A").unwrap() + interval,
            Note::from_str("D").unwrap() + interval + Interval::PlusOctave,
            Note::from_str("G").unwrap() + interval + Interval::PlusOctave,
            Note::from_str("B").unwrap() + interval + Interval::PlusOctave,
            Note::from_str("E").unwrap() + interval + Interval::PlusPlusOctave,
        ];
        for i in 0..NUMBER_STRING {
            let height_pos: Number = HEIGHT_TOP as f32
                + ((HEIGHT as f32 / NUMBER_STRING as f32)
                * (self.guitar_string_convert(i.clone()) as f32 + 1.0));
            let note = match roots[i as usize].clone().pitch_class {
                PitchClass::C => "C",
                PitchClass::CSharp => "C#",
                PitchClass::D => "D",
                PitchClass::DSharp => "D#",
                PitchClass::E => "E",
                PitchClass::F => "F",
                PitchClass::FSharp => "F#",
                PitchClass::G => "G",
                PitchClass::GSharp => "G#",
                PitchClass::A => "A",
                PitchClass::ASharp => "A#",
                PitchClass::B => "B",
            };
            let width_pos: Number = WIDTH_LEFT as f32 / 2.0;
            if note.len() > 1 {
                group_text_left = group_text_left.add(
                    Text::new()
                        .set("dominant-baseline", "middle")
                        .set("x", width_pos as Number)
                        .set("y", height_pos as Number)
                        .set("dx", OFFSET_TEXT_BOTTOM)
                        .set("style", style.clone())
                        .add(node::Text::new(note.to_string())),
                );
            } else {
                group_text_left = group_text_left.add(
                    Text::new()
                        .set("dominant-baseline", "middle")
                        .set("x", width_pos as Number)
                        .set("y", height_pos as Number)
                        .set("style", style.clone())
                        .add(node::Text::new(note.to_string())),
                );
            }
        }
        // End
        let style = match self.theme {
            Theme::Light => format!("background: {};", THEME_BG_LIGHT),
            Theme::Dark => format!("background: {};", THEME_BG_DARK),
        };
        let document = Document::new()
            .set("class", "chord-chart")
            .set("xmlns", "http://www.w3.org/2000/svg")
            .set("width", WIDTH_LEFT + WIDTH + WIDTH_RIGHT)
            .set("height", HEIGHT_TOP + HEIGHT + HEIGHT_BOTTOM)
            .set("preserveAspectRatio", "xMidYMid mee    t")
            .set("font-size", 16.0)
            .set("style", style)
            .set(
                "viewBox",
                (
                    0,
                    0,
                    WIDTH as i32 + WIDTH_LEFT as i32 + WIDTH_RIGHT as i32,
                    HEIGHT as i32 + HEIGHT_TOP as i32 + HEIGHT_BOTTOM as i32,
                ),
            )
            .add(group_grid)
            .add(group_text)
            .add(group_circle_tonic)
            .add(group_circle_in_scale)
            .add(group_text_circle)
            .add(group_text_left);
        document
    }

    /// Draw the svg
    pub fn draw_base_vertical(&self) -> Document {
        // Grid
        let style = match self.theme {
            Theme::Light => format!(
                "background: {}; stroke: {}; stroke-linecap: round;",
                THEME_BG_LIGHT, THEME_ITEM_LIGHT
            ),
            Theme::Dark => format!(
                "background: {}; stroke: {}; stroke-linecap: round;",
                THEME_BG_DARK, THEME_ITEM_DARK
            ),
        };
        let mut group_grid: Group = Group::new()
            .set("style", style.to_string())
            .set("class", "grid");
        let mut d: Data = Data::new();
        let width_pos: Number = VER_WIDTH_LEFT as f32;
        let height_pos: Number = VER_HEIGHT_TOP as f32;
        // let width_pos: Number = WIDTH_LEFT as f32;
        // let height_pos: Number = HEIGHT_TOP as f32;
        // All singles strings
        for n in 0..NUMBER_STRING {
            let nn: Number = (n) as Number;
            let w = VER_OFFSET_WIDTH as Number * nn;
            d = d.move_to((width_pos + w, height_pos as Number)); // m
            d = d.vertical_line_by((VER_HEIGHT, 0.0));
            //let h = (HEIGHT as Number / NUMBER_STRING as Number)
            //    * ((n + 1) as Number);
            //d = d.move_to((width_pos, height_pos + h as Number)); // m
            //d = d.horizontal_line_by((WIDTH, 0.0));
        }
        group_grid =
            group_grid.add(Path::new().set("class", "grid").set("d", d));
        d = Data::new();
        // Grid strings
        for n in 0..NUMBER_POSITION {
            let h: Number = (VER_HEIGHT as Number / NUMBER_POSITION as Number)
                * ((n + 1) as Number);
            let w = VER_WIDTH;
            d = d.move_to((width_pos, height_pos + h as Number)); // m
            d = d.horizontal_line_by((w as f32, 0.0)); // v

            //let w: Number = (WIDTH as Number / NUMBER_POSITION as Number)
            //    * ((n + 1) as Number);
            //let h = HEIGHT / NUMBER_STRING as u16;
            //d = d.move_to((width_pos + w as Number, height_pos + h as Number)); // m
            //d = d.vertical_line_by((HEIGHT - h, 0.0)); // v
        }
        group_grid =
            group_grid.add(Path::new().set("class", "grid").set("d", d));
        // Bottom point on fret
        let width_pos: Number = VER_WIDTH_LEFT as f32 + VER_WIDTH as f32;
        let height_pos: Number = VER_HEIGHT_TOP as f32;
        //let width_pos: Number = WIDTH_LEFT as f32;
        //let height_pos: Number = HEIGHT_TOP as f32 + HEIGHT as f32;
        let mut group_text: Group = Group::new();
        let style = match self.theme {
            Theme::Light => {
                format!("font-family: Verdana; fill: {};", THEME_ITEM_LIGHT)
            },
            Theme::Dark => {
                format!("font-family: Verdana; fill: {};", THEME_ITEM_DARK)
            },
        };
        group_text = group_text
            .set("class", "text")
            .set("style", style.clone())
            .set("text-anchor", "middle");
        let vec_n = vec![1, 3, 5, 7, 10, 12, 15];
        for n in vec_n {
            let mut h: Number = VER_HEIGHT as f32 / NUMBER_POSITION as f32;
            h = h * (n as f32 + 0.5);
            let w = VER_WIDTH_RIGHT / 2;

            //let mut w: Number = WIDTH as f32 / NUMBER_POSITION as f32;
            //w = w * (n as f32 + 0.5);
            //let h = HEIGHT_BOTTOM / 2;
            if n.to_string().len() > 1 {
                group_text = group_text.add(
                    Text::new()
                        .set("class", "header")
                        .set("dominant-baseline", "middle")
                        .set("style", style.clone())
                        .set("x", width_pos + w as Number)
                        .set(
                            "y",
                            height_pos
                                + h as Number
                                + VER_FLUTTER_OFFSET_NOTE_NAME,
                        )
                        .set("dx", OFFSET_TEXT_BOTTOM)
                        .add(node::Text::new(n.to_string())),
                );
            } else {
                group_text = group_text.add(
                    Text::new()
                        .set("class", "header")
                        .set("dominant-baseline", "middle")
                        .set("style", style.clone())
                        .set("x", width_pos + w as Number)
                        .set(
                            "y",
                            height_pos
                                + h as Number
                                + VER_FLUTTER_OFFSET_NOTE_NAME,
                        )
                        .add(node::Text::new(n.to_string())),
                );
            }
        }
        // Circle note
        let mut group_circle_tonic: Group = Group::new();
        group_circle_tonic = group_circle_tonic
            .set("class", "circle_tonic")
            .set("fill", "coral");
        let mut group_circle_in_scale: Group = Group::new();
        let style = match self.theme {
            Theme::Light => THEME_ITEM_LIGHT,
            Theme::Dark => THEME_ITEM_DARK,
        };
        group_circle_in_scale = group_circle_in_scale
            .set("class", "circle_in_scale")
            .set("fill", style);
        let mut h: Number = VER_HEIGHT as f32 / NUMBER_POSITION as f32;
        // let mut w: Number = WIDTH as f32 / NUMBER_POSITION as f32;
        let note_r: Number = h / 3.5;
        //let note_r: Number = w / 5.0;
        // Vector
        let vec_all_strings: Vec<DegreeAllStrings> =
            self.scale.get_string_combination();
        // Tonic
        for i in 0..NUMBER_STRING {
            let single_string = vec_all_strings[i as usize].clone();
            let width_pos: Number = VER_WIDTH_LEFT as f32
                + ((VER_WIDTH as f32 / (NUMBER_STRING - 1) as f32) * i as f32
                + 0.0);
            //let height_pos: Number = HEIGHT_TOP as f32
            //    + ((HEIGHT as f32 / NUMBER_STRING as f32)
            //        * (self.ukulele_string_convert(i) as f32 + 1.0));
            for j in 0..NUMBER_POSITION {
                for v in &single_string.degree_single_string {
                    if v.position == j as usize {
                        if v.sw_tonic.clone() {
                            let height_pos: Number = VER_HEIGHT_TOP as f32;
                            h = VER_HEIGHT as f32 / NUMBER_POSITION as f32;
                            h = h * (j as f32 + 0.5);
                            // let width_pos: Number = WIDTH_LEFT as f32;
                            // w = WIDTH as f32 / NUMBER_POSITION as f32;
                            // w = w * (j as f32 + 0.5);
                            group_circle_tonic = group_circle_tonic.add(
                                Circle::new()
                                    .set("cx", width_pos)
                                    .set("cy", height_pos + h as Number)
                                    .set("r", note_r),
                            );
                            break;
                        }
                    }
                }
            }
        }
        // In Scale
        for i in 0..NUMBER_STRING {
            let single_string = vec_all_strings[i as usize].clone();
            let width_pos: Number = VER_WIDTH_LEFT as f32
                + ((VER_WIDTH as f32 / (NUMBER_STRING - 1) as f32) * i as f32
                + 0.0);
            //let height_pos: Number = HEIGHT_TOP as f32
            //    + ((HEIGHT as f32 / NUMBER_STRING as f32)
            //        * (self.ukulele_string_convert(i) as f32 + 1.0));
            for j in 0..NUMBER_POSITION {
                for v in &single_string.degree_single_string {
                    if v.position == j as usize {
                        if !v.sw_tonic.clone() {
                            let height_pos: Number = VER_HEIGHT_TOP as f32;
                            h = VER_HEIGHT as f32 / NUMBER_POSITION as f32;
                            h = h * (j as f32 + 0.5);
                            //let width_pos: Number = WIDTH_LEFT as f32;
                            //w = WIDTH as f32 / NUMBER_POSITION as f32;
                            //w = w * (j as f32 + 0.5);
                            group_circle_in_scale = group_circle_in_scale.add(
                                Circle::new()
                                    .set("cx", width_pos)
                                    .set("cy", height_pos + h as Number)
                                    .set("r", note_r),
                            );
                            break;
                        }
                    }
                }
            }
        }
        // Text Tonic + In Scale
        let mut group_text_circle: Group = Group::new();
        /*let style = match self.theme {
            Theme::Light => THEME_BG_LIGHT,
            Theme::Dark => THEME_BG_DARK,
        };*/
        let style_color = match self.theme {
            Theme::Light => THEME_TEXT_COLOR_LIGHT,
            Theme::Dark => THEME_TEXT_COLOR_DARK,
        };
        group_text_circle = group_text_circle
            .set("class", "text_circle")
            .set("style", "font-family: Verdana;")
            .set("font-size", "14")
            .set("fill", style_color)
            .set("text-anchor", "middle");
        for i in 0..NUMBER_STRING {
            let single_string = vec_all_strings[i as usize].clone();
            let width_pos: Number = VER_WIDTH_LEFT as f32
                + ((VER_OFFSET_WIDTH as f32)
//                + ((VER_WIDTH as f32 / NUMBER_STRING as f32)
                * i as f32
                + 0.0);
            // let height_pos: Number = HEIGHT_TOP as f32
            //    + ((HEIGHT as f32 / NUMBER_STRING as f32)
            //        * (self.ukulele_string_convert(i) as f32 + 1.0));
            for j in 0..NUMBER_POSITION {
                for v in &single_string.degree_single_string {
                    if v.position == j as usize {
                        let note = match v.note.pitch_class {
                            PitchClass::C => "C",
                            PitchClass::CSharp => "C#",
                            PitchClass::D => "D",
                            PitchClass::DSharp => "D#",
                            PitchClass::E => "E",
                            PitchClass::F => "F",
                            PitchClass::FSharp => "F#",
                            PitchClass::G => "G",
                            PitchClass::GSharp => "G#",
                            PitchClass::A => "A",
                            PitchClass::ASharp => "A#",
                            PitchClass::B => "B",
                        };
                        let height_pos: Number = VER_HEIGHT_TOP as f32;
                        h = VER_HEIGHT as f32 / NUMBER_POSITION as f32;
                        h = h * (j as f32 + 0.5);
                        //let width_pos: Number = WIDTH_LEFT as f32;
                        //w = WIDTH as f32 / NUMBER_POSITION as f32;
                        //w = w * (j as f32 + 0.5);
                        let style = match self.theme {
                            Theme::Light => {
                                format!("font-family: Verdana; fill: {}", THEME_TEXT_COLOR_LIGHT)
                            },
                            Theme::Dark => {
                                format!("font-family: Verdana; fill: {}", THEME_TEXT_COLOR_DARK)
                            },
                        };
                        if note.len() > 1 {
                            group_text_circle = group_text_circle.add(
                                Text::new()
                                    .set("dominant-baseline", "middle")
                                    .set("x", width_pos)
                                    .set(
                                        "y",
                                        height_pos
                                            + h as Number
                                            + VER_FLUTTER_OFFSET_NOTE_NAME
                                            as Number,
                                    )
                                    .set("dx", OFFSET_TEXT_BOTTOM)
                                    .set("style", style)
                                    .add(node::Text::new(note.to_string())),
                            );
                        } else {
                            group_text_circle = group_text_circle.add(
                                Text::new()
                                    .set("dominant-baseline", "middle")
                                    .set("x", width_pos)
                                    .set(
                                        "y",
                                        height_pos
                                            + h as Number
                                            + VER_FLUTTER_OFFSET_NOTE_NAME
                                            as Number,
                                    )
                                    .set("style", style)
                                    .add(node::Text::new(note.to_string())),
                            );
                        }
                    }
                }
            }
        }
        let mut group_text_left: Group = Group::new();
        let style = match self.theme {
            Theme::Light => {
                format!("font-family: Verdana; fill: {};", THEME_ITEM_LIGHT)
            },
            Theme::Dark => {
                format!("font-family: Verdana; fill: {};", THEME_ITEM_DARK)
            },
        };
        group_text_left = group_text_left
            .set("class", "text_circle")
            .set("style", style.clone())
            .set("text-anchor", "middle");
        let interval = self.scale.tuning.get_interval();
        let roots = [
            Note::from_str("E").unwrap() + interval,
            Note::from_str("A").unwrap() + interval,
            Note::from_str("D").unwrap() + interval + Interval::PlusOctave,
            Note::from_str("G").unwrap() + interval + Interval::PlusOctave,
            Note::from_str("B").unwrap() + interval + Interval::PlusOctave,
            Note::from_str("E").unwrap() + interval + Interval::PlusPlusOctave,
        ];
        for i in 0..NUMBER_STRING {
            let width_pos: Number = VER_WIDTH_LEFT as f32
                + ((VER_OFFSET_WIDTH as f32)
                //  + ((VER_WIDTH as f32 / NUMBER_STRING as f32)
                * i.clone() as f32
                + 0.0);
            // let height_pos: Number = HEIGHT_TOP as f32
            //    + ((HEIGHT as f32 / NUMBER_STRING as f32)
            //        * (self.ukulele_string_convert(i.clone()) as f32 + 1.0));
            let note = match roots[i as usize].clone().pitch_class {
                PitchClass::C => "C",
                PitchClass::CSharp => "C#",
                PitchClass::D => "D",
                PitchClass::DSharp => "D#",
                PitchClass::E => "E",
                PitchClass::F => "F",
                PitchClass::FSharp => "F#",
                PitchClass::G => "G",
                PitchClass::GSharp => "G#",
                PitchClass::A => "A",
                PitchClass::ASharp => "A#",
                PitchClass::B => "B",
            };
            let height_pos: Number = VER_HEIGHT_TOP as f32 / 2.0;
            // let width_pos: Number = WIDTH_LEFT as f32 / 2.0;
            if note.len() > 1 {
                group_text_left = group_text_left.add(
                    Text::new()
                        .set("dominant-baseline", "middle")
                        .set("x", width_pos as Number)
                        .set("y", height_pos as Number)
                        .set("dx", OFFSET_TEXT_BOTTOM)
                        .set("style", style.clone())
                        .add(node::Text::new(note.to_string())),
                );
            } else {
                group_text_left = group_text_left.add(
                    Text::new()
                        .set("dominant-baseline", "middle")
                        .set("x", width_pos as Number)
                        .set("y", height_pos as Number)
                        .set("style", style.clone())
                        .add(node::Text::new(note.to_string())),
                );
            }
        }
        // End
        let style = match self.theme {
            Theme::Light => format!("background: {};", THEME_BG_LIGHT),
            Theme::Dark => format!("background: {};", THEME_BG_DARK),
        };
        let document = Document::new()
            .set("class", "chord-chart")
            .set("xmlns", "http://www.w3.org/2000/svg")
            .set("width", VER_WIDTH_LEFT + VER_WIDTH + VER_WIDTH_RIGHT)
            .set("height", VER_HEIGHT_TOP + VER_HEIGHT + VER_HEIGHT_BOTTOM)
            .set("preserveAspectRatio", "xMidYMid mee    t")
            .set("font-size", 16.0)
            .set("style", style)
            .set(
                "viewBox",
                (
                    0,
                    0,
                    VER_WIDTH as i32
                        + VER_WIDTH_LEFT as i32
                        + VER_WIDTH_RIGHT as i32,
                    VER_HEIGHT as i32
                        + VER_HEIGHT_TOP as i32
                        + VER_HEIGHT_BOTTOM as i32,
                ),
            )
            .add(group_grid)
            .add(group_text)
            .add(group_circle_tonic)
            .add(group_circle_in_scale)
            .add(group_text_circle)
            .add(group_text_left);
        document
    }
    ///
    /// # Arguments
    ///
    /// * `string` - ukulele string to invert
    fn guitar_string_convert(&self, string: u8) -> u8 {
        match string {
            0 => 5,
            1 => 4,
            2 => 3,
            3 => 2,
            4 => 1,
            5 => 0,
            _ => 0,
        }
    }
}