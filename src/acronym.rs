use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    str::FromStr,
};

use crate::error::AcronymParseError;

/// The acronym of a [`GameMod`].
///
/// [`GameMod`]: crate::generated_mods::GameMod
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Acronym([u8; 3]);

impl Acronym {
    /// Create an [`Acronym`] from a string.
    ///
    /// # Safety
    ///
    /// The given string must consist of two or three bytes representing capitalized ASCII letters or digits.
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::Acronym;
    ///
    /// let hd = unsafe { Acronym::from_str_unchecked("HD") };
    /// assert_eq!(hd.as_str(), "HD");
    /// ```
    ///
    /// Each of the following may lead to undefined behavior, don't do that!
    /// ```rust,no_run
    /// # use rosu_mods::Acronym;
    /// let _ = unsafe { Acronym::from_str_unchecked("HDHR") }; // must be 2 or 3 characters
    /// let _ = unsafe { Acronym::from_str_unchecked("hd") };   // must be uppercase
    /// ```
    pub const unsafe fn from_str_unchecked(s: &str) -> Self {
        let array = if s.len() == 2 {
            // SAFETY: `s` is guaranteed to be of length 2
            let [a, b] = unsafe { *(s.as_ptr().cast::<[u8; 2]>()) };

            [0, a, b]
        } else {
            // SAFETY: caller guarantees that `s` is of length 3
            unsafe { *s.as_ptr().cast::<[u8; 3]>() }
        };

        Self(array)
    }

    /// Returns the [`Acronym`] as a string.
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::Acronym;
    ///
    /// let hd = "HD".parse::<Acronym>().unwrap();
    /// assert_eq!(hd.as_str(), "HD");
    /// ```
    pub fn as_str(&self) -> &str {
        let start_idx = usize::from(self.0[0] == 0);

        // SAFETY: `self.0` is known to be constructed from a valid string
        unsafe { std::str::from_utf8_unchecked(&self.0[start_idx..]) }
    }
}

impl Debug for Acronym {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(self.as_str(), f)
    }
}

impl Display for Acronym {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.as_str())
    }
}

impl FromStr for Acronym {
    type Err = AcronymParseError;

    /// Create an [`Acronym`] from a string.
    ///
    /// Errors if the acronym consists of fewer than 2 or more than 3 bytes.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match <[u8; 2]>::try_from(s.as_bytes()) {
            Ok([a, b]) => Ok(Self([0, a.to_ascii_uppercase(), b.to_ascii_uppercase()])),
            Err(_) => s
                .as_bytes()
                .try_into()
                .map(|mut array: [u8; 3]| {
                    array.make_ascii_uppercase();

                    Self(array)
                })
                .map_err(|_| AcronymParseError {
                    acronym: Box::from(s),
                }),
        }
    }
}

impl PartialOrd for Acronym {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Acronym {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}
