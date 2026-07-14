use std::{fmt::Display, hint::unreachable_unchecked, num::NonZeroI8};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MahjongTile {
    pub identity: MahjongTilesIdentity,
    pub red: bool,
    // other shit probably ?
    // idk
    // uid maybe ???
}

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord)]
pub enum MahjongTilesIdentity {
    Man(NumberTile),
    Pin(NumberTile),
    Sou(NumberTile),
    Wind(Wind),
    Dragon(DragonTiles),
}

impl PartialEq for MahjongTilesIdentity {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Man(l0), Self::Man(r0))
            | (Self::Pin(l0), Self::Pin(r0))
            | (Self::Sou(l0), Self::Sou(r0)) => l0 == r0,
            (Self::Wind(l0), Self::Wind(r0)) => l0 == r0,
            (Self::Dragon(l0), Self::Dragon(r0)) => l0 == r0,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DragonTiles {
    White,
    Green,
    Red,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Wind {
    East = 0,
    South = 1,
    West = 2,
    North = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumberTile(u8);

#[derive(Debug)]
pub enum NumberError {
    ErrorZero,
    ErrorOutOfRange(u8),
}

impl MahjongTile {
    pub const fn new(id: MahjongTilesIdentity) -> Self {
        Self {
            identity: id,
            red: false,
        }
    }

    pub const fn follows(self, other: Self) -> bool {
        use MahjongTilesIdentity as MTI;
        match (self.identity.get_dora(), other.identity) {
            (MTI::Man(l0), MTI::Man(r0))
            | (MTI::Pin(l0), MTI::Pin(r0))
            | (MTI::Sou(l0), MTI::Sou(r0)) => l0.0 == r0.0,
            // while "get_dora" gets the "next" tile for every identity, honor tile don't really
            // have a next tile and are better treated individually here
            _ => false,
        }
    }

    pub const fn is_honor(self) -> bool {
        self.identity.is_honor()
    }

    pub fn is_terminal(self) -> bool {
        self.identity.is_terminal()
    }
}

impl NumberTile {
    // this is a valid state
    pub const N1: NumberTile = NumberTile(1);

    pub const unsafe fn new_unchecked(number: u8) -> Self {
        Self(number)
    }

    pub const fn new(number: u8) -> Result<Self, NumberError> {
        match number {
            0 => Err(NumberError::ErrorZero),
            1..=9 => Ok(unsafe { NumberTile::new_unchecked(number) }),
            _ => Err(NumberError::ErrorOutOfRange(number)),
        }
    }

    const fn get_next_one(self) -> Self {
        match self.0 {
            // [UNSAFE] 1 is safe value to init
            9 => unsafe { Self::new_unchecked(1) },
            // [UNSAFE] 2..9 are safe value to init
            1..9 => unsafe { Self::new_unchecked(self.0 + 1) },
            _ => unreachable!(),
        }
    }
}

impl DragonTiles {
    pub const fn get_next_one(self) -> Self {
        match self {
            Self::White => Self::Green,
            Self::Green => Self::Red,
            Self::Red => Self::White,
        }
    }
}

impl Wind {
    pub const fn get_next(self) -> Self {
        match self {
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
            Self::North => Self::East,
        }
    }

    pub fn relative_position(self, other: Wind) -> usize {
        let self_value: usize = self as usize;
        let other_value = other as usize;

        if self_value > other_value {
            other_value + 4 - self_value
        } else {
            other_value - self_value
        }
    }
}

impl MahjongTilesIdentity {
    pub const fn is_honor(self) -> bool {
        matches!(self, Self::Dragon(_) | Self::Wind(_))
    }

    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Man(NumberTile(1 | 9))
                | Self::Sou(NumberTile(1 | 9))
                | Self::Pin(NumberTile(1 | 9))
        )
    }

    pub const fn get_dora(self) -> Self {
        match self {
            Self::Man(x) => Self::Man(x.get_next_one()),
            Self::Pin(x) => Self::Pin(x.get_next_one()),
            Self::Sou(x) => Self::Sou(x.get_next_one()),
            Self::Wind(x) => Self::Wind(x.get_next()),
            Self::Dragon(x) => Self::Dragon(x.get_next_one()),
        }
    }
}

impl Display for NumberTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl Display for DragonTiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::White => "Dw",
            Self::Red => "Dr",
            Self::Green => "Dg",
        })
    }
}

impl Display for Wind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::East => "Ea",
            Self::South => "So",
            Self::West => "We",
            Self::North => "No",
        })
    }
}

impl Display for MahjongTilesIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dragon(x) => x.fmt(f),
            Self::Wind(x) => x.fmt(f),
            Self::Man(x) => f.write_fmt(format_args!("{x}m")),
            Self::Pin(x) => f.write_fmt(format_args!("{x}p")),
            Self::Sou(x) => f.write_fmt(format_args!("{x}s")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::tiles::Wind;

    #[test]
    fn test_wind_relative_value() {
        assert_eq!(Wind::East.relative_position(Wind::East), 0);
        assert_eq!(Wind::East.relative_position(Wind::South), 1); // doesn't work
        assert_eq!(Wind::East.relative_position(Wind::West), 2);
        assert_eq!(Wind::East.relative_position(Wind::North), 3);
        assert_eq!(Wind::South.relative_position(Wind::East), 3);
        assert_eq!(Wind::South.relative_position(Wind::South), 0);
        assert_eq!(Wind::South.relative_position(Wind::West), 1);
        assert_eq!(Wind::South.relative_position(Wind::North), 2);
        assert_eq!(Wind::North.relative_position(Wind::East), 1);
        assert_eq!(Wind::West.relative_position(Wind::West), 0);
    }
}
