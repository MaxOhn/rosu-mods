use std::{
    cmp::Ordering,
    collections::BTreeSet,
    convert::Infallible,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::{BitOr, BitOrAssign, Sub, SubAssign},
    str::FromStr,
};

use crate::{
    generated_mods::{DoubleTimeOsu, NightcoreOsu, PerfectOsu, SuddenDeathOsu},
    util, GameModsLegacy,
};

use crate::GameMode;

use super::{
    intersection::{GameModsIntermodeIntersection, IntersectionInner},
    iter::{GameModsIntermodeIter, IntoGameModsIntermodeIter},
    Acronym, GameMod, GameModIntermode, GameMods,
};

/// Combination of [`GameModIntermode`]s.
#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct GameModsIntermode {
    inner: BTreeSet<GameModIntermode>,
}

impl GameModsIntermode {
    /// Returns empty mods i.e. "`NoMod`"
    pub const fn new() -> Self {
        Self {
            inner: BTreeSet::new(),
        }
    }

    /// Return the accumulated bit values of all contained mods.
    ///
    /// Mods that don't have bit values will be ignored.
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::mods;
    ///
    /// let hdhrdtwu = mods!(HD HR DT WU);
    /// assert_eq!(hdhrdtwu.bits(), 8 + 16 + 64);
    /// ```
    pub fn bits(&self) -> u32 {
        self.inner
            .iter()
            .copied()
            .filter_map(GameModIntermode::bits)
            .fold(0, u32::bitor)
    }

    /// Return the accumulated bit values of all contained mods.
    ///
    /// If any contained mod has no bit value `None` is returned.
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::mods;
    ///
    /// let hdhrdt = mods!(HD HR DT);
    /// assert_eq!(hdhrdt.checked_bits(), Some(8 + 16 + 64));
    ///
    /// let hdhrdtwu = mods!(HD HR DT WU);
    /// assert_eq!(hdhrdtwu.checked_bits(), None);
    /// ```
    pub fn checked_bits(&self) -> Option<u32> {
        self.inner
            .iter()
            .copied()
            .map(GameModIntermode::bits)
            .try_fold(0, |bits, next| Some(next? | bits))
    }

    /// Returns `true` if no mods are contained.
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::{GameModIntermode, GameModsIntermode};
    ///
    /// let mut mods = GameModsIntermode::new();
    /// assert!(mods.is_empty());
    ///
    /// mods.insert(GameModIntermode::Hidden);
    /// assert!(!mods.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns the amount of contained mods.
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::{mods, GameModIntermode, GameModsIntermode};
    ///
    /// let hdhrdt = mods!(HD HR DT);
    /// assert_eq!(hdhrdt.len(), 3);
    ///
    /// let mut nm = GameModsIntermode::new();
    /// assert_eq!(nm.len(), 0);
    /// assert_eq!(nm.to_string(), "NM");
    /// ```
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Add a [`GameModIntermode`]
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::{GameModIntermode, GameModsIntermode};
    ///
    /// let mut mods = GameModsIntermode::new();
    /// assert_eq!(mods.to_string(), "NM");
    ///
    /// mods.insert(GameModIntermode::Traceable);
    /// assert_eq!(mods.to_string(), "TC");
    ///
    /// mods.insert(GameModIntermode::HardRock);
    /// assert_eq!(mods.to_string(), "HRTC");
    /// ```
    pub fn insert(&mut self, gamemod: GameModIntermode) {
        self.inner.insert(gamemod);
    }

    /// Check whether a given mod is contained.
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::{mods, GameModIntermode};
    ///
    /// let hd = mods!(HD);
    /// assert!(hd.contains(GameModIntermode::Hidden));
    /// assert!(!hd.contains(GameModIntermode::HardRock));
    /// ```
    pub fn contains<M>(&self, gamemod: M) -> bool
    where
        GameModIntermode: From<M>,
    {
        self.inner.contains(&GameModIntermode::from(gamemod))
    }

    /// Check whether a given [`Acronym`] is contained.
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::{mods, Acronym};
    ///
    /// let nc = mods!(NC);
    /// assert!(nc.contains_acronym("NC".parse::<Acronym>().unwrap()));
    /// assert!(!nc.contains_acronym("DT".parse::<Acronym>().unwrap()));
    /// ```
    pub fn contains_acronym(&self, acronym: Acronym) -> bool {
        self.inner
            .iter()
            .any(|gamemod| gamemod.acronym() == acronym)
    }

    /// Remove a gamemod and return whether it was contained.
    ///
    /// # Example
    /// ```
    /// use rosu_mods::{mods, GameModIntermode, GameModsIntermode};
    ///
    /// let mut mods: GameModsIntermode = mods!(HD HR);
    ///
    /// assert!(mods.remove(GameModIntermode::Hidden));
    /// assert_eq!(mods.to_string(), "HR");
    /// assert!(!mods.remove(GameModIntermode::DoubleTime));
    /// ```
    pub fn remove<M>(&mut self, gamemod: M) -> bool
    where
        GameModIntermode: From<M>,
    {
        self.inner.remove(&GameModIntermode::from(gamemod))
    }

    /// Remove all mods contained in the iterator.
    ///
    /// # Example
    /// ```
    /// use rosu_mods::{mods, GameModIntermode, GameModsIntermode};
    ///
    /// let mut mods: GameModsIntermode = mods!(HD HR WG DT BR);
    ///
    /// mods.remove_all([GameModIntermode::Hidden, GameModIntermode::Easy]);
    /// assert_eq!(mods.to_string(), "HRDTBRWG");
    ///
    /// mods.remove_all(mods!(NF WG));
    /// assert_eq!(mods.to_string(), "HRDTBR")
    /// ```
    pub fn remove_all<I, M>(&mut self, mods: I)
    where
        I: IntoIterator<Item = M>,
        GameModIntermode: From<M>,
    {
        for gamemod in mods {
            self.remove(gamemod);
        }
    }

    /// Parse bitflags into [`GameModsIntermode`]
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::{mods, GameModsIntermode};
    ///
    /// let bits = 8 + 64 + 512 + 1024;
    /// assert_eq!(GameModsIntermode::from_bits(bits), mods!(FL HD NC))
    /// ```
    pub fn from_bits(mut bits: u32) -> Self {
        struct BitIterator(u32);

        impl Iterator for BitIterator {
            type Item = bool;

            fn next(&mut self) -> Option<Self::Item> {
                if self.0 == 0 {
                    None
                } else {
                    let bit = self.0 & 0b1;
                    self.0 >>= 1;

                    Some(bit == 1)
                }
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                (usize::from(self.0 > 0), None)
            }
        }

        // Special handling for NC and PF since they require two bits
        bits &= if (bits & NightcoreOsu::bits()) == NightcoreOsu::bits() {
            !DoubleTimeOsu::bits()
        } else {
            !(1 << 9)
        };

        bits &= if (bits & PerfectOsu::bits()) == PerfectOsu::bits() {
            !SuddenDeathOsu::bits()
        } else {
            !(1 << 14)
        };

        #[allow(clippy::items_after_statements)]
        const BITFLAG_MODS: [GameModIntermode; 31] = [
            GameModIntermode::NoFail,
            GameModIntermode::Easy,
            GameModIntermode::TouchDevice,
            GameModIntermode::Hidden,
            GameModIntermode::HardRock,
            GameModIntermode::SuddenDeath,
            GameModIntermode::DoubleTime,
            GameModIntermode::Relax,
            GameModIntermode::HalfTime,
            GameModIntermode::Nightcore,
            GameModIntermode::Flashlight,
            GameModIntermode::Autoplay,
            GameModIntermode::SpunOut,
            GameModIntermode::Autopilot,
            GameModIntermode::Perfect,
            GameModIntermode::FourKeys,
            GameModIntermode::FiveKeys,
            GameModIntermode::SixKeys,
            GameModIntermode::SevenKeys,
            GameModIntermode::EightKeys,
            GameModIntermode::FadeIn,
            GameModIntermode::Random,
            GameModIntermode::Cinema,
            GameModIntermode::TargetPractice,
            GameModIntermode::NineKeys,
            GameModIntermode::DualStages,
            GameModIntermode::OneKey,
            GameModIntermode::ThreeKeys,
            GameModIntermode::TwoKeys,
            GameModIntermode::ScoreV2,
            GameModIntermode::Mirror,
        ];

        let inner = BitIterator(bits)
            .zip(BITFLAG_MODS)
            .filter_map(|(is_set, gamemod)| is_set.then_some(gamemod))
            .collect();

        Self { inner }
    }

    /// Try to parse a combination of mod acronyms into [`GameModsIntermode`].
    ///
    /// Returns `None` if an unknown acronym was encountered.
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::GameModsIntermode;
    ///
    /// let hdhrwu = GameModsIntermode::try_from_acronyms("HRWUHD").unwrap();
    /// assert_eq!(hdhrwu.to_string(), "HDHRWU");
    ///
    /// assert!(GameModsIntermode::try_from_acronyms("QQQ").is_none());
    /// ```
    pub fn try_from_acronyms(s: &str) -> Option<Self> {
        let uppercased = util::to_uppercase(s);

        if uppercased == "NM" {
            return Some(Self::new());
        }

        // We currently don't allow a gamemod to have an acronym of length 1
        if s.len() == 1 {
            return None;
        }

        let mut remaining = uppercased.as_ref();
        let mut mods = BTreeSet::new();

        while !remaining.is_empty() {
            // Split off the first two characters and check if it's an acronym
            let (candidate, rest) = util::split_prefix::<2>(remaining);

            // SAFETY: `candidate` is guaranteed to be of length 2 and has been capitalized
            let acronym = unsafe { Acronym::from_str_unchecked(candidate) };
            let gamemod = GameModIntermode::from_acronym(acronym);

            if !matches!(gamemod, GameModIntermode::Unknown(_)) && rest.len() != 1 {
                mods.insert(gamemod);
                remaining = rest;

                continue;
            }

            // Repeat for the first three characters
            let (candidate, rest) = util::split_prefix::<3>(remaining);

            // SAFETY: `candidate` is guaranteed to be of length 3 and has been capitalized
            let acronym = unsafe { Acronym::from_str_unchecked(candidate) };
            let gamemod = GameModIntermode::from_acronym(acronym);

            if matches!(gamemod, GameModIntermode::Unknown(_)) {
                return None;
            }

            mods.insert(gamemod);
            remaining = rest;
        }

        Some(Self { inner: mods })
    }

    /// Parse a combination of mod acronyms into [`GameModsIntermode`].
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::GameModsIntermode;
    ///
    /// let hdhrwu = GameModsIntermode::from_acronyms("HRWUHD");
    /// assert_eq!(hdhrwu.len(), 3);
    /// assert_eq!(hdhrwu.to_string(), "HDHRWU");
    ///
    /// let mut iter = GameModsIntermode::from_acronyms("QQhdQ").into_iter();
    /// assert_eq!(iter.next().unwrap().to_string(), "HDQ"); // unknown mod
    /// assert_eq!(iter.next().unwrap().to_string(), "QQ");  // unknown mod
    /// assert!(iter.next().is_none());
    /// ```
    pub fn from_acronyms(s: &str) -> Self {
        let uppercased = util::to_uppercase(s);

        if uppercased == "NM" {
            return Self::new();
        }

        let mut mods = BTreeSet::new();

        // We currently don't allow a gamemod to have an acronym of length 1
        let mut remaining = if s.len() == 1 {
            mods.insert(GameModIntermode::Unknown(Default::default()));

            ""
        } else {
            uppercased.as_ref()
        };

        while !remaining.is_empty() {
            // Split off the first two characters and check if it's an acronym
            let (candidate, rest) = util::split_prefix::<2>(remaining);

            // SAFETY: `candidate` is guaranteed to be of length 2 and has been capitalized
            let acronym = unsafe { Acronym::from_str_unchecked(candidate) };
            let gamemod = GameModIntermode::from_acronym(acronym);

            if !matches!(gamemod, GameModIntermode::Unknown(_)) && rest.len() != 1 {
                mods.insert(gamemod);
                remaining = rest;

                continue;
            }

            // Repeat for the first three characters
            let (candidate, three_letter_rest) = util::split_prefix::<3>(remaining);

            // SAFETY: `candidate` is guaranteed to be of length 3 and has been capitalized
            let acronym = unsafe { Acronym::from_str_unchecked(candidate) };
            let three_letter_gamemod = GameModIntermode::from_acronym(acronym);

            if !matches!(three_letter_gamemod, GameModIntermode::Unknown(_))
                || three_letter_rest.is_empty()
            {
                mods.insert(three_letter_gamemod);
                remaining = three_letter_rest;
            } else {
                mods.insert(gamemod);
                remaining = rest;
            }
        }

        Self { inner: mods }
    }

    /// Returns an iterator over all mods that appear in both [`GameModsIntermode`].
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::{mods, GameModIntermode};
    ///
    /// let hd = mods!(HD);
    /// let hdhr = mods!(HD HR);
    /// let mut intersection = hd.intersection(&hdhr);
    /// assert_eq!(intersection.next(), Some(GameModIntermode::Hidden));
    /// assert_eq!(intersection.next(), None);
    /// ```
    // https://github.com/rust-lang/rust/blob/c1d3610ac1ddd1cd605479274047fd0a3f37d220/library/alloc/src/collections/btree/set.rs#L517
    pub fn intersection<'m>(
        &'m self,
        other: &'m GameModsIntermode,
    ) -> GameModsIntermodeIntersection<'m> {
        let (self_min, self_max) =
            if let (Some(self_min), Some(self_max)) = (self.inner.first(), self.inner.last()) {
                (*self_min, *self_max)
            } else {
                return GameModsIntermodeIntersection {
                    inner: IntersectionInner::Answer(None),
                };
            };

        let (other_min, other_max) =
            if let (Some(other_min), Some(other_max)) = (other.inner.first(), other.inner.last()) {
                (*other_min, *other_max)
            } else {
                return GameModsIntermodeIntersection {
                    inner: IntersectionInner::Answer(None),
                };
            };

        GameModsIntermodeIntersection {
            inner: match (self_min.cmp(&other_max), self_max.cmp(&other_min)) {
                (Ordering::Greater, _) | (_, Ordering::Less) => IntersectionInner::Answer(None),
                (Ordering::Equal, _) => IntersectionInner::Answer(Some(self_min)),
                (_, Ordering::Equal) => IntersectionInner::Answer(Some(self_max)),
                _ => IntersectionInner::new_stitch(self.inner.iter(), other.inner.iter()),
            },
        }
    }

    /// Check whether the two [`GameMods`] have any common mods.
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::mods;
    ///
    /// let hd = mods!(HD);
    /// assert!(!hd.intersects(&mods!(HR)));
    /// assert!(hd.intersects(&mods!(HD HR)));
    /// ```
    pub fn intersects(&self, other: &Self) -> bool {
        self.intersection(other).next().is_some()
    }

    /// The legacy clock rate of the [`GameModsIntermode`].
    ///
    /// Looks for the first occurrence of DT, NC, HT, or DC
    /// and returns `1.5`, `0.75`, or `1.0` accordingly.
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::{mods, GameModIntermode};
    ///
    /// let hd = mods!(HD);
    /// assert_eq!(hd.legacy_clock_rate(), 1.0);
    ///
    /// let mut hddt = hd;
    /// hddt.insert(GameModIntermode::DoubleTime);
    /// assert_eq!(hddt.legacy_clock_rate(), 1.5);
    /// ```
    pub fn legacy_clock_rate(&self) -> f32 {
        GameModsLegacy::from_bits(self.bits()).clock_rate()
    }

    /// Returns an iterator over all contained mods.
    ///
    /// Note that the iterator will immediately yield `None` in case of "`NoMod`".
    pub fn iter(&self) -> GameModsIntermodeIter<'_> {
        GameModsIntermodeIter::new(self.inner.iter().copied())
    }

    /// Tries to turn a [`GameModsIntermode`] into a [`GameMods`].
    ///
    /// Returns `None` if any contained [`GameModIntermode`] is unknown for the
    /// given [`GameMode`].
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::{mods, GameMods, GameMode};
    ///
    /// let dtfi: GameMods = mods!(DT FI).try_with_mode(GameMode::Mania).unwrap();
    ///
    /// // The FadeIn mod doesn't exist in Taiko
    /// assert!(mods!(DT FI).try_with_mode(GameMode::Taiko).is_none());
    /// ```
    pub fn try_with_mode(&self, mode: GameMode) -> Option<GameMods> {
        self.inner
            .iter()
            .map(|gamemod| GameMod::new(gamemod.acronym().as_str(), mode))
            .try_fold(GameMods::default(), |mut mods, next| {
                if matches!(
                    next,
                    GameMod::UnknownOsu(_)
                        | GameMod::UnknownTaiko(_)
                        | GameMod::UnknownCatch(_)
                        | GameMod::UnknownMania(_)
                ) {
                    None
                } else {
                    mods.insert(next);

                    Some(mods)
                }
            })
    }

    /// Turn a [`GameModsIntermode`] into a [`GameMods`].
    ///
    /// Any contained [`GameModIntermode`] that's unknown for the given
    /// [`GameMode`] will be replaced with `GameModIntermode::Unknown`.
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::{mods, GameMods, GameMode};
    ///
    /// let dtfi: GameMods = mods!(DT FI).with_mode(GameMode::Mania);
    ///
    /// // The FadeIn mod doesn't exist in Taiko
    /// let dt_unknown: GameMods = mods!(DT FI).with_mode(GameMode::Taiko);
    /// assert_eq!(dt_unknown.to_string(), "DTFI");
    /// ```
    pub fn with_mode(&self, mode: GameMode) -> GameMods {
        self.inner
            .iter()
            .map(|gamemod| GameMod::new(gamemod.acronym().as_str(), mode))
            .collect()
    }

    /// Turns [`GameModsIntermode`] into [`GameModsLegacy`].
    pub fn as_legacy(&self) -> GameModsLegacy {
        GameModsLegacy::from_bits(self.bits())
    }

    /// Attempts to turns [`GameModsIntermode`] into [`GameModsLegacy`].
    ///
    /// Returns `None` if any contained [`GameModIntermode`] does not have a
    /// bit value.
    pub fn try_as_legacy(&self) -> Option<GameModsLegacy> {
        self.checked_bits().map(GameModsLegacy::from_bits)
    }
}

impl Debug for GameModsIntermode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_list().entries(self.inner.iter()).finish()
    }
}

impl Display for GameModsIntermode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.is_empty() {
            f.write_str("NM")
        } else {
            for gamemod in self.iter() {
                f.write_str(gamemod.acronym().as_str())?;
            }

            Ok(())
        }
    }
}

impl IntoIterator for GameModsIntermode {
    type Item = GameModIntermode;
    type IntoIter = IntoGameModsIntermodeIter;

    /// Turns [`GameModsIntermode`] into an iterator over all contained mods.
    ///
    /// Note that the iterator will immediately yield `None` in case of "`NoMod`".
    fn into_iter(self) -> Self::IntoIter {
        IntoGameModsIntermodeIter::new(self.inner.into_iter())
    }
}

impl<'a> IntoIterator for &'a GameModsIntermode {
    type Item = <GameModsIntermodeIter<'a> as Iterator>::Item;
    type IntoIter = GameModsIntermodeIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<M> FromIterator<M> for GameModsIntermode
where
    GameModIntermode: From<M>,
{
    fn from_iter<T: IntoIterator<Item = M>>(iter: T) -> Self {
        Self {
            inner: iter.into_iter().map(GameModIntermode::from).collect(),
        }
    }
}

impl<M> Extend<M> for GameModsIntermode
where
    GameModIntermode: From<M>,
{
    fn extend<T: IntoIterator<Item = M>>(&mut self, iter: T) {
        self.inner
            .extend(iter.into_iter().map(GameModIntermode::from));
    }
}

impl BitOr<GameModIntermode> for GameModsIntermode {
    type Output = Self;

    /// Adds a [`GameModIntermode`] to the [`GameModsIntermode`].
    fn bitor(mut self, rhs: GameModIntermode) -> Self::Output {
        self |= rhs;

        self
    }
}

impl BitOrAssign<GameModIntermode> for GameModsIntermode {
    /// Adds a [`GameModIntermode`] to the [`GameModsIntermode`].
    fn bitor_assign(&mut self, rhs: GameModIntermode) {
        self.insert(rhs);
    }
}

impl Sub<GameModIntermode> for GameModsIntermode {
    type Output = Self;

    /// Removes a [`GameModIntermode`] from the [`GameModsIntermode`]
    fn sub(mut self, rhs: GameModIntermode) -> Self::Output {
        self -= rhs;

        self
    }
}

impl SubAssign<GameModIntermode> for GameModsIntermode {
    /// Removes a [`GameModIntermode`] from the [`GameModsIntermode`]
    fn sub_assign(&mut self, rhs: GameModIntermode) {
        self.remove(rhs);
    }
}

impl From<GameMods> for GameModsIntermode {
    fn from(mods: GameMods) -> Self {
        Self {
            inner: mods.inner.values().map(GameMod::intermode).collect(),
        }
    }
}

impl From<GameModsLegacy> for GameModsIntermode {
    fn from(mods: GameModsLegacy) -> Self {
        mods.to_intermode()
    }
}

impl From<GameModIntermode> for GameModsIntermode {
    fn from(gamemod: GameModIntermode) -> Self {
        let mut mods = Self::new();
        mods.insert(gamemod);

        mods
    }
}

impl FromStr for GameModsIntermode {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_acronyms(s))
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "serde")))]
const _: () = {
    use serde::{
        de::{Deserialize, Deserializer, Error as DeError, SeqAccess, Visitor},
        ser::{Serialize, Serializer},
    };

    use crate::serde::{GameModRaw, MaybeOwnedStr, BITFLAGS_U32};

    impl<'de> Deserialize<'de> for GameModsIntermode {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            struct GameModsIntermodeVisitor;

            impl<'de> Visitor<'de> for GameModsIntermodeVisitor {
                type Value = GameModsIntermode;

                fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                    f.write_str("integer bitflags, mod acronyms, or a sequence of mods")
                }

                fn visit_i64<E: DeError>(self, v: i64) -> Result<Self::Value, E> {
                    let bits = u32::try_from(v).map_err(|_| DeError::custom(BITFLAGS_U32))?;

                    self.visit_u32(bits)
                }

                fn visit_u32<E: DeError>(self, v: u32) -> Result<Self::Value, E> {
                    Ok(GameModsIntermode::from_bits(v))
                }

                fn visit_u64<E: DeError>(self, v: u64) -> Result<Self::Value, E> {
                    let bits = u32::try_from(v).map_err(|_| DeError::custom(BITFLAGS_U32))?;

                    self.visit_u32(bits)
                }

                fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
                    Ok(GameModsIntermode::from_acronyms(v))
                }

                fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                    let mut inner = BTreeSet::new();

                    while let Some(raw) = seq.next_element::<GameModRaw<'_>>()? {
                        fn try_acronym_to_gamemod<E: DeError>(
                            acronym: &MaybeOwnedStr<'_>,
                        ) -> Result<GameModIntermode, E> {
                            acronym
                                .as_str()
                                .parse()
                                .map(GameModIntermode::from_acronym)
                                .map_err(DeError::custom)
                        }

                        let gamemod = match raw {
                            GameModRaw::Bits(bits) => GameModIntermode::try_from_bits(bits)
                                .ok_or_else(|| DeError::custom("invalid bitflags"))?,
                            GameModRaw::Acronym(acronym) => try_acronym_to_gamemod(&acronym)?,
                            GameModRaw::Full { acronym, .. } => try_acronym_to_gamemod(&acronym)?,
                        };

                        inner.insert(gamemod);
                    }

                    Ok(GameModsIntermode { inner })
                }
            }

            d.deserialize_any(GameModsIntermodeVisitor)
        }
    }

    impl Serialize for GameModsIntermode {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            self.inner.serialize(s)
        }
    }
};

#[cfg(feature = "rkyv")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "rkyv")))]
const _: () = {
    use rkyv::{
        rancor::Fallible,
        ser::{Allocator, Writer},
        vec::{ArchivedVec, VecResolver},
        Archive, Archived, Deserialize, Place, Serialize,
    };

    impl Archive for GameModsIntermode {
        type Archived = Archived<Vec<GameModIntermode>>;
        type Resolver = VecResolver;

        fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
            ArchivedVec::resolve_from_len(self.inner.len(), resolver, out);
        }
    }

    impl<S: Fallible + Allocator + Writer + ?Sized> Serialize<S> for GameModsIntermode {
        fn serialize(&self, s: &mut S) -> Result<Self::Resolver, S::Error> {
            ArchivedVec::serialize_from_iter::<GameModIntermode, _, _>(self.inner.iter(), s)
        }
    }

    impl<D: Fallible + ?Sized> Deserialize<GameModsIntermode, D>
        for ArchivedVec<Archived<GameModIntermode>>
    {
        fn deserialize(&self, _: &mut D) -> Result<GameModsIntermode, D::Error> {
            Ok(self.iter().copied().collect())
        }
    }
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push() {
        let mut mods = GameModsIntermode::default();
        mods.insert(GameModIntermode::HardRock);
        mods.insert(GameModIntermode::Wiggle);

        assert_eq!(mods.len(), 2);
        assert_eq!(mods.to_string(), "HRWG");
    }

    #[test]
    fn from_bits_nomod() {
        assert!(GameModsIntermode::from_bits(0).is_empty());
    }

    #[test]
    fn from_bits_valid() {
        let mut expected = GameModsIntermode::default();
        expected.insert(GameModIntermode::Nightcore);
        expected.insert(GameModIntermode::Hidden);

        assert_eq!(GameModsIntermode::from_bits(8 + 64 + 512), expected);
    }

    #[test]
    fn from_bits_invalid_nightcore() {
        assert_eq!(GameModsIntermode::from_bits(512), GameModsIntermode::new());
    }

    #[test]
    fn from_str_nonempty() {
        let mods: GameModsIntermode = "TCWGFLWU".parse().unwrap();

        let mut expected = GameModsIntermode::default();
        expected.insert(GameModIntermode::Flashlight);
        expected.insert(GameModIntermode::Traceable);
        expected.insert(GameModIntermode::Wiggle);
        expected.insert(GameModIntermode::WindUp);

        assert_eq!(mods, expected);
    }

    #[test]
    fn from_str_unknown() {
        let mut iter = "YYQQQ".parse::<GameModsIntermode>().unwrap().into_iter();

        // Since acronyms of length 1 are not valid, it picks the last three
        // characters.
        // Also, "QQQ" comes before "YY" alphabetically so it'll be the first mod.
        assert_eq!(iter.next().unwrap().to_string(), "QQQ");
        assert_eq!(iter.next().unwrap().to_string(), "YY");
        assert!(iter.next().is_none());
    }

    #[test]
    fn contains() {
        let mut mods = GameModsIntermode::default();
        mods.insert(GameModIntermode::Hidden);
        mods.insert(GameModIntermode::HardRock);
        mods.insert(GameModIntermode::Nightcore);

        assert!(mods.contains(GameModIntermode::Nightcore));
        assert!(mods.contains(GameModIntermode::Hidden));
        assert!(!mods.contains(GameModIntermode::DoubleTime));
    }

    #[test]
    fn checked_bits() {
        let mut mods = GameModsIntermode::default();
        mods.insert(GameModIntermode::Hidden);
        mods.insert(GameModIntermode::Traceable);
        mods.insert(GameModIntermode::DoubleTime);

        assert_eq!(mods.checked_bits(), None);
    }

    #[test]
    fn unchecked_bits() {
        let mut mods = GameModsIntermode::default();
        mods.insert(GameModIntermode::Traceable);
        mods.insert(GameModIntermode::DoubleTime);
        mods.insert(GameModIntermode::Hidden);

        assert_eq!(mods.bits(), 72);
    }

    #[test]
    fn intersection() {
        let mut a = GameModsIntermode::default();
        a.insert(GameModIntermode::Hidden);
        a.insert(GameModIntermode::WindUp);
        a.insert(GameModIntermode::HardRock);

        let mut b = GameModsIntermode::default();
        b.insert(GameModIntermode::WindUp);
        b.insert(GameModIntermode::Classic);
        b.insert(GameModIntermode::HardRock);

        let mut intersection = a.intersection(&b);
        assert_eq!(intersection.next(), Some(GameModIntermode::HardRock));
        assert_eq!(intersection.next(), Some(GameModIntermode::WindUp));
        assert_eq!(intersection.next(), None);
    }

    #[cfg(feature = "serde")]
    mod serde {
        use super::*;

        #[test]
        fn deser_str() {
            let json = r#""HDHRWG""#;
            let mods = serde_json::from_str::<GameModsIntermode>(json).unwrap();

            let mut expected = GameModsIntermode::default();
            expected.insert(GameModIntermode::Hidden);
            expected.insert(GameModIntermode::HardRock);
            expected.insert(GameModIntermode::Wiggle);

            assert_eq!(mods, expected);
        }

        #[test]
        fn deser_bits() {
            let json = "1096";
            let mods = serde_json::from_str::<GameModsIntermode>(json).unwrap();

            let mut expected = GameModsIntermode::default();
            expected.insert(GameModIntermode::Hidden);
            expected.insert(GameModIntermode::DoubleTime);
            expected.insert(GameModIntermode::Flashlight);

            assert_eq!(mods, expected);
        }

        #[test]
        fn deser_seq() {
            let json = r#"["WU", "BL", 2, { "acronym": "NS", "settings": { "any": true } }]"#;
            let mods = serde_json::from_str::<GameModsIntermode>(json).unwrap();

            let mut expected = GameModsIntermode::default();
            expected.insert(GameModIntermode::Blinds);
            expected.insert(GameModIntermode::Easy);
            expected.insert(GameModIntermode::WindUp);
            expected.insert(GameModIntermode::NoScope);

            assert_eq!(mods, expected);
        }
    }
}
