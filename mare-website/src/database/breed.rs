use std::fmt::Display;

use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, sqlx::Type)]
#[repr(i64)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Breed {
    Earth = 0,
    Pegasus = 1,
    Unicorn = 2,
}

impl Display for Breed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let breed = match self {
            Breed::Earth => " Earth ",
            Breed::Pegasus => "Pegasus",
            Breed::Unicorn => "Unicorn",
        };

        write!(f, "{breed}")
    }
}

impl From<i64> for Breed {
    fn from(value: i64) -> Self {
        match value {
            0 => Breed::Earth,
            1 => Breed::Pegasus,
            2 => Breed::Unicorn,
            _ => unreachable!(),
        }
    }
}
