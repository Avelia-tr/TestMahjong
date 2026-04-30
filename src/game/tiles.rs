use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MahjongTile {
    identity: MahjongTilesIdentity,
    red: bool,
    // other shit probably ?
    // idk
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MahjongTilesIdentity {
    Man(NumberTile),
    Pin(NumberTile),
    Sou(NumberTile),
    Wind(WindsTiles),
    Dragon(DragonTiles),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DragonTiles {
    White,
    Green,
    Red,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WindsTiles {
    East,
    South,
    West,
    North,
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
}

impl NumberTile {
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

    fn get_next_one(&self) -> Self {
        match self.0 {
            // [UNSAFE] 0 is safe value to init
            9 => unsafe { Self::new_unchecked(0) },
            // [UNSAFE] 1..9 are safe value to init
            0..9 => unsafe { Self::new_unchecked(self.0 + 1) },
            _ => unreachable!(), // it is a guarantee that TileNumber is between 0 and 9 inclusive
        }
    }
}

impl DragonTiles {
    fn get_next_one(&self) -> Self {
        match self {
            Self::White => Self::Green,
            Self::Green => Self::Red,
            Self::Red => Self::White,
        }
    }
}

impl WindsTiles {
    fn get_next_one(&self) -> Self {
        match self {
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
            Self::North => Self::East,
        }
    }
}

impl MahjongTilesIdentity {
    fn is_honor(&self) -> bool {
        matches!(self, Self::Dragon(_) | Self::Wind(_))
    }

    fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Man(NumberTile(1 | 9))
                | Self::Sou(NumberTile(1 | 9))
                | Self::Pin(NumberTile(1 | 9))
        )
    }

    fn get_dora(&self) -> Self {
        match self {
            Self::Man(x) => Self::Man(x.get_next_one()),
            Self::Pin(x) => Self::Pin(x.get_next_one()),
            Self::Sou(x) => Self::Sou(x.get_next_one()),
            Self::Wind(x) => Self::Wind(x.get_next_one()),
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

impl Display for WindsTiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::East => "Ea",
            Self::South => "No",
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
            Self::Man(x) => f.write_fmt(format_args!("{}m", x)),
            Self::Pin(x) => f.write_fmt(format_args!("{}p", x)),
            Self::Sou(x) => f.write_fmt(format_args!("{}s", x)),
        }
    }
}
