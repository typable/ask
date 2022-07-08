pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color {
    pub fn fg(&self) -> usize {
        match self {
            Self::Black => 30,
            Self::Red => 31,
            Self::Green => 32,
            Self::Yellow => 33,
            Self::Blue => 34,
            Self::Magenta => 35,
            Self::Cyan => 36,
            Self::White => 37,
            Self::BrightBlack => 90,
            Self::BrightRed => 91,
            Self::BrightGreen => 92,
            Self::BrightYellow => 93,
            Self::BrightBlue => 94,
            Self::BrightMagenta => 95,
            Self::BrightCyan => 96,
            Self::BrightWhite => 97,
        }
    }
    pub fn bg(&self) -> usize {
        match self {
            Self::Black => 40,
            Self::Red => 41,
            Self::Green => 42,
            Self::Yellow => 43,
            Self::Blue => 44,
            Self::Magenta => 45,
            Self::Cyan => 46,
            Self::White => 47,
            Self::BrightBlack => 100,
            Self::BrightRed => 101,
            Self::BrightGreen => 102,
            Self::BrightYellow => 103,
            Self::BrightBlue => 104,
            Self::BrightMagenta => 105,
            Self::BrightCyan => 106,
            Self::BrightWhite => 107,
        }
    }
}

#[macro_export]
macro_rules! color {
    ($str:expr, $fg:expr) => {
        format!("\x1b[{}m{}\x1b[0m", $fg.fg(), $str)
    };
    ($str:expr, $fg:expr, $bg:expr) => {
        format!("\x1b[{};{}m{}\x1b[0m", $fg.fg(), $bg.bg(), $str)
    };
}
