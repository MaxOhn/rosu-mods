use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Error while trying to parse an [`Acronym`].
///
/// [`Acronym`]: crate::Acronym
#[derive(Clone, Debug)]
pub struct AcronymParseError {
    /// The string on which parsing failed.
    pub acronym: Box<str>,
}

impl Error for AcronymParseError {}

impl Display for AcronymParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Failed to parse string `{}` into an acronym",
            self.acronym
        )
    }
}

/// Error while trying to parse [`GameModsLegacy`].
///
/// [`GameModsLegacy`]: crate::GameModsLegacy
#[derive(Clone, Debug)]
pub struct GameModsLegacyParseError {
    /// The string on which parsing failed.
    pub mods: Box<str>,
}

impl Error for GameModsLegacyParseError {}

impl Display for GameModsLegacyParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Failed to parse string `{}` into GameModsLegacy",
            self.mods
        )
    }
}
