use std::fmt;

use cmdjewel_core::{
    board::Board,
    gems::{Gem, GemColor, GemSelector},
};
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize, Serializer,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    /// Global settings
    pub settings: Settings,
    /// GameSaves
    pub save: Save,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    /// Music volume between 0 and 1, inclusive
    pub music_vol: f32,
    /// Color palette
    pub theme: Option<Theme>,
}

/// Theme colours for the Cursive theme cmdjewel uses.
#[derive(Serialize, Deserialize, Clone)]
pub struct Theme {
    pub background: Color,
    pub foreground: Color,
    pub title_primary: Color,
    pub inactive_fg: Color,
    pub inactive_bg: Color,
    /// Color of highlighted UI components e.g. buttons
    pub highlight: Color,
    /// Ditto, but with text colors
    pub highlight_text: Color,
    /// Inactive focus color
    pub highlight_inactive: Color,
    /// Gem colors
    pub grey: Color,
    pub blue: Color,
    pub red: Color,
    pub yellow: Color,
    pub green: Color,
    pub orange: Color,
    pub purple: Color,
}

/// Theme colours
#[derive(Clone)]
pub struct Color(pub u8, pub u8, pub u8);

impl Into<cursive::theme::Color> for Color {
    fn into(self) -> cursive::theme::Color {
        cursive::theme::Color::Rgb(self.0, self.1, self.2)
    }
}

struct ColorVisitor;

impl<'de> Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a hexadecimal color string with the format #RRGGBB")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let r = u8::from_str_radix(&value[1..3], 16);
        let g = u8::from_str_radix(&value[3..5], 16);
        let b = u8::from_str_radix(&value[5..7], 16);
        if r.is_err() || g.is_err() || b.is_err() {
            Err(E::custom(format!(
                "String '{}' not valid #RRGGBB color string",
                value
            )))
        } else {
            Ok(Color(r.unwrap(), g.unwrap(), b.unwrap()))
        }
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ColorVisitor)
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("#{:02x}{:02x}{:02x}", self.0, self.1, self.2))
    }
}

/// The default theme is Nord.
impl std::default::Default for Theme {
    fn default() -> Self {
        Theme {
            background: Color(46, 52, 64),
            foreground: Color(213, 219, 230),
            highlight_text: Color(213, 219, 230),
            title_primary: Color(126, 158, 189),
            inactive_fg: Color(76, 86, 106),
            inactive_bg: Color(59, 66, 82),
            highlight: Color(190, 96, 105),
            highlight_inactive: Color(126, 158, 189),
            grey: Color(67, 76, 94),
            blue: Color(126, 158, 189),
            red: Color(190, 96, 105),
            yellow: Color(233, 201, 138),
            green: Color(162, 188, 139),
            orange: Color(207, 135, 111),
            purple: Color(174, 174, 255),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Save {
    pub classic: Option<GameSave>,
    pub zen: Option<GameSave>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameSave {
    pub data: Vec<GemWrapper>,
    pub score: u32,
    pub level: u8,
    pub level_progress: f32,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Config {
            settings: Settings {
                music_vol: 1.0,
                theme: None,
            },
            save: Save {
                classic: None,
                zen: None,
            },
        }
    }
}

impl GameSave {
    pub fn new(board: &Board) -> Self {
        GameSave {
            data: board
                .as_ref()
                .iter()
                .map(|x| GemWrapper(x.clone()))
                .collect::<Vec<GemWrapper>>(),
            score: board.get_score(),
            level: board.get_level(),
            level_progress: board.get_level_progress(),
        }
    }
}

/*
   Redefine gems
*/

#[derive(Serialize, Deserialize, Clone)]
pub struct GemWrapper(#[serde(with = "GemDef")] pub Gem);

/// Types of gems to use.
#[derive(Serialize, Deserialize)]
#[serde(remote = "Gem")]
pub enum GemDef {
    Empty,
    #[serde(with = "GemColorDef")]
    Normal(GemColor),
    #[serde(with = "GemColorDef")]
    Flame(GemColor),
    // Star gems.
    #[serde(with = "GemColorDef")]
    Star(GemColor),
    // Suprnova gems.
    #[serde(with = "GemColorDef")]
    Supernova(GemColor),
    // Hypercube
    #[serde(with = "GemSelectorDef")]
    Hypercube(GemSelector),
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "GemColor")]
pub enum GemColorDef {
    Blue,
    White,
    Red,
    Yellow,
    Green,
    Orange,
    Purple,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "GemSelector")]
pub enum GemSelectorDef {
    #[serde(with = "GemColorDef")]
    Color(GemColor),
    All,
    None,
}
