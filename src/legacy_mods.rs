use std::{
    fmt::{Binary, Display, Formatter, Result as FmtResult},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, Sub, SubAssign},
    str::FromStr,
};

use crate::{
    error::GameModsLegacyParseError, iter::GameModsLegacyIter, util, Acronym, GameModsIntermode,
};

/// Lightweight bitflag type for legacy mods.
///
/// # Example
/// ```
/// use rosu_mods::GameModsLegacy;
///
/// let nomod = GameModsLegacy::default();
/// assert_eq!(nomod, GameModsLegacy::NoMod);
///
/// // Created via bit operations or from a u32
/// let hdhr_1 = GameModsLegacy::Hidden | GameModsLegacy::HardRock;
/// let hdhr_2 = GameModsLegacy::from_bits(8 + 16);
/// assert_eq!(hdhr_1, hdhr_2);
///
/// // Various methods for convenience like `contains` and `intersects`.
/// let ezhdpf = GameModsLegacy::Easy | GameModsLegacy::Hidden | GameModsLegacy::Perfect;
/// assert!(!ezhdpf.contains(GameModsLegacy::HardRock));
/// let hdpf = GameModsLegacy::Hidden | GameModsLegacy::Perfect;
/// assert!(ezhdpf.intersects(hdpf));
///
/// // Parsing a `&str`
/// let hdhrdt = "dthdhr".parse::<GameModsLegacy>().unwrap();
/// assert_eq!(hdhrdt.bits(), 8 + 16 + 64);
///
/// // The Display implementation combines all acronyms
/// assert_eq!(hdhrdt.to_string(), "HDHRDT".to_string());
///
/// // Has an iterator type
/// let mut iter = GameModsLegacy::from_bits(536871512).iter();
/// assert_eq!(iter.next(), Some(GameModsLegacy::Hidden));
/// assert_eq!(iter.next(), Some(GameModsLegacy::HardRock));
/// assert_eq!(iter.next(), Some(GameModsLegacy::Nightcore));
/// assert_eq!(iter.next(), Some(GameModsLegacy::ScoreV2));
/// assert_eq!(iter.next(), None);
/// ```
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GameModsLegacy(u32);

#[allow(non_upper_case_globals)]
impl GameModsLegacy {
    pub const NoMod: Self = Self::from_bits_retain(0);
    pub const NoFail: Self = Self::from_bits_retain(1 << 0);
    pub const Easy: Self = Self::from_bits_retain(1 << 1);
    pub const TouchDevice: Self = Self::from_bits_retain(1 << 2);
    pub const Hidden: Self = Self::from_bits_retain(1 << 3);
    pub const HardRock: Self = Self::from_bits_retain(1 << 4);
    pub const SuddenDeath: Self = Self::from_bits_retain(1 << 5);
    pub const DoubleTime: Self = Self::from_bits_retain(1 << 6);
    pub const Relax: Self = Self::from_bits_retain(1 << 7);
    pub const HalfTime: Self = Self::from_bits_retain(1 << 8);
    pub const Nightcore: Self = Self::from_bits_retain((1 << 9) | Self::DoubleTime.bits());
    pub const Flashlight: Self = Self::from_bits_retain(1 << 10);
    pub const Autoplay: Self = Self::from_bits_retain(1 << 11);
    pub const SpunOut: Self = Self::from_bits_retain(1 << 12);
    pub const Autopilot: Self = Self::from_bits_retain(1 << 13);
    pub const Perfect: Self = Self::from_bits_retain((1 << 14) | Self::SuddenDeath.bits());
    pub const Key4: Self = Self::from_bits_retain(1 << 15);
    pub const Key5: Self = Self::from_bits_retain(1 << 16);
    pub const Key6: Self = Self::from_bits_retain(1 << 17);
    pub const Key7: Self = Self::from_bits_retain(1 << 18);
    pub const Key8: Self = Self::from_bits_retain(1 << 19);
    pub const FadeIn: Self = Self::from_bits_retain(1 << 20);
    pub const Random: Self = Self::from_bits_retain(1 << 21);
    pub const Cinema: Self = Self::from_bits_retain(1 << 22);
    pub const Target: Self = Self::from_bits_retain(1 << 23);
    pub const Key9: Self = Self::from_bits_retain(1 << 24);
    pub const KeyCoop: Self = Self::from_bits_retain(1 << 25);
    pub const Key1: Self = Self::from_bits_retain(1 << 26);
    pub const Key3: Self = Self::from_bits_retain(1 << 27);
    pub const Key2: Self = Self::from_bits_retain(1 << 28);
    pub const ScoreV2: Self = Self::from_bits_retain(1 << 29);
    pub const Mirror: Self = Self::from_bits_retain(1 << 30);
}

impl GameModsLegacy {
    /// Returns the clock rate for the mods i.e. 1.5 for DT, 0.75 for HT,
    /// and 1.0 otherwise.
    pub const fn clock_rate(self) -> f32 {
        if self.contains(Self::DoubleTime) {
            1.5
        } else if self.contains(Self::HalfTime) {
            0.75
        } else {
            1.0
        }
    }

    /// Returns the amount of contained mods.
    ///
    /// # Example
    /// ```
    /// use rosu_mods::GameModsLegacy;
    ///
    /// assert_eq!(GameModsLegacy::NoMod.len(), 0);
    /// let mods = GameModsLegacy::from_bits(8 + 16 + 64 + 128);
    /// assert_eq!(mods.len(), 4);
    /// ```
    pub const fn len(self) -> usize {
        self.bits().count_ones() as usize
            - self.contains(Self::Nightcore) as usize
            - self.contains(Self::Perfect) as usize
    }

    /// Returns an iterator over [`GameModsLegacy`].
    ///
    /// # Example
    /// ```
    /// use rosu_mods::GameModsLegacy;
    ///
    /// let mut iter = GameModsLegacy::from_bits(8 + 16 + 64 + 128).iter();
    /// assert_eq!(iter.next(), Some(GameModsLegacy::Hidden));
    /// assert_eq!(iter.next(), Some(GameModsLegacy::HardRock));
    /// assert_eq!(iter.next(), Some(GameModsLegacy::DoubleTime));
    /// assert_eq!(iter.next(), Some(GameModsLegacy::Relax));
    /// assert_eq!(iter.next(), None);
    ///
    /// let mut iter = GameModsLegacy::NoMod.iter();
    /// assert_eq!(iter.next(), Some(GameModsLegacy::NoMod));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(self) -> GameModsLegacyIter {
        self.into_iter()
    }

    /// Convert [`GameModsLegacy`] to [`GameModsIntermode`].
    pub fn to_intermode(self) -> GameModsIntermode {
        GameModsIntermode::from_bits(self.bits())
    }
}

impl GameModsLegacy {
    const fn all() -> Self {
        Self::from_bits_retain(u32::MAX >> 2)
    }

    /// Get the underlying bits value.
    ///
    /// The returned value is exactly the bits set in this flags value.
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Convert from a bits value.
    ///
    /// This method will return `None` if any unknown bits are set.
    pub const fn try_from_bits(bits: u32) -> Option<Self> {
        if Self::from_bits(bits).0 == bits {
            Some(Self(bits))
        } else {
            None
        }
    }

    /// Convert from a bits value, unsetting any unknown bits.
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits & Self::all().bits())
    }

    /// Convert from a bits value exactly.
    ///
    /// Unknown bits are retained.
    pub const fn from_bits_retain(bits: u32) -> Self {
        Self(bits)
    }

    /// Whether all bits in this flags value are unset.
    pub const fn is_empty(self) -> bool {
        self.0 == Self::NoMod.bits()
    }

    /// Whether any set bits in a source flags value are also set in a target flags value.
    pub const fn intersects(self, other: Self) -> bool {
        self.bits() & other.bits() != 0
    }

    /// Whether all set bits in a source flags value are also set in a target flags value.
    pub const fn contains(self, other: Self) -> bool {
        self.bits() & other.bits() == other.bits()
    }

    /// The bitwise or (`|`) of the bits in two flags values.
    pub fn insert(&mut self, other: Self) {
        *self = Self::from_bits_retain(self.bits()).union(other);
    }

    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
    ///
    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
    /// `remove` won't truncate `other`, but the `!` operator will.
    pub fn remove(&mut self, other: Self) {
        *self = Self::from_bits_retain(self.bits()).difference(other);
    }

    /// The bitwise and (`&`) of the bits in two flags values.
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self::from_bits_retain(self.bits() & other.bits())
    }

    /// The bitwise or (`|`) of the bits in two flags values.
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self::from_bits_retain(self.bits() | other.bits())
    }

    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
    ///
    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
    /// `difference` won't truncate `other`, but the `!` operator will.
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self::from_bits_retain(self.bits() & !other.bits())
    }

    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
    #[must_use]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self::from_bits_retain(self.bits() ^ other.bits())
    }
}

impl Display for GameModsLegacy {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for m in self.into_iter() {
            let acronym = match m {
                Self::NoMod => "NM",
                Self::NoFail => "NF",
                Self::Easy => "EZ",
                Self::TouchDevice => "TD",
                Self::Hidden => "HD",
                Self::HardRock => "HR",
                Self::SuddenDeath => "SD",
                Self::DoubleTime => "DT",
                Self::Relax => "RX",
                Self::HalfTime => "HT",
                Self::Nightcore => "NC",
                Self::Flashlight => "FL",
                Self::SpunOut => "SO",
                Self::Autopilot => "AP",
                Self::Perfect => "PF",
                Self::FadeIn => "FI",
                Self::Random => "RD",
                Self::Target => "TP",
                Self::ScoreV2 => "V2",
                Self::Mirror => "MR",
                Self::Key1 => "1K",
                Self::Key2 => "2K",
                Self::Key3 => "3K",
                Self::Key4 => "4K",
                Self::Key5 => "5K",
                Self::Key6 => "6K",
                Self::Key7 => "7K",
                Self::Key8 => "8K",
                Self::Key9 => "9K",
                Self::Autoplay | Self::Cinema | Self::KeyCoop => "",
                _ => unreachable!(),
            };

            f.write_str(acronym)?;
        }

        Ok(())
    }
}

impl Binary for GameModsLegacy {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Binary::fmt(&self.0, f)
    }
}

impl BitOr for GameModsLegacy {
    type Output = Self;
    /// The bitwise or (`|`) of the bits in two flags values.
    fn bitor(self, other: GameModsLegacy) -> Self {
        self.union(other)
    }
}

impl BitOrAssign for GameModsLegacy {
    /// The bitwise or (`|`) of the bits in two flags values.
    fn bitor_assign(&mut self, other: Self) {
        self.insert(other);
    }
}

impl BitXor for GameModsLegacy {
    type Output = Self;

    /// The bitwise exclusive-or (`^`) of the bits in two flags values.
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}

impl BitAnd for GameModsLegacy {
    type Output = Self;

    /// The bitwise and (`&`) of the bits in two flags values.
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}

impl BitAndAssign for GameModsLegacy {
    /// The bitwise and (`&`) of the bits in two flags values.
    fn bitand_assign(&mut self, other: Self) {
        *self = Self::from_bits_retain(self.bits()).intersection(other);
    }
}

impl Sub for GameModsLegacy {
    type Output = Self;

    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
    ///
    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
    /// `difference` won't truncate `other`, but the `!` operator will.
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}

impl SubAssign for GameModsLegacy {
    /// The intersection of a source flags value with the complement of a target flags value (`&!`).
    ///
    /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
    /// `difference` won't truncate `other`, but the `!` operator will.
    fn sub_assign(&mut self, other: Self) {
        self.remove(other);
    }
}

impl Extend<GameModsLegacy> for GameModsLegacy {
    /// The bitwise or (`|`) of the bits in each flags value.
    fn extend<T: IntoIterator<Item = Self>>(&mut self, iterator: T) {
        for item in iterator {
            self.insert(item);
        }
    }
}

impl FromIterator<GameModsLegacy> for GameModsLegacy {
    /// The bitwise or (`|`) of the bits in each flags value.
    fn from_iter<T: IntoIterator<Item = Self>>(iterator: T) -> Self {
        let mut mods = Self::NoMod;
        mods.extend(iterator);

        mods
    }
}

impl From<GameModsLegacy> for u32 {
    fn from(mods: GameModsLegacy) -> Self {
        mods.bits()
    }
}

impl FromStr for GameModsLegacy {
    type Err = GameModsLegacyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = Self::default();
        let upper = util::to_uppercase(s);

        for m in util::cut(&upper, 2) {
            let m = match m {
                "NM" => Self::NoMod,
                "NF" => Self::NoFail,
                "EZ" => Self::Easy,
                "TD" => Self::TouchDevice,
                "HD" => Self::Hidden,
                "HR" => Self::HardRock,
                "SD" => Self::SuddenDeath,
                "DT" => Self::DoubleTime,
                "RX" | "RL" => Self::Relax,
                "HT" => Self::HalfTime,
                "NC" => Self::Nightcore,
                "FL" => Self::Flashlight,
                "SO" => Self::SpunOut,
                "AP" => Self::Autopilot,
                "PF" => Self::Perfect,
                "FI" => Self::FadeIn,
                "RD" => Self::Random,
                "TP" => Self::Target,
                "V2" => Self::ScoreV2,
                "MR" => Self::Mirror,
                "1K" | "K1" => Self::Key1,
                "2K" | "K2" => Self::Key2,
                "3K" | "K3" => Self::Key3,
                "4K" | "K4" => Self::Key4,
                "5K" | "K5" => Self::Key5,
                "6K" | "K6" => Self::Key6,
                "7K" | "K7" => Self::Key7,
                "8K" | "K8" => Self::Key8,
                "9K" | "K9" => Self::Key9,
                _ => return Err(GameModsLegacyParseError { mods: Box::from(s) }),
            };

            res.insert(m);
        }

        Ok(res)
    }
}

impl TryFrom<Acronym> for GameModsLegacy {
    type Error = GameModsLegacyParseError;

    fn try_from(acronym: Acronym) -> Result<Self, Self::Error> {
        acronym.as_str().parse()
    }
}

impl IntoIterator for GameModsLegacy {
    type Item = GameModsLegacy;
    type IntoIter = GameModsLegacyIter;

    fn into_iter(self) -> Self::IntoIter {
        GameModsLegacyIter::new(self)
    }
}

#[cfg(feature = "serde")]
const _: () = {
    use serde::{
        de::{Deserialize, Deserializer},
        ser::{Serialize, Serializer},
    };

    use crate::GameModsIntermode;

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de> Deserialize<'de> for GameModsLegacy {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let mods = GameModsIntermode::deserialize(d)?;

            Ok(Self::from(mods.bits()))
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl Serialize for GameModsLegacy {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            s.serialize_u32(self.bits())
        }
    }
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_str() {
        assert_eq!(
            GameModsLegacy::from_str("Nm").unwrap(),
            GameModsLegacy::NoMod
        );
        assert_eq!(
            GameModsLegacy::from_str("hD").unwrap(),
            GameModsLegacy::Hidden
        );

        let mods = GameModsLegacy::from_bits(24);
        assert_eq!(GameModsLegacy::from_str("HRhD").unwrap(), mods);
        assert!(GameModsLegacy::from_str("HHDR").is_err());
    }

    #[test]
    fn iter() {
        let mut iter = GameModsLegacy::default().iter();
        assert_eq!(iter.next(), Some(GameModsLegacy::NoMod));
        assert_eq!(iter.next(), None);

        let mut iter = GameModsLegacy::from_bits(584).iter();
        assert_eq!(iter.next(), Some(GameModsLegacy::Hidden));
        assert_eq!(iter.next(), Some(GameModsLegacy::Nightcore));
        assert_eq!(iter.next(), None);
    }
}
