use std::fmt::{self, Display};

use serde::{
    Deserialize, Serialize, Serializer,
    de::{self, Visitor},
};

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

/// Theme colours. We have to make a custom type that we convert because... serde. I hate serde.
#[derive(Clone)]
pub struct Color(pub u8, pub u8, pub u8);

impl From<Color> for cursive::theme::Color {
    fn from(val: Color) -> Self {
        cursive::theme::Color::Rgb(val.0, val.1, val.2)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.0, self.1, self.2)
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
        if let Ok(r) = u8::from_str_radix(&value[1..3], 16)
            && let Ok(g) = u8::from_str_radix(&value[3..5], 16)
            && let Ok(b) = u8::from_str_radix(&value[5..7], 16)
            && value.len() == 7
        {
            Ok(Color(r, g, b))
        } else {
            Err(E::custom(format!(
                "String \"{}\" not valid #RRGGBB color string",
                value
            )))
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
        serializer.serialize_str(&format!("{}", self))
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
