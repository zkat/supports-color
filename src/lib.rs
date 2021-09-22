#![doc = include_str!("../README.md")]

pub use atty::Stream;
use regex::Regex;

lazy_static::lazy_static! {
    static ref TERM_256_COLOR_REGEX: Regex = Regex::new(r"-256(color)?$").unwrap();
    static ref TERM_ANSI_REGEX: Regex = Regex::new(r"^screen|^xterm|^vt100|^vt220|^rxvt|color|ansi|cygwin|linux").unwrap();
}

fn env_force_color() -> usize {
    if let Ok(force) = std::env::var("FORCE_COLOR") {
        match force.as_ref() {
            "true" => 1,
            "false" => 0,
            f if f.is_empty() => 1,
            f => std::cmp::min(f.parse().unwrap_or(1), 3),
        }
    } else {
        0
    }
}

fn translate_level(level: usize) -> Option<ColorLevel> {
    if level == 0 {
        None
    } else {
        Some(ColorLevel {
            level,
            has_basic: true,
            has_256: level >= 2,
            has_16m: level >= 3,
        })
    }
}

fn supports_color(stream: Stream) -> usize {
    let force_color = env_force_color();
    let no_color = match std::env::var("NO_COLOR") {
        Ok(val) if val == *"0" => false,
        Ok(_) => true,
        Err(_) => false,
    };
    let min = std::cmp::max(force_color, 0);
    if force_color > 0 {
        force_color
    } else if !atty::is(stream) || no_color {
        0
    } else if std::env::var("TERM") == Ok("dumb".into()) {
        min
    } else if std::env::var("COLORTERM") == Ok("truecolor".into())
        || std::env::var("TERM_PROGRAM") == Ok("iTerm.app".into())
        || std::env::var("TERM").map(|term| TERM_256_COLOR_REGEX.is_match(&term)) == Ok(true)
    {
        3
    } else if std::env::var("TERM_PROGRAM") == Ok("Apple_Terminal".into()) {
        2
    } else if std::env::var("COLORTERM").is_ok()
        || std::env::var("TERM").map(|term| TERM_ANSI_REGEX.is_match(&term)) == Ok(true)
        || std::env::consts::OS == "windows"
        || is_ci::is_ci()
    {
        1
    } else {
        min
    }
}

/**
Returns a [ColorLevel] if a [Stream] supports terminal colors.
*/
pub fn on(stream: Stream) -> Option<ColorLevel> {
    translate_level(supports_color(stream))
}

/**
Color level support details.

This type is returned from [on]. See documentation for its fields for more details.
*/
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ColorLevel {
    level: usize,
    /// Basic ANSI colors are supported.
    pub has_basic: bool,
    /// 256-bit colors are supported.
    pub has_256: bool,
    /// 16 million (RGB) colors are supported.
    pub has_16m: bool,
}
