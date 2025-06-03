use std::fmt::{self, Debug, Formatter};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Coordinate(pub usize);

impl From<Coordinate> for u16 {
    fn from(value: Coordinate) -> Self {
        value.0.try_into().expect("coordinate should fit in u16")
    }
}

impl From<u16> for Coordinate {
    fn from(value: u16) -> Self {
        Coordinate(value.into())
    }
}

#[cfg(feature = "uniffi")]
uniffi::custom_type!(Coordinate, u16);

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Point {
    pub line: Coordinate,
    pub column: Coordinate,
    pub offset: Coordinate,
}

impl From<markdown::unist::Point> for Point {
    fn from(value: markdown::unist::Point) -> Self {
        Self {
            line: Coordinate(value.line),
            column: Coordinate(value.column),
            offset: Coordinate(value.offset),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Position {
    pub start: Point,
    pub end: Point,
}

impl From<markdown::unist::Position> for Position {
    fn from(value: markdown::unist::Position) -> Self {
        Self {
            start: value.start.into(),
            end: value.end.into(),
        }
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}-{}:{} ({}-{})",
            self.start.line.0,
            self.start.column.0,
            self.end.line.0,
            self.end.column.0,
            self.start.offset.0,
            self.end.offset.0,
        )
    }
}
