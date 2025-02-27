use std::{borrow::Borrow, cmp::Ordering};

use crate::{GameMod, GameModIntermode, GameMode};

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) struct GameModOrder {
    // `GameMods` should be able to contain the same intermode for multiple
    // modes so we need to keep track of it
    mode: GameMode,
    // In order for `Borrow` to be implementable we need to contain a
    // `GameModIntermode` as opposed to just the `GameModKind` and `Acronym`
    intermode: GameModIntermode,
}

impl From<&GameMod> for GameModOrder {
    fn from(gamemod: &GameMod) -> Self {
        GameModOrder {
            mode: gamemod.mode(),
            intermode: gamemod.intermode(),
        }
    }
}

impl PartialOrd for GameModOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GameModOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        self.mode
            .cmp(&other.mode)
            .then_with(|| self.intermode.cmp(&other.intermode))
    }
}

impl PartialEq<GameModIntermode> for GameModOrder {
    fn eq(&self, other: &GameModIntermode) -> bool {
        self.intermode.eq(other)
    }
}

impl Borrow<GameModIntermode> for GameModOrder {
    fn borrow(&self) -> &GameModIntermode {
        &self.intermode
    }
}

#[cfg(test)]
mod tests {
    use crate::GameMods;

    use super::*;

    #[test]
    fn gamemods_order() {
        let mut mods = GameMods::new();
        mods.insert(GameMod::HiddenOsu(Default::default()));
        mods.insert(GameMod::HiddenMania(Default::default()));
        mods.insert(GameMod::TraceableOsu(Default::default()));
        mods.insert(GameMod::DoubleTimeOsu(Default::default()));

        let mut iter = mods.iter();
        assert!(matches!(iter.next().unwrap(), GameMod::DoubleTimeOsu(_)));
        assert!(matches!(dbg!(iter.next().unwrap()), GameMod::HiddenOsu(_)));
        assert!(matches!(iter.next().unwrap(), GameMod::TraceableOsu(_)));
        assert!(matches!(iter.next().unwrap(), GameMod::HiddenMania(_)));
        assert!(iter.next().is_none());

        assert_eq!(mods.to_string(), "DTHDTCHD");
    }
}
