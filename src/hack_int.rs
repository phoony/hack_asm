use thiserror::Error;

/// A HackInt is an integer between 0 and 32767 (inclusive).
/// So technically a u16 is one bit larger but it is an in-built type we can use.
/// However, a HackInt shall always be inside of the aforementioned bounds.
/// We assure the correctness of this by checking the user input inside of the parser.
#[derive(Copy, Clone)]
pub struct HackInt(u16);

#[derive(Error, Debug)]
pub enum ParseHackIntError {
    #[error("number is not in bounds")]
    SizeExceeded,
    #[error("could not parse int")]
    ParseInt(#[from] std::num::ParseIntError),
}

impl HackInt {
    const MAX: u16 = 32767;

    pub fn try_new(value: u16) -> Result<Self, ParseHackIntError> {
        if value > Self::MAX {
            return Err(ParseHackIntError::SizeExceeded);
        }

        Ok(Self(value))
    }

    pub fn parse(input: &str) -> Result<Self, ParseHackIntError> {
        let value: u16 = input.parse()?;
        Self::try_new(value)
    }

    pub(crate) const fn new_unchecked(value: u16) -> HackInt {
        Self(value)
    }
}

impl From<HackInt> for u16 {
    fn from(hack_int: HackInt) -> Self {
        hack_int.0
    }
}

impl TryInto<HackInt> for u16 {
    type Error = ParseHackIntError;

    fn try_into(self) -> Result<HackInt, Self::Error> {
        HackInt::try_new(self)
    }
}
