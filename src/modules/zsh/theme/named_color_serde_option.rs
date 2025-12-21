use serde::{self, Deserialize, Deserializer, Serializer};
use zsh_seq::NamedColor;

// Helper function to serialize Option<NamedColor> to a string
pub fn serialize<S>(color: &Option<NamedColor>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match color {
        Some(c) => {
            let s = match c {
                NamedColor::Black => "Black".to_string(),
                NamedColor::Red => "Red".to_string(),
                NamedColor::Green => "Green".to_string(),
                NamedColor::Yellow => "Yellow".to_string(),
                NamedColor::Blue => "Blue".to_string(),
                NamedColor::Magenta => "Magenta".to_string(),
                NamedColor::Cyan => "Cyan".to_string(),
                NamedColor::White => "White".to_string(),
                NamedColor::LightBlack => "LightBlack".to_string(),
                NamedColor::LightRed => "LightRed".to_string(),
                NamedColor::LightGreen => "LightGreen".to_string(),
                NamedColor::LightYellow => "LightYellow".to_string(),
                NamedColor::LightBlue => "LightBlue".to_string(),
                NamedColor::LightMagenta => "LightMagenta".to_string(),
                NamedColor::LightCyan => "LightCyan".to_string(),
                NamedColor::LightWhite => "LightWhite".to_string(),
                NamedColor::Code256(code) => format!("Code256({})", code),
                NamedColor::FullColor((r, g, b)) => format!("FullColor({},{},{})", r, g, b),
            };
            serializer.serialize_str(&s)
        }
        None => serializer.serialize_str("None"),
    }
}

// Helper function to deserialize Option<NamedColor> from a string
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NamedColor>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    deserialize_from_str(&s).map_err(serde::de::Error::custom)
}

pub fn deserialize_from_str(s: &str) -> Result<Option<NamedColor>, String> {
    if s.eq_ignore_ascii_case("None") {
        Ok(None)
    } else if s == "Black" {
        Ok(Some(NamedColor::Black))
    } else if s == "Red" {
        Ok(Some(NamedColor::Red))
    } else if s == "Green" {
        Ok(Some(NamedColor::Green))
    } else if s == "Yellow" {
        Ok(Some(NamedColor::Yellow))
    } else if s == "Blue" {
        Ok(Some(NamedColor::Blue))
    } else if s == "Magenta" {
        Ok(Some(NamedColor::Magenta))
    } else if s == "Cyan" {
        Ok(Some(NamedColor::Cyan))
    } else if s == "White" {
        Ok(Some(NamedColor::White))
    } else if s == "LightBlack" {
        Ok(Some(NamedColor::LightBlack))
    } else if s == "LightRed" {
        Ok(Some(NamedColor::LightRed))
    } else if s == "LightGreen" {
        Ok(Some(NamedColor::LightGreen))
    } else if s == "LightYellow" {
        Ok(Some(NamedColor::LightYellow))
    } else if s == "LightBlue" {
        Ok(Some(NamedColor::LightBlue))
    } else if s == "LightMagenta" {
        Ok(Some(NamedColor::LightMagenta))
    } else if s == "LightCyan" {
        Ok(Some(NamedColor::LightCyan))
    } else if s == "LightWhite" {
        Ok(Some(NamedColor::LightWhite))
    } else if s.starts_with("Code256(") && s.ends_with(')') {
        let code_str = &s[8..s.len() - 1];
        let code = code_str
            .parse::<u8>()
            .map_err(|e| format!("Invalid Code256 format: {}", e))?;
        Ok(Some(NamedColor::Code256(code)))
    } else if s.starts_with("FullColor(") && s.ends_with(')') {
        let parts: Vec<&str> = s[10..s.len() - 1].split(',').collect();
        if parts.len() == 3 {
            let r = parts[0]
                .trim()
                .parse::<u8>()
                .map_err(|e| format!("Invalid FullColor format (R): {}", e))?;
            let g = parts[1]
                .trim()
                .parse::<u8>()
                .map_err(|e| format!("Invalid FullColor format (G): {}", e))?;
            let b = parts[2]
                .trim()
                .parse::<u8>()
                .map_err(|e| format!("Invalid FullColor format (B): {}", e))?;
            Ok(Some(NamedColor::FullColor((r, g, b))))
        } else {
            Err(format!(
                "Invalid FullColor format: {}. Expected FullColor(r,g,b)",
                s
            ))
        }
    } else {
        Err(format!(
            "Unknown NamedColor variant or invalid format: {}",
            s
        ))
    }
}
