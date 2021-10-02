//! Detects whether a terminal supports color, and gives details about that
//! support. It takes into account the `NO_COLOR` environment variable.
//!
//! This crate is a Rust port of [@sindresorhus](https://github.com/sindresorhus)'
//! [NPM package by the same name](https://npm.im/supports-color).
//!
//! ## Example
//!
//! ```rust
//! use supports_color::Stream;
//!
//! if let Some(support) = supports_color::on(Stream::Stdout) {
//!     if support.has_16m {
//!         println!("16 million (RGB) colors are supported");
//!     } else if support.has_256 {
//!         println!("256-bit colors are supported.");
//!     } else if support.has_basic {
//!         println!("Only basic ANSI colors are supported.");
//!     }
//! } else {
//!     println!("No color support.");
//! }
//! ```

pub use atty::Stream;

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
    {
        3
    } else if std::env::var("TERM_PROGRAM") == Ok("Apple_Terminal".into())
        || std::env::var("TERM").map(|term| check_256_color(&term)) == Ok(true)
    {
        2
    } else if std::env::var("COLORTERM").is_ok()
        || std::env::var("TERM").map(|term| check_ansi_color(&term)) == Ok(true)
        || std::env::consts::OS == "windows"
        || is_ci::uncached()
    {
        1
    } else {
        min
    }
}

fn check_ansi_color(term: &str) -> bool {
    term.starts_with("screen")
        || term.starts_with("xterm")
        || term.starts_with("vt100")
        || term.starts_with("vt220")
        || term.starts_with("rxvt")
        || term.contains("color")
        || term.contains("ansi")
        || term.contains("cygwin")
        || term.contains("linux")
}

fn check_256_color(term: &str) -> bool {
    term.ends_with("256") || term.ends_with("256color")
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
