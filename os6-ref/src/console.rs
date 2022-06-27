//! SBI console driver, for text output

use crate::sbi::console_putchar;
use core::fmt::{self, Write};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
/// print string macro
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
/// println string macro
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

/*
以下代码提供了与颜色相关的 ANSI 转义字符，以及彩色输出可以使用的函数与宏。

可以使用它们，甚至扩展它们，来提升开发体验和显示效果。
*/

// 使用 ANSI 转义字符来加上颜色
#[macro_export]
macro_rules! colorize {
    ($content: ident, $foreground_color: ident) => {
        format_args!("\u{1B}[{}m{}\u{1B}[0m", $foreground_color as u8, $content)
    };
    ($content: ident, $foreground_color: ident, $background_color: ident) => {
        format_args!(
            "\u{1B}[{}m\u{1B}[{}m{}\u{1B}[0m",
            $foreground_color.into(),
            $background_color.into(),
            $content
        )
    };
}

pub fn print_colorized(
    args: fmt::Arguments,
    foreground_color: impl Into<u8>,
    background_color: impl Into<u8>,
) {
    Stdout
        .write_fmt(colorize!(args, foreground_color, background_color))
        .unwrap();
}

#[macro_export]
macro_rules! print_colorized {
    ($fmt: literal, $foreground_color: expr, $background_color: expr $(, $($arg: tt)+)?) => {
        $crate::console::print_colorized(format_args!($fmt $(, $($arg)+)?), $foreground_color as u8, $background_color as u8);
    };
}

#[macro_export]
macro_rules! println_colorized {
    ($fmt: literal, $foreground_color: expr, $background_color: expr $(, $($arg: tt)+)?) => {
        $crate::console::print_colorized(format_args!(concat!($fmt, "\n") $(, $($arg)+)?), $foreground_color as u8, $background_color as u8);
    }
}

#[allow(dead_code)]
pub enum ANSICON {
    Reset = 0,
    Bold = 1,
    Underline = 4,
    Blink = 5,
    Reverse = 7,
    FgBlack = 30,
    FgRed = 31,
    FgGreen = 32,
    FgYellow = 33,
    FgBlue = 34,
    FgMagenta = 35,
    FgCyan = 36,
    FgWhite = 37,
    FgDefault = 39,
    FgLightGray = 90,
    FgLightRed = 91,
    FgLightGreen = 92,
    FgLightYellow = 93,
    FgLightBlue = 94,
    FgLightMagenta = 95,
    FgLightCyan = 96,
    FgLightWhite = 97,
    BgBlack = 40,
    BgRed = 41,
    BgGreen = 42,
    BgYellow = 43,
    BgBlue = 44,
    BgMagenta = 45,
    BgCyan = 46,
    BgWhite = 47,
    BgDefault = 49,
    BgLightGray = 100,
    BgLightRed = 101,
    BgLightGreen = 102,
    BgLightYellow = 103,
    BgLightBlue = 104,
    BgLightMagenta = 105,
    BgLightCyan = 106,
    BgLightWhite = 107,
}

impl From<ANSICON> for u8 {
    fn from(con: ANSICON) -> Self {
        con as Self
    }
}
