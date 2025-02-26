/// The different types of a [`GameMod`]
///
/// [`GameMod`]: super::GameMod
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(
    feature = "rkyv",
    derive(
        rkyv::Archive,
        rkyv::Serialize,
        rkyv::Deserialize,
        rkyv::Portable,
        rkyv::bytecheck::CheckBytes,
    ),
    bytecheck(crate = rkyv::bytecheck),
    rkyv(as = Self),
    repr(u8),
)]
pub enum GameModKind {
    DifficultyReduction,
    DifficultyIncrease,
    Conversion,
    Automation,
    Fun,
    System,
}
