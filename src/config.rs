use crate::types::OS;
use serde::{Deserialize, Deserializer};
use crossterm::event::KeyCode;
use ratatui::style::Color;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub os: Option<OS>,

    #[serde(default = "default_su")]
    pub su_command: String,

    #[serde(default)]
    pub theme: Theme,

    #[serde(default)]
    pub keybindings: Keybindings,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Theme {
    #[serde(default)]
    pub selected_bg: ColorDef,

    #[serde(default = "default_package_icon")]
    pub package_icon: String,

    #[serde(default = "default_artifact_icon")]
    pub artifact_icon: String,
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

#[derive(Debug, Clone)]
pub struct ColorDef(pub Color);

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

impl Default for ColorDef {
    fn default() -> Self {
        Self(Color::Green)
    }
}

impl<'de> Deserialize<'de> for ColorDef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let color = parse_color(&s).map_err(serde::de::Error::custom)?;
        Ok(ColorDef(color))
    }
}

impl<'de> Deserialize<'de> for Keybindings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Default, Deserialize)]
        struct Helper {
            #[serde(default = "default_quit")]
            quit: Vec<String>,

            #[serde(default = "default_select")]
            select: Vec<String>,

            #[serde(default = "default_confirm")]
            confirm: Vec<String>,

            #[serde(default = "default_select_all")]
            select_all: Vec<String>,

            #[serde(default = "default_up")]
            cursor_up: Vec<String>,

            #[serde(default = "default_down")]
            cursor_down: Vec<String>,
        }

        let helper = Helper::deserialize(deserializer)?;

        let parse_vec = |v: Vec<String>, field_name: &str| {
            v.into_iter()
                .map(|s| {
                    parse_keycode_str(&s).map_err(|e| {
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
            Ok(KeyCode::Char(c.chars().next().unwrap().to_ascii_lowercase()))
        },
        _ => Err(format!("Unknown key: {}", s)),
    }
}

fn parse_color(s: &str) -> Result<Color, String> {
    let s = s.trim().to_lowercase();

    match s.as_str() {
        "black" => return Ok(Color::Black),
        "red" => return Ok(Color::Red),
        "green" => return Ok(Color::Green),
        "yellow" => return Ok(Color::Yellow),
        "blue" => return Ok(Color::Blue),
        "magenta" => return Ok(Color::Magenta),
        "cyan" => return Ok(Color::Cyan),
        "white" => return Ok(Color::Gray),
        "gray" | "grey" => return Ok(Color::DarkGray),
        _ => {},
    }

    // TODO: tailwind-like

    if s.starts_with('#') {
        let hex = &s[1..];
        match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).map_err(|_| "invalid hex")?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).map_err(|_| "invalid hex")?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).map_err(|_| "invalid hex")?;
                return Ok(Color::Rgb(r, g, b));
            },
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "invalid hex")?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "invalid hex")?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "invalid hex")?;
                return Ok(Color::Rgb(r, g, b));
            }
            _ => return Err("hex color must be #rgb or #rrggbb".to_string()),
        }
    }

    Err(format!("unknown color: {}", s))
}

fn default_quit() -> Vec<String> { vec!["q".to_string(), "escape".to_string()] }
fn default_select() -> Vec<String> { vec!["space".to_string()] }
fn default_confirm() -> Vec<String> { vec!["enter".to_string()] }
fn default_select_all() -> Vec<String> { vec!["a".to_string()] }
fn default_up() -> Vec<String> { vec!["up".to_string(), "k".to_string()] }
fn default_down() -> Vec<String> { vec!["down".to_string(), "j".to_string()] }
fn default_su() -> String { "sudo".to_string() }
fn default_package_icon() -> String { "📦".to_string() }
fn default_artifact_icon() -> String { "🧩".to_string() }
