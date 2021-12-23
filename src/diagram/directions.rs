use std::ops::Not;

use super::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Directions {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}

macro_rules! one_direction {
    ( $($dir: ident),* ) => {
        $(
            pub fn $dir() -> Self {
                Self {
                    $dir: true,
                    ..Self::none()
                }
            }
        )*
    };
}

impl Directions {
    pub fn none() -> Self {
        Self {
            left: false,
            right: false,
            up: false,
            down: false,
        }
    }

    pub fn all() -> Self {
        !Self::none()
    }

    pub fn opposite(self) -> Self {
        Self {
            left: self.right,
            right: self.left,
            up: self.down,
            down: self.up,
        }
    }

    one_direction!(left, right, up, down);

    pub fn deltas(&self) -> impl Iterator<Item = (Vec2<isize>, Directions)> {
        [
            (self.left, Directions::left()),
            (self.right, Directions::right()),
            (self.up, Directions::up()),
            (self.down, Directions::down()),
        ]
        .into_iter()
        .zip([vec2(-1, 0), vec2(1, 0), vec2(0, 1), vec2(0, -1)].into_iter())
        .filter_map(|((active, direction), delta)| {
            if active {
                Some((delta, direction))
            } else {
                None
            }
        })
    }

    pub fn map(self, op: impl Fn(bool) -> bool) -> Self {
        Self {
            left: op(self.left),
            right: op(self.right),
            up: op(self.up),
            down: op(self.down),
        }
    }

    pub fn zip(self, other: Self, op: impl Fn(bool, bool) -> bool) -> Self {
        Self {
            left: op(self.left, other.left),
            right: op(self.right, other.right),
            up: op(self.up, other.up),
            down: op(self.down, other.down),
        }
    }

    pub fn and(self, rhs: Self) -> Self {
        self.zip(rhs, |a, b| a && b)
    }
}

impl Add<Self> for Directions {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.zip(rhs, |a, b| a || b)
    }
}

impl Sub<Self> for Directions {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.zip(rhs, |a, b| a && !b)
    }
}

impl Not for Directions {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.map(|x| !x)
    }
}
