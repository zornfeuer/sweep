use crate::types::OS;
use serde::{Deserialize, Deserializer};
use crossterm::event::KeyCode;

#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct Config {
    pub os: Option<OS>,
    pub theme: Theme,
    pub keybindings: Keybindings,
}

impl Config {
    pub fn load_config() -> anyhow::Result<Self> {
        let config_path = dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("sweep/config.toml");
     
        if config_path.exists() {
            let contents = std::fs::read_to_string(config_path)?;
            let config: Self = toml::from_str(&contents)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Theme {
    pub selected_bg: String,
    pub package_icon: String,
    pub artifact_icon: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            selected_bg: "blue".to_string(),
            package_icon: "".to_string(),
            artifact_icon: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Keybindings {
    pub quit: Vec<KeyCode>,
    pub select: Vec<KeyCode>,
    pub confirm: Vec<KeyCode>,
    pub select_all: Vec<KeyCode>,
    pub cursor_up: Vec<KeyCode>,
    pub cursor_down: Vec<KeyCode>,
}

impl<'de> Deserialize<'de> for Keybindings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            quit: Vec<String>,
            select: Vec<String>,
            confirm: Vec<String>,
            select_all: Vec<String>,
            cursor_up: Vec<String>,
            cursor_down: Vec<String>,
        }

        let helper = Helper::deserialize(deserializer)?;

        let parse_vec = |v: Vec<String>, field_name: &str| {
            v.into_iter()
                .map(|s| {
                    Keybindings::parse_keycode_str(&s).map_err(|e| {
                        serde::de::Error::custom(format!("In keybindings.{}: {}", field_name, e))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        };

        Ok(Keybindings {
            quit: parse_vec(helper.quit, "quit")?,
            select: parse_vec(helper.select, "select")?,
            confirm: parse_vec(helper.confirm, "confirm")?,
            select_all: parse_vec(helper.select_all, "select_all")?,
            cursor_up: parse_vec(helper.cursor_up, "cursor_up")?,
            cursor_down: parse_vec(helper.cursor_down, "cursor_down")?,
        })
    }
}

impl Keybindings {
    fn parse_keycode_str(s: &str) -> Result<KeyCode, String> {
        let s = s.trim().to_lowercase();

        match s.as_str() {
            "esc" | "escape" => Ok(KeyCode::Esc),
            "enter" | "return" => Ok(KeyCode::Enter),
            "space" => Ok(KeyCode::Char(' ')),
            "up" => Ok(KeyCode::Up),
            "down" => Ok(KeyCode::Down),
            "left" => Ok(KeyCode::Left),
            "right" => Ok(KeyCode::Right),
            "tab" => Ok(KeyCode::Tab),
            "backspace" | "bs" => Ok(KeyCode::Backspace),
            c if c.len() == 1 && c.chars().next().unwrap().is_ascii_alphanumeric() => {
                Ok(KeyCode::Char(c.chars().next().unwrap().to_ascii_uppercase()))
            },
            _ => Err(format!("Unknown key: {}", s)),
        }
    }
}

impl Default for Keybindings {
    fn default() -> Self {
        Self {
            quit: vec![KeyCode::Char('q'), KeyCode::Esc],
            select: vec![KeyCode::Char(' ')],
            confirm: vec![KeyCode::Enter],
            select_all: vec![KeyCode::Char('a')],
            cursor_up: vec![KeyCode::Up, KeyCode::Char('k')],
            cursor_down: vec![KeyCode::Down, KeyCode::Char('j')],
        }
    }
}
