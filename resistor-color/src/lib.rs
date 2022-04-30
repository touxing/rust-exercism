use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy, IntEnum, IntoEnumIterator)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(color: ResistorColor) -> usize {
    color.int_value() as usize
}

pub fn value_to_color_string(value: usize) -> String {
    ResistorColor::from_int(value as u8)
        .map(|c| format!("{:?}", c))
        .unwrap_or_else(|_| String::from("value out of range"))
}

pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().collect()
}
