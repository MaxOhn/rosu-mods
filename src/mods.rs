use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    iter::FromIterator,
    ops::BitOr,
};

use crate::{
    generated_mods::{GameMod, GameModIntermode},
    intersection::{GameModsIntersection, IntersectionInner},
    iter::{GameModsIter, GameModsIterMut, IntoGameModsIter},
    order::GameModOrder,
    Acronym, GameMode, GameModsIntermode, GameModsLegacy,
};

/// Combination of [`GameMod`]s.
#[derive(Clone, Default, PartialEq)]
pub struct GameMods {
    pub(crate) inner: BTreeMap<GameModOrder, GameMod>,
}

impl GameMods {
    /// Returns empty mods i.e. "`NoMod`"
    pub const fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    /// Return the accumulated bit values of all contained mods.
    ///
    /// Mods that don't have bit values will be ignored.
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    ///
    /// # Example
    /// ```rust
    /// # use rosu_mods::{GameMod, GameMods};
    /// # let hdhrdtwu: GameMods = [
    /// #   GameMod::HiddenOsu(Default::default()),
    /// #   GameMod::HardRockOsu(Default::default()),
    /// #   GameMod::DoubleTimeOsu(Default::default()),
    /// #   GameMod::WindUpOsu(Default::default()),
    /// # ].into_iter().collect();
    /// # /*
    /// let hdhrdtwu = mods!(Osu: HD HR DT WU);
    /// # */
    /// assert_eq!(hdhrdtwu.bits(), 8 + 16 + 64);
    /// ```
    pub fn bits(&self) -> u32 {
        self.inner
            .values()
            .filter_map(GameMod::bits)
            .fold(0, u32::bitor)
    }

    /// Return the accumulated bit values of all contained mods.
    ///
    /// If any contained mod has no bit value `None` is returned.
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    ///
    /// # Example
    /// ```rust
    /// # use rosu_mods::{GameMod, GameMods};
    /// # let hdhrdt: GameMods = [
    /// #   GameMod::HiddenOsu(Default::default()),
    /// #   GameMod::HardRockOsu(Default::default()),
    /// #   GameMod::DoubleTimeOsu(Default::default()),
    /// # ].into_iter().collect();
    /// # let hdhrdtwu: GameMods = [
    /// #   GameMod::HiddenOsu(Default::default()),
    /// #   GameMod::HardRockOsu(Default::default()),
    /// #   GameMod::DoubleTimeOsu(Default::default()),
    /// #   GameMod::WindUpOsu(Default::default()),
    /// # ].into_iter().collect();
    /// # /*
    /// let hdhrdt = mods!(Osu: HD HR DT);
    /// # */
    /// assert_eq!(hdhrdt.checked_bits(), Some(8 + 16 + 64));
    ///
    /// # /*
    /// let hdhrdtwu = mods!(Osu: HD HR DT WU);
    /// # */
    /// assert_eq!(hdhrdtwu.checked_bits(), None);
    /// ```
    pub fn checked_bits(&self) -> Option<u32> {
        self.inner
            .values()
            .map(GameMod::bits)
            .try_fold(0, |bits, next| Some(next? | bits))
    }

    /// Returns `true` if no mods are contained.
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::{GameMod, GameMods};
    ///
    /// let mut mods = GameMods::new();
    /// assert!(mods.is_empty());
    ///
    /// mods.insert(GameMod::HiddenOsu(Default::default()));
    /// assert!(!mods.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns the amount of contained mods.
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::{GameMod, GameMods};
    ///
    /// # let hdhrdt: GameMods = [
    /// #   GameMod::HiddenCatch(Default::default()),
    /// #   GameMod::HardRockCatch(Default::default()),
    /// #   GameMod::DoubleTimeCatch(Default::default()),
    /// # ].into_iter().collect();
    /// # /*
    /// let hdhrdt = mods!(Catch: HD HR DT);
    /// # */
    /// assert_eq!(hdhrdt.len(), 3);
    ///
    /// let mut nm = GameMods::new();
    /// assert_eq!(nm.len(), 0);
    /// assert_eq!(nm.to_string(), "NM");
    /// ```
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Add a [`GameMod`]
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::{GameMod, GameMods};
    ///
    /// let mut mods = GameMods::new();
    /// assert_eq!(mods.to_string(), "NM");
    ///
    /// mods.insert(GameMod::TraceableOsu(Default::default()));
    /// assert_eq!(mods.to_string(), "TC");
    ///
    /// mods.insert(GameMod::HardRockOsu(Default::default()));
    /// assert_eq!(mods.to_string(), "HRTC");
    /// ```
    pub fn insert(&mut self, gamemod: GameMod) {
        self.inner.insert(GameModOrder::from(&gamemod), gamemod);
    }

    /// Check whether a given [`GameMod`] is contained.
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::GameMod;
    ///
    /// # let hd = rosu_mods::GameMods::from(GameMod::HiddenTaiko(Default::default()));
    /// # /*
    /// let hd = mods!(Taiko: HD);
    /// # */
    /// assert!(hd.contains(&GameMod::HiddenTaiko(Default::default())));
    /// assert!(!hd.contains(&GameMod::HiddenOsu(Default::default())));
    /// ```
    pub fn contains(&self, gamemod: &GameMod) -> bool {
        self.inner.contains_key(&GameModOrder::from(gamemod))
    }

    /// Check whether a given [`GameModIntermode`] is contained.
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::GameModIntermode;
    ///
    /// # let hd = rosu_mods::GameMods::from(rosu_mods::GameMod::HiddenTaiko(Default::default()));
    /// # /*
    /// let hd = mods!(Taiko: HD);
    /// # */
    /// assert!(hd.contains_intermode(GameModIntermode::Hidden));
    /// assert!(!hd.contains_intermode(GameModIntermode::HardRock));
    /// ```
    pub fn contains_intermode<M>(&self, gamemod: M) -> bool
    where
        GameModIntermode: From<M>,
    {
        self.inner.contains_key(&GameModIntermode::from(gamemod))
    }

    /// Check whether any of the given mods are contained.
    ///
    /// Note that this method does not consider the mods' modes so it could
    /// return `true` even if it's a different mode.
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::mods;
    ///
    /// # use rosu_mods::{GameMod, GameMods};
    /// # let hd = GameMods::from(GameMod::HiddenTaiko(Default::default()));
    /// # /*
    /// let hd = mods!(Taiko: HD);
    /// # */
    ///
    /// assert!(hd.contains_any(mods!(HD HR)));
    /// assert!(!hd.contains_any(mods!(HR DT)));
    ///
    /// // Careful: It returns `true` even if it's a different mode
    /// # assert!(hd.contains_any(GameMods::from(GameMod::HiddenOsu(Default::default()))));
    /// # /*
    /// assert!(hd.contains_any(mods!(Osu: HD)));
    /// # */
    /// ```
    pub fn contains_any<I, M>(&self, mods: I) -> bool
    where
        I: IntoIterator<Item = M>,
        GameModIntermode: From<M>,
    {
        mods.into_iter()
            .any(|gamemod| self.contains_intermode(gamemod))
    }

    /// Check whether a given [`Acronym`] is contained.
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::Acronym;
    ///
    /// # use rosu_mods::{GameMod, GameMods};
    /// # let mods: GameMods = [
    /// #   GameMod::NoFailOsu(Default::default()),
    /// #   GameMod::DoubleTimeOsu(Default::default()),
    /// # ].into_iter().collect();
    /// # /*
    /// let mods = mods!(Osu: NF DT);
    /// # */
    ///
    /// let nf = "NF".parse::<Acronym>().unwrap();
    /// assert!(mods.contains_acronym(nf));
    ///
    /// let hd = "HD".parse::<Acronym>().unwrap();
    /// assert!(!mods.contains_acronym(hd));
    /// ```
    pub fn contains_acronym(&self, acronym: Acronym) -> bool {
        self.inner
            .values()
            .any(|gamemod| gamemod.acronym() == acronym)
            || (self.is_empty() && acronym.as_str() == "NM")
    }

    /// Remove a [`GameMod`] and return whether it was contained.
    ///
    /// # Example
    /// ```
    /// use rosu_mods::{GameMod, GameMods};
    ///
    /// # let mut mods: GameMods = [
    /// #   GameMod::DoubleTimeMania(Default::default()),
    /// #   GameMod::MirrorMania(Default::default())
    /// # ].into_iter().collect();
    /// # /*
    /// let mut mods: GameMods = mods!(Mania: DT MR);
    /// #*/
    ///
    /// assert!(mods.remove(&GameMod::MirrorMania(Default::default())));
    /// assert_eq!(mods.to_string(), "DT");
    /// assert!(!mods.remove(&GameMod::DoubleTimeCatch(Default::default())));
    /// ```
    pub fn remove(&mut self, gamemod: &GameMod) -> bool {
        self.inner.remove(&GameModOrder::from(gamemod)).is_some()
    }

    /// Remove a gamemod and return whether it was contained.
    ///
    /// If the same gamemod is contained for multiple modes, only one of them will be removed.
    ///
    /// # Example
    /// ```
    /// use rosu_mods::{mods, GameModIntermode, GameMod, GameMods};
    ///
    /// let mut mods: GameMods = [
    ///     GameMod::HiddenOsu(Default::default()),
    ///     GameMod::HiddenTaiko(Default::default()),
    ///     GameMod::HardRockOsu(Default::default()),
    /// ].into_iter().collect();
    ///
    /// assert_eq!(mods.to_string(), "HDHRHD");
    ///
    /// assert!(mods.remove_intermode(GameModIntermode::Hidden));
    /// assert_eq!(mods.to_string(), "HRHD");
    /// assert!(!mods.remove_intermode(GameModIntermode::DoubleTime));
    /// ```
    pub fn remove_intermode<M>(&mut self, gamemod: M) -> bool
    where
        GameModIntermode: From<M>,
    {
        self.inner
            .remove(&GameModIntermode::from(gamemod))
            .is_some()
    }

    /// Remove all mods contained in the iterator.
    ///
    /// # Example
    /// ```
    /// use rosu_mods::{mods, GameMod, GameMods};
    ///
    /// # let mut mods: GameMods = [
    /// #   GameMod::HiddenOsu(Default::default()),
    /// #   GameMod::HardRockOsu(Default::default()),
    /// #   GameMod::WiggleOsu(Default::default()),
    /// #   GameMod::DoubleTimeOsu(Default::default()),
    /// #   GameMod::BarrelRollOsu(Default::default()),
    /// # ].into_iter().collect();
    /// # /*
    /// let mut mods: GameMods = mods!(Osu: HD HR WG DT BR);
    /// # */
    ///
    /// mods.remove_all([
    ///     GameMod::HiddenOsu(Default::default()),
    ///     GameMod::EasyOsu(Default::default())
    /// ].iter());
    /// assert_eq!(mods.to_string(), "DTHRBRWG");
    ///
    /// mods.remove_all(mods!(Osu: NF WG).iter());
    /// assert_eq!(mods.to_string(), "DTHRBR")
    /// ```
    pub fn remove_all<'m, I>(&mut self, mods: I)
    where
        I: Iterator<Item = &'m GameMod>,
    {
        for gamemod in mods {
            self.remove(gamemod);
        }
    }

    /// Remove all mods contained in the iterator.
    ///
    /// If the same gamemod is contained for multiple modes, each occurence of the gamemod
    /// in the iterator will remove only one of the contained gamemods.
    ///
    /// # Example
    /// ```
    /// use rosu_mods::{mods, GameMod, GameMods};
    ///
    ///  let mut mods: GameMods = [
    ///    GameMod::HiddenOsu(Default::default()),
    ///    GameMod::HardRockOsu(Default::default()),
    ///    GameMod::HardRockCatch(Default::default()),
    ///    GameMod::WiggleOsu(Default::default()),
    ///    GameMod::DoubleTimeOsu(Default::default()),
    ///    GameMod::BarrelRollOsu(Default::default()),
    ///  ].into_iter().collect();
    ///
    /// assert_eq!(mods.to_string(), "DTHDHRBRWGHR");
    /// mods.remove_all_intermode(mods!(HD HR WG));
    /// assert_eq!(mods.to_string(), "DTBRHR");
    /// ```
    pub fn remove_all_intermode<I, M>(&mut self, mods: I)
    where
        I: IntoIterator<Item = M>,
        GameModIntermode: From<M>,
    {
        for gamemod in mods {
            self.remove_intermode(gamemod);
        }
    }

    /// Returns an iterator over all mods that appear in both [`GameMods`].
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::GameMods;
    ///
    /// # use rosu_mods::GameMod;
    /// # let hd = GameMods::from(GameMod::HiddenCatch(Default::default()));
    /// # let hdhr: GameMods = [
    /// #   GameMod::HiddenCatch(Default::default()),
    /// #   GameMod::HardRockCatch(Default::default()),
    /// # ].into_iter().collect();
    /// # /*
    /// let hd = mods!(Catch: HD);
    /// let hdhr = mods!(Catch: HD HR);
    /// # */
    /// let mut intersection = hd.intersection(&hdhr);
    ///
    /// assert_eq!(intersection.next(), Some(&GameMod::HiddenCatch(Default::default())));
    /// assert_eq!(intersection.next(), None);
    /// ```
    // https://github.com/rust-lang/rust/blob/c1d3610ac1ddd1cd605479274047fd0a3f37d220/library/alloc/src/collections/btree/set.rs#L517
    pub fn intersection<'m>(&'m self, other: &'m GameMods) -> GameModsIntersection<'m> {
        let (Some(self_min), Some(self_max)) =
            (self.inner.first_key_value(), self.inner.last_key_value())
        else {
            return GameModsIntersection {
                inner: IntersectionInner::Answer(None),
            };
        };

        let (Some(other_min), Some(other_max)) =
            (other.inner.first_key_value(), other.inner.last_key_value())
        else {
            return GameModsIntersection {
                inner: IntersectionInner::Answer(None),
            };
        };

        GameModsIntersection {
            inner: match (self_min.0.cmp(other_max.0), self_max.0.cmp(other_min.0)) {
                (Ordering::Greater, _) | (_, Ordering::Less) => IntersectionInner::Answer(None),
                (Ordering::Equal, _) => IntersectionInner::Answer(Some(self_min.1)),
                (_, Ordering::Equal) => IntersectionInner::Answer(Some(self_max.1)),
                _ => IntersectionInner::new_stitch(self.inner.iter(), other.inner.iter()),
            },
        }
    }

    /// Check whether the two [`GameMods`] have any common mods.
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::GameMods;
    ///
    /// # use rosu_mods::GameMod;
    /// # let hd = GameMods::from(GameMod::HiddenCatch(Default::default()));
    /// # let hr = GameMods::from(GameMod::HardRockCatch(Default::default()));
    /// # let hdhr: GameMods = [
    /// #   GameMod::HiddenCatch(Default::default()),
    /// #   GameMod::HardRockCatch(Default::default()),
    /// # ].into_iter().collect();
    /// # /*
    /// let hd = mods!(Catch: HD);
    /// let hr = mods!(Catch: HR);
    /// # */
    /// assert!(!hd.intersects(&hr));
    ///
    /// # /*
    /// let hdhr = mods!(Catch: HD HR);
    /// # */
    /// assert!(hd.intersects(&hdhr));
    /// ```
    pub fn intersects(&self, other: &Self) -> bool {
        self.intersection(other).next().is_some()
    }

    /// The clock rate of the [`GameMods`].
    ///
    /// Returns `None` if any contained [`GameMod`] has no single clock rate.
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::GameMod;
    ///
    /// # let hd: rosu_mods::GameMods = [GameMod::HiddenOsu(Default::default())].into_iter().collect();
    /// # /*
    /// let hd = mods!(Osu: HD);
    /// # */
    /// assert_eq!(hd.clock_rate(), Some(1.0));
    ///
    /// let mut hddt = hd;
    /// hddt.insert(GameMod::DoubleTimeOsu(Default::default()));
    /// assert_eq!(hddt.clock_rate(), Some(1.5));
    ///
    /// let mut hddtwu = hddt;
    /// hddtwu.insert(GameMod::WindUpOsu(Default::default()));
    /// assert_eq!(hddtwu.clock_rate(), None);
    /// ```
    pub fn clock_rate(&self) -> Option<f64> {
        self.inner
            .values()
            .map(GameMod::clock_rate)
            .try_fold(1.0, |clock_rate, next| next.map(|next| clock_rate * next))
    }

    /// Tries to create [`GameMods`] from a [`GameModsIntermode`].
    ///
    /// Returns `None` if any contained [`GameModIntermode`] is unknown for the
    /// given [`GameMode`].
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::{mods, GameMods, GameModsIntermode, GameMode};
    ///
    /// let intermode: GameModsIntermode = mods!(DT FI);
    /// let mods = GameMods::try_from_intermode(&intermode, GameMode::Mania).unwrap();
    ///
    /// // The FadeIn mod doesn't exist in Taiko
    /// assert!(GameMods::try_from_intermode(&intermode, GameMode::Taiko).is_none());
    /// ```
    pub fn try_from_intermode(mods: &GameModsIntermode, mode: GameMode) -> Option<Self> {
        mods.try_with_mode(mode)
    }

    /// Create [`GameMods`] from a [`GameModsIntermode`].
    ///
    /// Any contained [`GameModIntermode`] that's unknown for the given
    /// [`GameMode`] will be replaced with `GameModIntermode::Unknown`.
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::{mods, GameMods, GameModsIntermode, GameMode};
    ///
    /// let intermode: GameModsIntermode = mods!(DT FI);
    /// let mods = GameMods::from_intermode(&intermode, GameMode::Mania);
    ///
    /// // The FadeIn mod doesn't exist in Taiko
    /// let dt = GameMods::from_intermode(&intermode, GameMode::Taiko);
    /// ```
    pub fn from_intermode(mods: &GameModsIntermode, mode: GameMode) -> Self {
        mods.with_mode(mode)
    }

    /// Returns an iterator over all contained mods.
    ///
    /// Note that the iterator will immediately yield `None` in case of "`NoMod`".
    pub fn iter(&self) -> GameModsIter<'_> {
        GameModsIter::new(self.inner.values())
    }

    /// Returns an iterator that allows modifying each contained mod.
    ///
    /// Note that the iterator will immediately yield `None` in case of "`NoMod`".
    pub fn iter_mut(&mut self) -> GameModsIterMut<'_> {
        GameModsIterMut::new(self.inner.values_mut())
    }

    /// Checks whether some contained mods exclude other contained mods.
    ///
    /// # Example
    /// ```rust
    /// use rosu_mods::GameMod;
    ///
    /// # let mut mods: rosu_mods::GameMods = [
    /// #   GameMod::EasyOsu(Default::default()),
    /// # ].into_iter().collect();
    /// # /*
    /// let mut mods = mods!(Osu: EZ);
    /// # */
    /// assert!(mods.is_valid());
    ///
    /// mods.insert(GameMod::HardRockOsu(Default::default()));
    /// assert!(!mods.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        for gamemod in self.inner.values() {
            for &acronym in gamemod.incompatible_mods().iter() {
                if self.contains_acronym(acronym) {
                    return false;
                }
            }
        }

        true
    }

    /// Remove all mods that are excluded by other contained mods.
    ///
    /// # Example
    /// ```rust
    /// # let mut mods: rosu_mods::GameMods = [
    /// #   rosu_mods::GameMod::EasyOsu(Default::default()),
    /// #   rosu_mods::GameMod::HardRockOsu(Default::default())
    /// # ].into_iter().collect();
    /// # /*
    /// let mut mods = mods!(Osu: EZ HR);
    /// # */
    /// assert_eq!(mods.to_string(), "EZHR");
    ///
    /// mods.sanitize();
    /// assert_eq!(mods.to_string(), "EZ");
    /// ```
    pub fn sanitize(&mut self) {
        'outer: loop {
            let mods = self.inner.values();

            for gamemod in mods {
                for &excluded in gamemod.incompatible_mods().iter() {
                    let intermode = GameModIntermode::from_acronym(excluded);

                    if self.contains_intermode(intermode) {
                        self.inner.retain(|key, _| *key != intermode);

                        continue 'outer;
                    }
                }
            }

            break;
        }
    }

    /// Turns [`GameMods`] into [`GameModsLegacy`].
    pub fn as_legacy(&self) -> GameModsLegacy {
        GameModsLegacy::from_bits(self.bits())
    }

    /// Attempts to turns [`GameMods`] into [`GameModsLegacy`].
    ///
    /// Returns `None` if any contained [`GameMod`] does not have a bit value.
    pub fn try_as_legacy(&self) -> Option<GameModsLegacy> {
        self.checked_bits().map(GameModsLegacy::from_bits)
    }
}

impl Debug for GameMods {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_list().entries(self.inner.values()).finish()
    }
}

impl Display for GameMods {
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

impl From<GameMod> for GameMods {
    fn from(gamemod: GameMod) -> Self {
        let mut mods = Self::new();
        mods.insert(gamemod);

        mods
    }
}

impl IntoIterator for GameMods {
    type Item = GameMod;
    type IntoIter = IntoGameModsIter;

    /// Turns [`GameMods`] into an iterator over all contained mods.
    ///
    /// Note that the iterator will immediately yield `None` in case of "`NoMod`".
    fn into_iter(self) -> Self::IntoIter {
        IntoGameModsIter::new(self.inner.into_values())
    }
}

impl<'a> IntoIterator for &'a GameMods {
    type Item = <GameModsIter<'a> as Iterator>::Item;
    type IntoIter = GameModsIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut GameMods {
    type Item = <GameModsIterMut<'a> as Iterator>::Item;
    type IntoIter = GameModsIterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl FromIterator<GameMod> for GameMods {
    fn from_iter<T: IntoIterator<Item = GameMod>>(iter: T) -> Self {
        Self {
            inner: iter
                .into_iter()
                .map(|gamemod| (GameModOrder::from(&gamemod), gamemod))
                .collect(),
        }
    }
}

impl Extend<GameMod> for GameMods {
    fn extend<T: IntoIterator<Item = GameMod>>(&mut self, iter: T) {
        let iter = iter
            .into_iter()
            .map(|gamemod| (GameModOrder::from(&gamemod), gamemod));

        self.inner.extend(iter);
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "serde")))]
const _: () = {
    use serde::ser::{Serialize, SerializeSeq, Serializer};

    impl Serialize for GameMods {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut s = s.serialize_seq(Some(self.inner.len()))?;

            for gamemod in self.inner.values() {
                s.serialize_element(gamemod)?;
            }

            s.end()
        }
    }
};

#[cfg(feature = "rkyv")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "rkyv")))]
const _: () = {
    use rkyv::{
        rancor::{Fallible, Source},
        ser::{Allocator, Writer},
        vec::{ArchivedVec, VecResolver},
        Archive, Archived, Deserialize, Place, Serialize,
    };

    impl Archive for GameMods {
        type Archived = Archived<Vec<GameMod>>;
        type Resolver = VecResolver;

        fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
            ArchivedVec::resolve_from_len(self.inner.len(), resolver, out);
        }
    }

    impl<S: Fallible<Error: Source> + Allocator + Writer + ?Sized> Serialize<S> for GameMods {
        fn serialize(&self, s: &mut S) -> Result<Self::Resolver, S::Error> {
            ArchivedVec::serialize_from_iter::<GameMod, _, _>(self.inner.values(), s)
        }
    }

    impl<D: Fallible + ?Sized> Deserialize<GameMods, D> for ArchivedVec<Archived<GameMod>> {
        fn deserialize(&self, deserializer: &mut D) -> Result<GameMods, D::Error> {
            self.iter().map(|m| m.deserialize(deserializer)).collect()
        }
    }
};

#[cfg(test)]
mod tests {
    use crate::generated_mods::DoubleTimeOsu;

    use super::*;

    #[test]
    fn insert_valid() {
        let mut mods = GameMods::new();
        mods.insert(GameMod::HiddenOsu(Default::default()));
        mods.insert(GameMod::HardRockOsu(Default::default()));

        assert_eq!(mods.len(), 2);
        assert_eq!(mods.to_string(), "HDHR");
    }

    #[test]
    fn contains() {
        let mods: GameMods = [
            GameMod::HiddenOsu(Default::default()),
            GameMod::HardRockOsu(Default::default()),
            GameMod::NightcoreOsu(Default::default()),
        ]
        .into_iter()
        .collect();
        assert!(mods.contains_intermode(GameModIntermode::Nightcore));
        assert!(mods.contains_intermode(GameModIntermode::Hidden));
        assert!(!mods.contains_intermode(GameModIntermode::DoubleTime));
    }

    #[test]
    fn checked_bits() {
        let mods: GameMods = [
            GameMod::HiddenOsu(Default::default()),
            GameMod::TraceableOsu(Default::default()),
            GameMod::DoubleTimeOsu(Default::default()),
        ]
        .into_iter()
        .collect();

        assert_eq!(mods.checked_bits(), None);
    }

    #[test]
    fn unchecked_bits() {
        let mods: GameMods = [
            GameMod::TraceableOsu(Default::default()),
            GameMod::DoubleTimeOsu(Default::default()),
            GameMod::HiddenOsu(Default::default()),
        ]
        .into_iter()
        .collect();

        assert_eq!(mods.bits(), 72);
    }

    #[test]
    fn intersection() {
        let a: GameMods = [
            GameMod::HiddenOsu(Default::default()),
            GameMod::WindUpOsu(Default::default()),
            GameMod::HardRockOsu(Default::default()),
        ]
        .into_iter()
        .collect();

        let b: GameMods = [
            GameMod::WindUpOsu(Default::default()),
            GameMod::ClassicOsu(Default::default()),
            GameMod::HardRockOsu(Default::default()),
        ]
        .into_iter()
        .collect();

        let mut iter = a.intersection(&b);
        assert_eq!(
            iter.next().map(GameMod::intermode),
            Some(GameModIntermode::HardRock)
        );
        assert_eq!(
            iter.next().map(GameMod::intermode),
            Some(GameModIntermode::WindUp)
        );
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn clock_rate_unaffected() {
        let mods: GameMods = [
            GameMod::HiddenOsu(Default::default()),
            GameMod::HardRockOsu(Default::default()),
            GameMod::WiggleOsu(Default::default()),
        ]
        .into_iter()
        .collect();

        assert_eq!(mods.clock_rate(), Some(1.0));
    }

    #[test]
    fn clock_rate_speed_change() {
        let mut mods: GameMods = [GameMod::HardRockOsu(Default::default())]
            .into_iter()
            .collect();

        mods.insert(GameMod::DoubleTimeOsu(DoubleTimeOsu {
            speed_change: Some(1.25),
            adjust_pitch: Some(false),
        }));
        assert_eq!(mods.clock_rate(), Some(1.25));
    }

    #[test]
    fn clock_rate_variable() {
        let mods: GameMods = [
            GameMod::HiddenOsu(Default::default()),
            GameMod::WindUpOsu(Default::default()),
        ]
        .into_iter()
        .collect();

        assert_eq!(mods.clock_rate(), None);
    }

    #[test]
    fn sanitize() {
        let mut mods: GameMods = [
            GameMod::BlindsOsu(Default::default()),
            GameMod::FlashlightOsu(Default::default()),
            GameMod::HiddenOsu(Default::default()),
            GameMod::TraceableOsu(Default::default()),
        ]
        .into_iter()
        .collect();

        mods.sanitize();

        assert_eq!(mods.to_string(), "BLHD");
    }
}
