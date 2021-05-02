use ukebox::pitch_class::PitchClass;
use ukebox::Semitones;
use crate::Degree;
use crate::NUMBER_STRING;
use std::fmt;
use std::str::FromStr;
use strum::IntoEnumIterator;
use crate::note::{TraitNoteSemitones, Note};
use crate::tuning::Tuning;
use crate::interval::Interval;

/// Number of pitch classes.
const PITCH_CLASS_COUNT: Semitones = 12;

// Custom error for strings that cannot be parsed into notes.
#[derive(Debug)]
pub struct ParseScaleError {
    pub name: String,
}

impl fmt::Display for ParseScaleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse scale \"{}\"", self.name)
    }
}

#[derive(Debug, Clone)]
pub struct DegreeAllStrings {
    pub string_number: usize,
    pub degree_single_string: Vec<DegreeSingleString>,
}

/// Degree on the fret
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DegreeSingleString {
    pub position: usize, // 0..19
    pub sw_tonic: bool,
    pub note: Note,
}

/// The type of scale
#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
pub enum ScaleType {
    Major,
    Minor,
    Dorian,
    Mixolydian,
    Lydian,
    PhrygianMinor,
    Phrygian,
    Locrian,
    WholeTone,
    HalfWholeDiminished,
    WholeHalfDiminished,
    MinorBlues,
    MinorPentagonic,
    MajorPentagonic,
    HarmonicMinor,
    HarmonicMajor,
    Dorian4,
    PhrygianDominant,
    MelodicMinor,
    LydianAugumented,
    LydianDominant,
    SuperLocrian,
    _8TonesSpanish,
    Bhairav,
    HungarianMinor,
    Hirajoshi,
    InSen,
    Iwato,
    Kumoi,
    PelogSelisir,
    PelogTembung,
    Messiaen3,
    Messiaen4,
    Messiaen5,
    Messiaen6,
    Messiaen7,
}

pub struct ScaleTypeSelect {
    pub short: String,
    pub long: String,
    pub note_scale_c: Vec<String>,
    pub note_scale_c_sharp: Vec<String>,
    pub note_scale_d: Vec<String>,
    pub note_scale_d_sharp: Vec<String>,
    pub note_scale_e: Vec<String>,
    pub note_scale_f: Vec<String>,
    pub note_scale_f_sharp: Vec<String>,
    pub note_scale_g: Vec<String>,
    pub note_scale_g_sharp: Vec<String>,
    pub note_scale_a: Vec<String>,
    pub note_scale_a_sharp: Vec<String>,
    pub note_scale_b: Vec<String>,
}

impl ScaleType {
    /// Degree from the tonic 1..12
    fn get_degree_from_tonic(self) -> Vec<Degree> {
        use ScaleType::*;
        let degree = match self {
            Major => vec![1, 3, 5, 6, 8, 10, 12],
            Minor => vec![1, 3, 4, 6, 8, 9, 11],
            Dorian => vec![1, 3, 4, 6, 8, 10, 11],
            Mixolydian => vec![1, 3, 5, 6, 8, 10, 11],
            Lydian => vec![1, 3, 5, 7, 8, 10, 12],
            PhrygianMinor => vec![1, 2, 3, 6, 8, 9, 11],
            Phrygian => vec![1, 2, 4, 6, 8, 9, 11],
            Locrian => vec![1, 2, 4, 6, 7, 9, 11],
            WholeTone => vec![1, 3, 5, 7, 9, 11],
            HalfWholeDiminished => vec![1, 2, 4, 5, 7, 8, 10, 11],
            WholeHalfDiminished => vec![1, 3, 4, 6, 7, 9, 10, 12],
            MinorBlues => vec![1, 4, 6, 7, 8, 11],
            MinorPentagonic => vec![1, 4, 6, 7, 8, 11],
            MajorPentagonic => vec![1, 4, 6, 8, 11],
            HarmonicMinor => vec![1, 3, 4, 6, 8, 9, 12],
            HarmonicMajor => vec![1, 3, 5, 6, 8, 9, 12],
            Dorian4 => vec![1, 3, 4, 7, 8, 10, 11],
            PhrygianDominant => vec![1, 2, 5, 6, 8, 9, 11],
            MelodicMinor => vec![1, 3, 4, 6, 8, 10, 12],
            LydianAugumented => vec![1, 3, 5, 7, 9, 10, 12],
            LydianDominant => vec![1, 3, 5, 7, 8, 10, 11],
            SuperLocrian => vec![1, 2, 4, 5, 7, 9, 11],
            _8TonesSpanish => vec![1, 2, 4, 5, 6, 7, 9, 11],
            Bhairav => vec![1, 2, 5, 6, 8, 9, 12],
            HungarianMinor => vec![1, 3, 4, 7, 8, 9, 12],
            Hirajoshi => vec![1, 3, 4, 8, 9, 12],
            InSen => vec![1, 2, 6, 8, 11],
            Iwato => vec![1, 2, 6, 7, 11],
            Kumoi => vec![1, 3, 4, 8, 10],
            PelogSelisir => vec![1, 2, 4, 8, 9],
            PelogTembung => vec![1, 2, 6, 8, 9],
            Messiaen3 => vec![1, 3, 4, 5, 7, 8, 9, 11, 12],
            Messiaen4 => vec![1, 2, 3, 6, 7, 8, 9, 12],
            Messiaen5 => vec![1, 2, 6, 7, 8, 12],
            Messiaen6 => vec![1, 3, 5, 6, 7, 9, 11, 12],
            Messiaen7 => vec![1, 2, 3, 4, 6, 7, 8, 9, 10, 12],
        };
        degree
    }

    /// Name short
    pub fn get_name_short(self) -> String {
        use ScaleType::*;
        let s = match self {
            Major => "major",
            Minor => "minor",
            Dorian => "dorian",
            Mixolydian => "mixolydian",
            Lydian => "lydian",
            PhrygianMinor => "phrygian_minor",
            Phrygian => "phrygian",
            Locrian => "locrian",
            WholeTone => "whole_tone",
            HalfWholeDiminished => "half_whole_diminished",
            WholeHalfDiminished => "whole_half_diminished",
            MinorBlues => "minor_blues",
            MinorPentagonic => "minor_pentagonic",
            MajorPentagonic => "maj_pentagonic",
            HarmonicMinor => "harmonic_minor",
            HarmonicMajor => "harmonic_major",
            Dorian4 => "dorian_4",
            PhrygianDominant => "phrygian_dominant",
            MelodicMinor => "melodic_minor",
            LydianAugumented => "lydian_augmented",
            LydianDominant => "lydian_dominant",
            SuperLocrian => "super_locrian",
            _8TonesSpanish => "8_tones_spanish",
            Bhairav => "bhairav",
            HungarianMinor => "hungarian_minor",
            Hirajoshi => "hirajoshi",
            InSen => "in_sen",
            Iwato => "iwato",
            Kumoi => "kumoi",
            PelogSelisir => "pelog_selisir",
            PelogTembung => "pelog_tembung",
            Messiaen3 => "messiaen_3",
            Messiaen4 => "messiaen_4",
            Messiaen5 => "messiaen_5",
            Messiaen6 => "messiaen_6",
            Messiaen7 => "messiaen_7",
        };
        s.to_string()
    }

    /// Name
    fn get_name_long(self) -> String {
        use ScaleType::*;
        let s = match self {
            Major => "Major",
            Minor => "Minor",
            Dorian => "Dorian",
            Mixolydian => "Mixolydian",
            Lydian => "Lydian",
            PhrygianMinor => "Phrygian Minor",
            Phrygian => "Phrygian Major",
            Locrian => "Locrian",
            WholeTone => "Whole Tone",
            HalfWholeDiminished => "Half-whole Diminished",
            WholeHalfDiminished => "Whole-half Diminished",
            MinorBlues => "Minor Blues",
            MinorPentagonic => "Minor Pentagonic",
            MajorPentagonic => "Major Pentagonic",
            HarmonicMinor => "Harmonic Minor",
            HarmonicMajor => "Harmonic Major",
            Dorian4 => "Dorian #4",
            PhrygianDominant => "Phrygian Dominant",
            MelodicMinor => "Melodic Minor",
            LydianAugumented => "Lydian Augumented",
            LydianDominant => "Lydian Dominant",
            SuperLocrian => "Super Locrian",
            _8TonesSpanish => "8-Tones Spanish",
            Bhairav => "Bhairav",
            HungarianMinor => "Hungarian Minor",
            Hirajoshi => "Hirajoshi",
            InSen => "In-Sen",
            Iwato => "Iwato",
            Kumoi => "Kumoi",
            PelogSelisir => "Pelog Selisir",
            PelogTembung => "Pelog Tembung",
            Messiaen3 => "Messiaen 3",
            Messiaen4 => "Messiaen 4",
            Messiaen5 => "Messiaen 5",
            Messiaen6 => "Messiaen 6",
            Messiaen7 => "Messiaen 7",
        };
        s.to_string()
    }

    /// All scale know by the lib
    pub fn get_all_scale(self, tuning: Tuning) -> Vec<ScaleTypeSelect> {
        let mut vec_scale: Vec<ScaleTypeSelect> = Vec::new();
        for c in ScaleType::iter() {
            let short = c.clone().get_name_short();
            let long = c.clone().get_name_long();
            let degree: Vec<Degree> = c.clone().get_degree_from_tonic();

            let semitones = tuning.get_semitones();

            let note_scale_c: Vec<String> = degree
                .iter()
                .map(|x| Note::from_semitones(semitones + (x - 1)).to_string())
                .collect();
            let note_scale_c_sharp: Vec<String> = degree
                .iter()
                .map(|x| {
                    Note::from_semitones(semitones + 1 + (x - 1)).to_string()
                })
                .collect();
            let note_scale_d: Vec<String> = degree
                .iter()
                .map(|x| {
                    Note::from_semitones(semitones + 2 + (x - 1)).to_string()
                })
                .collect();
            let note_scale_d_sharp: Vec<String> = degree
                .iter()
                .map(|x| {
                    Note::from_semitones(semitones + 3 + (x - 1)).to_string()
                })
                .collect();
            let note_scale_e: Vec<String> = degree
                .iter()
                .map(|x| {
                    Note::from_semitones(semitones + 4 + (x - 1)).to_string()
                })
                .collect();
            let note_scale_f: Vec<String> = degree
                .iter()
                .map(|x| {
                    Note::from_semitones(semitones + 5 + (x - 1)).to_string()
                })
                .collect();
            let note_scale_f_sharp: Vec<String> = degree
                .iter()
                .map(|x| {
                    Note::from_semitones(semitones + 6 + (x - 1)).to_string()
                })
                .collect();
            let note_scale_g: Vec<String> = degree
                .iter()
                .map(|x| {
                    Note::from_semitones(semitones + 7 + (x - 1)).to_string()
                })
                .collect();
            let note_scale_g_sharp: Vec<String> = degree
                .iter()
                .map(|x| {
                    Note::from_semitones(semitones + 8 + (x - 1)).to_string()
                })
                .collect();
            let note_scale_a: Vec<String> = degree
                .iter()
                .map(|x| {
                    Note::from_semitones(semitones + 9 + (x - 1)).to_string()
                })
                .collect();
            let note_scale_a_sharp: Vec<String> = degree
                .iter()
                .map(|x| {
                    Note::from_semitones(semitones + 10 + (x - 1)).to_string()
                })
                .collect();
            let note_scale_b: Vec<String> = degree
                .iter()
                .map(|x| {
                    Note::from_semitones(semitones + 11 + (x - 1)).to_string()
                })
                .collect();

            let scale_type_select = ScaleTypeSelect {
                short,
                long,
                note_scale_c,
                note_scale_c_sharp,
                note_scale_d,
                note_scale_d_sharp,
                note_scale_e,
                note_scale_f,
                note_scale_f_sharp,
                note_scale_g,
                note_scale_g_sharp,
                note_scale_a,
                note_scale_a_sharp,
                note_scale_b,
            };
            vec_scale.push(scale_type_select);
        }
        vec_scale
    }
}

impl FromStr for ScaleType {
    type Err = ParseScaleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ScaleType::*;

        let name = s.to_string();

        let scale: Self = match s {
            "major" => Major,
            "minor" => Minor,
            "dorian" => Dorian,
            "mixolydian" => Mixolydian,
            "lydian" => Lydian,
            "phrygian_min" => PhrygianMinor,
            "phrygian" => Phrygian,
            "locrian" => Locrian,
            "whole_tone" => WholeTone,
            "half_whole_diminished" => HalfWholeDiminished,
            "whole_half_diminished" => WholeHalfDiminished,
            "minor_blues" => MinorBlues,
            "minor_pentagonic" => MinorPentagonic,
            "maj_pentagonic" => MajorPentagonic,
            "harmonic_minor" => HarmonicMinor,
            "harmonic_major" => HarmonicMajor,
            "dorian_4" => Dorian4,
            "phrygian_dominant" => PhrygianDominant,
            "melodic_minor" => MelodicMinor,
            "lydian_augmented" => LydianAugumented,
            "lydian_dominant" => LydianDominant,
            "super_locrian" => SuperLocrian,
            "8_tones_spanish" => _8TonesSpanish,
            "bhairav" => Bhairav,
            "hungarian_minor" => HungarianMinor,
            "hirajoshi" => Hirajoshi,
            "in_sen" => InSen,
            "iwato" => Iwato,
            "kumoi" => Kumoi,
            "pelog_selisir" => PelogSelisir,
            "pelog_tembung" => PelogTembung,
            "messiaen_3" => Messiaen3,
            "messiaen_4" => Messiaen4,
            "messiaen_5" => Messiaen5,
            "messiaen_6" => Messiaen6,
            "messiaen_7" => Messiaen7,
            _ => return Err(ParseScaleError { name }),
        };

        Ok(scale)
    }
}

/// Only one octave, the Vector begin on the tonic
#[derive(Debug, Clone, Copy)]
pub struct Scale {
    pub scale_type: ScaleType,
    pub tuning: Tuning,
    pub tonic: Note, // .pitch_class: PitchClass
}

impl Scale {
    /// Get semitones from tonic in this Struct
    /// Semitones + bool if tonic
    fn get_degree(self) -> Vec<(Semitones, bool)> {
        use PitchClass::*;
        let degree: Vec<Degree> = self.scale_type.get_degree_from_tonic();

        // Get semittones

        // There does not seem to be a good way to turn integers into enum
        // variants without using external crates. Hardcoding the mapping
        // is not so elegant but at least readable.
        let pc_semitones: Semitones = match self.tonic.pitch_class {
            C => 0,
            CSharp => 1,
            D => 2,
            DSharp => 3,
            E => 4,
            F => 5,
            FSharp => 6,
            G => 7,
            GSharp => 8,
            A => 9,
            ASharp => 10,
            B => 11,
        };

        let mut vec_pitch_class: Vec<PitchClass> = Vec::new();
        for d in degree {
            let mut degree_compute = (d - 1) + &pc_semitones; // d - 1 because
            // degree start at
            // 1, not 0

            // Make sure we get a value between 0 and 11.
            degree_compute = degree_compute % PITCH_CLASS_COUNT;

            // There does not seem to be a good way to turn integers into enum
            // variants without using external crates. Hardcoding the mapping
            // is not so elegant but at least readable.
            let temp = match degree_compute {
                0 => C,
                1 => CSharp,
                2 => D,
                3 => DSharp,
                4 => E,
                5 => F,
                6 => FSharp,
                7 => G,
                8 => GSharp,
                9 => A,
                10 => ASharp,
                11 => B,
                // Because of the modulo, `degre_compute` will always be in the correct range.
                _ => unreachable!(),
            };
            vec_pitch_class.push(temp);
        }

        let mut vec_semitones: Vec<(Semitones, bool)> = Vec::new();
        for n in 0..255 {
            let n_pitch_class = PitchClass::from(n as Semitones);
            let sw_tonic = if n_pitch_class.clone() == self.tonic.pitch_class {
                true
            } else {
                false
            };
            for vpc in vec_pitch_class.clone() {
                // Found
                if vpc == n_pitch_class {
                    vec_semitones.push((n, sw_tonic.clone()));
                }
            }
        }
        vec_semitones
    }

    /// Get note for a specific string (begin at 1 (not 0))
    pub fn get_string_combination(self) -> Vec<DegreeAllStrings> {
        use PitchClass::*;
        let interval = self.tuning.get_interval();
        let roots = [
            Note::from_str("E").unwrap() + interval,
            Note::from_str("A").unwrap() + interval,
            Note::from_str("D").unwrap() + interval + Interval::PlusOctave,
            Note::from_str("G").unwrap() + interval + Interval::PlusOctave,
            Note::from_str("B").unwrap() + interval + Interval::PlusOctave,
            Note::from_str("E").unwrap() + interval + Interval::PlusPlusOctave,
        ];
        let mut vec_degree_all_strings: Vec<DegreeAllStrings> = Vec::new();
        let mut string_number = 0;
        loop {
            string_number += 1;
            if string_number > NUMBER_STRING as usize {
                break;
            }
            let i = string_number.clone() - 1;
            let p_class: PitchClass = roots[i].pitch_class;

            let pos_begin: usize = match &p_class {
                C => 0,
                CSharp => 1,
                D => 2,
                DSharp => 3,
                E => 4,
                F => 5,
                FSharp => 6,
                G => 7,
                GSharp => 8,
                A => 9,
                ASharp => 10,
                B => 11,
            };
            let pos_end = &pos_begin + 19;

            let mut vec_degree_single_string: Vec<DegreeSingleString> =
                Vec::new();

            let mut count: usize = pos_begin;
            let mut sw_begin = true;
            loop {
                if sw_begin {
                    sw_begin = false
                } else {
                    count += 1;
                }
                if count > pos_end {
                    break;
                }
                for (d, sw_bool) in self.get_degree() {
                    // Position in scale
                    if d as usize == count {
                        let position = count.clone() - &pos_begin;
                        let sw_tonic = sw_bool.clone();
                        let note =
                            Note::from_semitones(count.clone() as Semitones);
                        let degree_single_string = DegreeSingleString {
                            position,
                            sw_tonic,
                            note,
                        };
                        vec_degree_single_string.push(degree_single_string);
                    }
                }
            }

            let degree_all_strings = DegreeAllStrings {
                string_number,
                degree_single_string: vec_degree_single_string,
            };
            vec_degree_all_strings.push(degree_all_strings);
        }
        vec_degree_all_strings
    }
}