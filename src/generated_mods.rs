//! Each individual [`GameMod`] as defined by osu!lazer.
//!
//! See <https://raw.githubusercontent.com/ppy/osu-web/master/database/mods.json>
//!
//! This file was generated automatically.

#![allow(clippy::all, clippy::pedantic)]

use std::{
    borrow::Borrow,
    cmp::Ordering,
    fmt::{Display, Formatter, Result as FmtResult},
    num::NonZeroU8,
};

use crate::{Acronym, GameMode};

mod all_structs {
    /// Larger circles, more forgiving HP drain, less accuracy required, and three lives!
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct EasyOsu {
        /// Number of extra lives
        pub retries: Option<f64>,
    }
    /// You can't fail, no matter what.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct NoFailOsu {}
    /// Less zoom...
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct HalfTimeOsu {
        /// The actual decrease to apply
        pub speed_change: Option<f64>,
        /// Should pitch be adjusted with speed
        pub adjust_pitch: Option<bool>,
    }
    /// Whoaaaaa...
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct DaycoreOsu {
        /// The actual decrease to apply
        pub speed_change: Option<f64>,
    }
    /// Everything just got a bit harder...
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct HardRockOsu {}
    /// Miss and fail.
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct SuddenDeathOsu {
        ///
        pub fail_on_slider_tail: Option<bool>,
        /// Automatically restarts when failed.
        pub restart: Option<bool>,
    }
    /// SS or quit.
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct PerfectOsu {
        /// Automatically restarts when failed.
        pub restart: Option<bool>,
    }
    /// Zoooooooooom...
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct DoubleTimeOsu {
        /// The actual increase to apply
        pub speed_change: Option<f64>,
        /// Should pitch be adjusted with speed
        pub adjust_pitch: Option<bool>,
    }
    /// Uguuuuuuuu...
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct NightcoreOsu {
        /// The actual increase to apply
        pub speed_change: Option<f64>,
    }
    /// Play with no approach circles and fading circles/sliders.
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct HiddenOsu {
        /// The main object body will not fade when enabled.
        pub only_fade_approach_circles: Option<bool>,
    }
    /// Restricted view area.
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct FlashlightOsu {
        /// Milliseconds until the flashlight reaches the cursor
        pub follow_delay: Option<f64>,
        /// Multiplier applied to the default flashlight size.
        pub size_multiplier: Option<f64>,
        /// Decrease the flashlight size as combo increases.
        pub combo_based_size: Option<bool>,
    }
    /// Play with blinds on your screen.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct BlindsOsu {}
    /// Once you start a slider, follow precisely or get a miss.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct StrictTrackingOsu {}
    /// Fail if your accuracy drops too low!
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct AccuracyChallengeOsu {
        /// Trigger a failure if your accuracy goes below this value.
        pub minimum_accuracy: Option<f64>,
        /// The mode of accuracy that will trigger failure.
        pub accuracy_judge_mode: Option<String>,
        /// Automatically restarts when failed.
        pub restart: Option<bool>,
    }
    /// Practice keeping up with the beat of the song.
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct TargetPracticeOsu {
        /// Use a custom seed instead of a random one
        pub seed: Option<f64>,
        /// Whether a metronome beat should play in the background
        pub metronome: Option<bool>,
    }
    /// Override a beatmap's difficulty settings.
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct DifficultyAdjustOsu {
        /// Override a beatmap's set CS.
        pub circle_size: Option<f64>,
        /// Override a beatmap's set AR.
        pub approach_rate: Option<f64>,
        /// Override a beatmap's set HP.
        pub drain_rate: Option<f64>,
        /// Override a beatmap's set OD.
        pub overall_difficulty: Option<f64>,
        /// Adjust difficulty beyond sane limits.
        pub extended_limits: Option<bool>,
    }
    /// Feeling nostalgic?
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct ClassicOsu {
        /// Scores sliders proportionally to the number of ticks hit.
        pub no_slider_head_accuracy: Option<bool>,
        /// Applies note lock to the full hit window.
        pub classic_note_lock: Option<bool>,
        /// Always plays a slider's tail sample regardless of whether it was hit or not.
        pub always_play_tail_sample: Option<bool>,
        /// Make hit circles fade out into a miss, rather than after it.
        pub fade_hit_circle_early: Option<bool>,
        /// More closely resembles the original HP drain mechanics.
        pub classic_health: Option<bool>,
    }
    /// It never gets boring!
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct RandomOsu {
        /// How sharp angles should be
        pub angle_sharpness: Option<f64>,
        /// Use a custom seed instead of a random one
        pub seed: Option<f64>,
    }
    /// Flip objects on the chosen axes.
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct MirrorOsu {
        ///
        pub reflection: Option<String>,
    }
    /// Don't use the same key twice in a row!
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct AlternateOsu {}
    /// You must only use one key!
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct SingleTapOsu {}
    /// Watch a perfect automated play through the song.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct AutoplayOsu {}
    /// Watch the video without visual distractions.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct CinemaOsu {}
    /// You don't need to click. Give your clicking/tapping fingers a break from the heat of things.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct RelaxOsu {}
    /// Automatic cursor movement - just follow the rhythm.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct AutopilotOsu {}
    /// Spinners will be automatically completed.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct SpunOutOsu {}
    /// Everything rotates. EVERYTHING.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct TransformOsu {}
    /// They just won't stay still...
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct WiggleOsu {
        /// Multiplier applied to the wiggling strength.
        pub strength: Option<f64>,
    }
    /// Circles spin in. No approach circles.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct SpinInOsu {}
    /// Hit them at the right size!
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct GrowOsu {
        /// The initial size multiplier applied to all objects.
        pub start_scale: Option<f64>,
    }
    /// Hit them at the right size!
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct DeflateOsu {
        /// The initial size multiplier applied to all objects.
        pub start_scale: Option<f64>,
    }
    /// Can you keep up?
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct WindUpOsu {
        /// The starting speed of the track
        pub initial_rate: Option<f64>,
        /// The final speed to ramp to
        pub final_rate: Option<f64>,
        /// Should pitch be adjusted with speed
        pub adjust_pitch: Option<bool>,
    }
    /// Sloooow doooown...
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct WindDownOsu {
        /// The starting speed of the track
        pub initial_rate: Option<f64>,
        /// The final speed to ramp to
        pub final_rate: Option<f64>,
        /// Should pitch be adjusted with speed
        pub adjust_pitch: Option<bool>,
    }
    /// Put your faith in the approach circles...
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct TraceableOsu {}
    /// The whole playfield is on a wheel!
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct BarrelRollOsu {
        /// Rotations per minute
        pub spin_speed: Option<f64>,
        /// The direction of rotation
        pub direction: Option<String>,
    }
    /// Never trust the approach circles...
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct ApproachDifferentOsu {
        /// Change the initial size of the approach circle, relative to hit circles.
        pub scale: Option<f64>,
        /// Change the animation style of the approach circles.
        pub style: Option<String>,
    }
    /// Can you still feel the rhythm without music?
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct MutedOsu {
        /// Increase volume as combo builds.
        pub inverse_muting: Option<bool>,
        /// Add a metronome beat to help you keep track of the rhythm.
        pub enable_metronome: Option<bool>,
        /// The combo count at which point the track reaches its final volume.
        pub mute_combo_count: Option<f64>,
        /// Hit sounds are also muted alongside the track.
        pub affects_hit_sounds: Option<bool>,
    }
    /// Where's the cursor?
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct NoScopeOsu {
        /// The combo count at which the cursor becomes completely hidden
        pub hidden_combo_count: Option<f64>,
    }
    /// No need to chase the circles – your cursor is a magnet!
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct MagnetisedOsu {
        /// How strong the pull is.
        pub attraction_strength: Option<f64>,
    }
    /// Hit objects run away!
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct RepelOsu {
        /// How strong the repulsion is.
        pub repulsion_strength: Option<f64>,
    }
    /// Let track speed adapt to you.
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct AdaptiveSpeedOsu {
        /// The starting speed of the track
        pub initial_rate: Option<f64>,
        /// Should pitch be adjusted with speed
        pub adjust_pitch: Option<bool>,
    }
    /// Burn the notes into your memory.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct FreezeFrameOsu {}
    /// Don't let their popping distract you!
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct BubblesOsu {}
    /// Colours hit objects based on the rhythm.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct SynesthesiaOsu {}
    /// 3D. Almost.
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct DepthOsu {
        /// How far away objects appear.
        pub max_depth: Option<f64>,
        /// Whether approach circles should be visible.
        pub show_approach_circles: Option<bool>,
    }
    /// The cursor blooms into.. a larger cursor!
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct BloomOsu {
        /// The combo count at which the cursor reaches its maximum size
        pub max_size_combo_count: Option<f64>,
        /// The multiplier applied to cursor size when combo reaches maximum
        pub max_cursor_size: Option<f64>,
    }
    /// Automatically applied to plays on devices with a touchscreen.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct TouchDeviceOsu {}
    /// Score set on earlier osu! versions with the V2 scoring algorithm active.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct ScoreV2Osu {}
    /// Beats move slower, and less accuracy required!
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct EasyTaiko {}
    /// You can't fail, no matter what.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct NoFailTaiko {}
    /// Less zoom...
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct HalfTimeTaiko {
        /// The actual decrease to apply
        pub speed_change: Option<f64>,
        /// Should pitch be adjusted with speed
        pub adjust_pitch: Option<bool>,
    }
    /// Whoaaaaa...
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct DaycoreTaiko {
        /// The actual decrease to apply
        pub speed_change: Option<f64>,
    }
    /// Everything just got a bit harder...
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct HardRockTaiko {}
    /// Miss and fail.
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct SuddenDeathTaiko {
        /// Automatically restarts when failed.
        pub restart: Option<bool>,
    }
    /// SS or quit.
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct PerfectTaiko {
        /// Automatically restarts when failed.
        pub restart: Option<bool>,
    }
    /// Zoooooooooom...
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct DoubleTimeTaiko {
        /// The actual increase to apply
        pub speed_change: Option<f64>,
        /// Should pitch be adjusted with speed
        pub adjust_pitch: Option<bool>,
    }
    /// Uguuuuuuuu...
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct NightcoreTaiko {
        /// The actual increase to apply
        pub speed_change: Option<f64>,
    }
    /// Beats fade out before you hit them!
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct HiddenTaiko {}
    /// Restricted view area.
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct FlashlightTaiko {
        /// Multiplier applied to the default flashlight size.
        pub size_multiplier: Option<f64>,
        /// Decrease the flashlight size as combo increases.
        pub combo_based_size: Option<bool>,
    }
    /// Fail if your accuracy drops too low!
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct AccuracyChallengeTaiko {
        /// Trigger a failure if your accuracy goes below this value.
        pub minimum_accuracy: Option<f64>,
        /// The mode of accuracy that will trigger failure.
        pub accuracy_judge_mode: Option<String>,
        /// Automatically restarts when failed.
        pub restart: Option<bool>,
    }
    /// Shuffle around the colours!
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct RandomTaiko {
        /// Use a custom seed instead of a random one
        pub seed: Option<f64>,
    }
    /// Override a beatmap's difficulty settings.
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct DifficultyAdjustTaiko {
        /// Adjust a beatmap's set scroll speed
        pub scroll_speed: Option<f64>,
        /// Override a beatmap's set HP.
        pub drain_rate: Option<f64>,
        /// Override a beatmap's set OD.
        pub overall_difficulty: Option<f64>,
        /// Adjust difficulty beyond sane limits.
        pub extended_limits: Option<bool>,
    }
    /// Feeling nostalgic?
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct ClassicTaiko {}
    /// Dons become kats, kats become dons
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct SwapTaiko {}
    /// One key for dons, one key for kats.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct SingleTapTaiko {}
    /// No more tricky speed changes!
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct ConstantSpeedTaiko {}
    /// Watch a perfect automated play through the song.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct AutoplayTaiko {}
    /// Watch the video without visual distractions.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct CinemaTaiko {}
    /// No need to remember which key is correct anymore!
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct RelaxTaiko {}
    /// Can you keep up?
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct WindUpTaiko {
        /// The starting speed of the track
        pub initial_rate: Option<f64>,
        /// The final speed to ramp to
        pub final_rate: Option<f64>,
        /// Should pitch be adjusted with speed
        pub adjust_pitch: Option<bool>,
    }
    /// Sloooow doooown...
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct WindDownTaiko {
        /// The starting speed of the track
        pub initial_rate: Option<f64>,
        /// The final speed to ramp to
        pub final_rate: Option<f64>,
        /// Should pitch be adjusted with speed
        pub adjust_pitch: Option<bool>,
    }
    /// Can you still feel the rhythm without music?
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct MutedTaiko {
        /// Increase volume as combo builds.
        pub inverse_muting: Option<bool>,
        /// Add a metronome beat to help you keep track of the rhythm.
        pub enable_metronome: Option<bool>,
        /// The combo count at which point the track reaches its final volume.
        pub mute_combo_count: Option<f64>,
        /// Hit sounds are also muted alongside the track.
        pub affects_hit_sounds: Option<bool>,
    }
    /// Let track speed adapt to you.
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct AdaptiveSpeedTaiko {
        /// The starting speed of the track
        pub initial_rate: Option<f64>,
        /// Should pitch be adjusted with speed
        pub adjust_pitch: Option<bool>,
    }
    /// Score set on earlier osu! versions with the V2 scoring algorithm active.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct ScoreV2Taiko {}
    /// Larger fruits, more forgiving HP drain, less accuracy required, and three lives!
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct EasyCatch {
        /// Number of extra lives
        pub retries: Option<f64>,
    }
    /// You can't fail, no matter what.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct NoFailCatch {}
    /// Less zoom...
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct HalfTimeCatch {
        /// The actual decrease to apply
        pub speed_change: Option<f64>,
        /// Should pitch be adjusted with speed
        pub adjust_pitch: Option<bool>,
    }
    /// Whoaaaaa...
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct DaycoreCatch {
        /// The actual decrease to apply
        pub speed_change: Option<f64>,
    }
    /// Everything just got a bit harder...
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct HardRockCatch {}
    /// Miss and fail.
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct SuddenDeathCatch {
        /// Automatically restarts when failed.
        pub restart: Option<bool>,
    }
    /// SS or quit.
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct PerfectCatch {
        /// Automatically restarts when failed.
        pub restart: Option<bool>,
    }
    /// Zoooooooooom...
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct DoubleTimeCatch {
        /// The actual increase to apply
        pub speed_change: Option<f64>,
        /// Should pitch be adjusted with speed
        pub adjust_pitch: Option<bool>,
    }
    /// Uguuuuuuuu...
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct NightcoreCatch {
        /// The actual increase to apply
        pub speed_change: Option<f64>,
    }
    /// Play with fading fruits.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct HiddenCatch {}
    /// Restricted view area.
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct FlashlightCatch {
        /// Multiplier applied to the default flashlight size.
        pub size_multiplier: Option<f64>,
        /// Decrease the flashlight size as combo increases.
        pub combo_based_size: Option<bool>,
    }
    /// Fail if your accuracy drops too low!
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct AccuracyChallengeCatch {
        /// Trigger a failure if your accuracy goes below this value.
        pub minimum_accuracy: Option<f64>,
        /// The mode of accuracy that will trigger failure.
        pub accuracy_judge_mode: Option<String>,
        /// Automatically restarts when failed.
        pub restart: Option<bool>,
    }
    /// Override a beatmap's difficulty settings.
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct DifficultyAdjustCatch {
        /// Override a beatmap's set CS.
        pub circle_size: Option<f64>,
        /// Override a beatmap's set AR.
        pub approach_rate: Option<f64>,
        /// Adjust the patterns as if Hard Rock is enabled.
        pub hard_rock_offsets: Option<bool>,
        /// Override a beatmap's set HP.
        pub drain_rate: Option<f64>,
        /// Override a beatmap's set OD.
        pub overall_difficulty: Option<f64>,
        /// Adjust difficulty beyond sane limits.
        pub extended_limits: Option<bool>,
    }
    /// Feeling nostalgic?
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct ClassicCatch {}
    /// Fruits are flipped horizontally.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct MirrorCatch {}
    /// Watch a perfect automated play through the song.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct AutoplayCatch {}
    /// Watch the video without visual distractions.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct CinemaCatch {}
    /// Use the mouse to control the catcher.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct RelaxCatch {}
    /// Can you keep up?
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct WindUpCatch {
        /// The starting speed of the track
        pub initial_rate: Option<f64>,
        /// The final speed to ramp to
        pub final_rate: Option<f64>,
        /// Should pitch be adjusted with speed
        pub adjust_pitch: Option<bool>,
    }
    /// Sloooow doooown...
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct WindDownCatch {
        /// The starting speed of the track
        pub initial_rate: Option<f64>,
        /// The final speed to ramp to
        pub final_rate: Option<f64>,
        /// Should pitch be adjusted with speed
        pub adjust_pitch: Option<bool>,
    }
    /// The fruits are... floating?
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct FloatingFruitsCatch {}
    /// Can you still feel the rhythm without music?
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct MutedCatch {
        /// Increase volume as combo builds.
        pub inverse_muting: Option<bool>,
        /// Add a metronome beat to help you keep track of the rhythm.
        pub enable_metronome: Option<bool>,
        /// The combo count at which point the track reaches its final volume.
        pub mute_combo_count: Option<f64>,
        /// Hit sounds are also muted alongside the track.
        pub affects_hit_sounds: Option<bool>,
    }
    /// Where's the catcher?
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct NoScopeCatch {
        /// The combo count at which the cursor becomes completely hidden
        pub hidden_combo_count: Option<f64>,
    }
    /// Score set on earlier osu! versions with the V2 scoring algorithm active.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct ScoreV2Catch {}
    /// More forgiving HP drain, less accuracy required, and three lives!
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct EasyMania {
        /// Number of extra lives
        pub retries: Option<f64>,
    }
    /// You can't fail, no matter what.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct NoFailMania {}
    /// Less zoom...
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct HalfTimeMania {
        /// The actual decrease to apply
        pub speed_change: Option<f64>,
        /// Should pitch be adjusted with speed
        pub adjust_pitch: Option<bool>,
    }
    /// Whoaaaaa...
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct DaycoreMania {
        /// The actual decrease to apply
        pub speed_change: Option<f64>,
    }
    /// No more timing the end of hold notes.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct NoReleaseMania {}
    /// Everything just got a bit harder...
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct HardRockMania {}
    /// Miss and fail.
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct SuddenDeathMania {
        /// Automatically restarts when failed.
        pub restart: Option<bool>,
    }
    /// SS or quit.
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct PerfectMania {
        /// Automatically restarts when failed.
        pub restart: Option<bool>,
    }
    /// Zoooooooooom...
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct DoubleTimeMania {
        /// The actual increase to apply
        pub speed_change: Option<f64>,
        /// Should pitch be adjusted with speed
        pub adjust_pitch: Option<bool>,
    }
    /// Uguuuuuuuu...
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct NightcoreMania {
        /// The actual increase to apply
        pub speed_change: Option<f64>,
    }
    /// Keys appear out of nowhere!
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct FadeInMania {}
    /// Keys fade out before you hit them!
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct HiddenMania {}
    /// Decrease the playfield's viewing area.
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct CoverMania {
        /// The proportion of playfield height that notes will be hidden for.
        pub coverage: Option<f64>,
        /// The direction on which the cover is applied
        pub direction: Option<String>,
    }
    /// Restricted view area.
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct FlashlightMania {
        /// Multiplier applied to the default flashlight size.
        pub size_multiplier: Option<f64>,
        /// Decrease the flashlight size as combo increases.
        pub combo_based_size: Option<bool>,
    }
    /// Fail if your accuracy drops too low!
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct AccuracyChallengeMania {
        /// Trigger a failure if your accuracy goes below this value.
        pub minimum_accuracy: Option<f64>,
        /// The mode of accuracy that will trigger failure.
        pub accuracy_judge_mode: Option<String>,
        /// Automatically restarts when failed.
        pub restart: Option<bool>,
    }
    /// Shuffle around the keys!
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct RandomMania {
        /// Use a custom seed instead of a random one
        pub seed: Option<f64>,
    }
    /// Double the stages, double the fun!
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct DualStagesMania {}
    /// Notes are flipped horizontally.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct MirrorMania {}
    /// Override a beatmap's difficulty settings.
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct DifficultyAdjustMania {
        /// Override a beatmap's set HP.
        pub drain_rate: Option<f64>,
        /// Override a beatmap's set OD.
        pub overall_difficulty: Option<f64>,
        /// Adjust difficulty beyond sane limits.
        pub extended_limits: Option<bool>,
    }
    /// Feeling nostalgic?
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct ClassicMania {}
    /// Hold the keys. To the beat.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct InvertMania {}
    /// No more tricky speed changes!
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct ConstantSpeedMania {}
    /// Replaces all hold notes with normal notes.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct HoldOffMania {}
    /// Play with one key.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct OneKeyMania {}
    /// Play with two keys.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct TwoKeysMania {}
    /// Play with three keys.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct ThreeKeysMania {}
    /// Play with four keys.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct FourKeysMania {}
    /// Play with five keys.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct FiveKeysMania {}
    /// Play with six keys.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct SixKeysMania {}
    /// Play with seven keys.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct SevenKeysMania {}
    /// Play with eight keys.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct EightKeysMania {}
    /// Play with nine keys.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct NineKeysMania {}
    /// Play with ten keys.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct TenKeysMania {}
    /// Watch a perfect automated play through the song.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct AutoplayMania {}
    /// Watch the video without visual distractions.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct CinemaMania {}
    /// Can you keep up?
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct WindUpMania {
        /// The starting speed of the track
        pub initial_rate: Option<f64>,
        /// The final speed to ramp to
        pub final_rate: Option<f64>,
        /// Should pitch be adjusted with speed
        pub adjust_pitch: Option<bool>,
    }
    /// Sloooow doooown...
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct WindDownMania {
        /// The starting speed of the track
        pub initial_rate: Option<f64>,
        /// The final speed to ramp to
        pub final_rate: Option<f64>,
        /// Should pitch be adjusted with speed
        pub adjust_pitch: Option<bool>,
    }
    /// Can you still feel the rhythm without music?
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct MutedMania {
        /// Increase volume as combo builds.
        pub inverse_muting: Option<bool>,
        /// Add a metronome beat to help you keep track of the rhythm.
        pub enable_metronome: Option<bool>,
        /// The combo count at which point the track reaches its final volume.
        pub mute_combo_count: Option<f64>,
        /// Hit sounds are also muted alongside the track.
        pub affects_hit_sounds: Option<bool>,
    }
    /// Let track speed adapt to you.
    #[derive(Clone, Debug, Default, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
    )]
    pub struct AdaptiveSpeedMania {
        /// The starting speed of the track
        pub initial_rate: Option<f64>,
        /// Should pitch be adjusted with speed
        pub adjust_pitch: Option<bool>,
    }
    /// Score set on earlier osu! versions with the V2 scoring algorithm active.
    #[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
    #[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self), repr(transparent))]
    pub struct ScoreV2Mania {}
    /// Any unknown mod.
    #[derive(Copy, Eq, Clone, Debug, PartialEq, PartialOrd, Ord, Hash)]
    #[cfg_attr(feature = "rkyv",derive(rkyv::Archive,rkyv::Serialize,rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),bytecheck(crate = rkyv::bytecheck),rkyv(as = Self),repr(transparent),)]
    pub struct UnknownMod {
        pub acronym: crate::Acronym,
    }
}
use crate::GameModKind;
pub use all_structs::{
    AccuracyChallengeCatch, AccuracyChallengeMania, AccuracyChallengeOsu, AccuracyChallengeTaiko,
    AdaptiveSpeedMania, AdaptiveSpeedOsu, AdaptiveSpeedTaiko, AlternateOsu, ApproachDifferentOsu,
    AutopilotOsu, AutoplayCatch, AutoplayMania, AutoplayOsu, AutoplayTaiko, BarrelRollOsu,
    BlindsOsu, BloomOsu, BubblesOsu, CinemaCatch, CinemaMania, CinemaOsu, CinemaTaiko,
    ClassicCatch, ClassicMania, ClassicOsu, ClassicTaiko, ConstantSpeedMania, ConstantSpeedTaiko,
    CoverMania, DaycoreCatch, DaycoreMania, DaycoreOsu, DaycoreTaiko, DeflateOsu, DepthOsu,
    DifficultyAdjustCatch, DifficultyAdjustMania, DifficultyAdjustOsu, DifficultyAdjustTaiko,
    DoubleTimeCatch, DoubleTimeMania, DoubleTimeOsu, DoubleTimeTaiko, DualStagesMania, EasyCatch,
    EasyMania, EasyOsu, EasyTaiko, EightKeysMania, FadeInMania, FiveKeysMania, FlashlightCatch,
    FlashlightMania, FlashlightOsu, FlashlightTaiko, FloatingFruitsCatch, FourKeysMania,
    FreezeFrameOsu, GrowOsu, HalfTimeCatch, HalfTimeMania, HalfTimeOsu, HalfTimeTaiko,
    HardRockCatch, HardRockMania, HardRockOsu, HardRockTaiko, HiddenCatch, HiddenMania, HiddenOsu,
    HiddenTaiko, HoldOffMania, InvertMania, MagnetisedOsu, MirrorCatch, MirrorMania, MirrorOsu,
    MutedCatch, MutedMania, MutedOsu, MutedTaiko, NightcoreCatch, NightcoreMania, NightcoreOsu,
    NightcoreTaiko, NineKeysMania, NoFailCatch, NoFailMania, NoFailOsu, NoFailTaiko,
    NoReleaseMania, NoScopeCatch, NoScopeOsu, OneKeyMania, PerfectCatch, PerfectMania, PerfectOsu,
    PerfectTaiko, RandomMania, RandomOsu, RandomTaiko, RelaxCatch, RelaxOsu, RelaxTaiko, RepelOsu,
    ScoreV2Catch, ScoreV2Mania, ScoreV2Osu, ScoreV2Taiko, SevenKeysMania, SingleTapOsu,
    SingleTapTaiko, SixKeysMania, SpinInOsu, SpunOutOsu, StrictTrackingOsu, SuddenDeathCatch,
    SuddenDeathMania, SuddenDeathOsu, SuddenDeathTaiko, SwapTaiko, SynesthesiaOsu,
    TargetPracticeOsu, TenKeysMania, ThreeKeysMania, TouchDeviceOsu, TraceableOsu, TransformOsu,
    TwoKeysMania, UnknownMod, WiggleOsu, WindDownCatch, WindDownMania, WindDownOsu, WindDownTaiko,
    WindUpCatch, WindUpMania, WindUpOsu, WindUpTaiko,
};
pub use gamemod::GameMod;
pub use intermode::GameModIntermode;
/// Types for (de)serialization through `rkyv`.
#[cfg(feature = "rkyv")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "rkyv")))]
#[doc(hidden)]
pub mod rkyv {
    pub use super::all_structs::{
        AccuracyChallengeCatchResolver, AccuracyChallengeManiaResolver,
        AccuracyChallengeOsuResolver, AccuracyChallengeTaikoResolver, AdaptiveSpeedManiaResolver,
        AdaptiveSpeedOsuResolver, AdaptiveSpeedTaikoResolver, AlternateOsuResolver,
        ApproachDifferentOsuResolver, ArchivedAccuracyChallengeCatch,
        ArchivedAccuracyChallengeMania, ArchivedAccuracyChallengeOsu,
        ArchivedAccuracyChallengeTaiko, ArchivedAdaptiveSpeedMania, ArchivedAdaptiveSpeedOsu,
        ArchivedAdaptiveSpeedTaiko, ArchivedApproachDifferentOsu, ArchivedBarrelRollOsu,
        ArchivedBloomOsu, ArchivedClassicOsu, ArchivedCoverMania, ArchivedDaycoreCatch,
        ArchivedDaycoreMania, ArchivedDaycoreOsu, ArchivedDaycoreTaiko, ArchivedDeflateOsu,
        ArchivedDepthOsu, ArchivedDifficultyAdjustCatch, ArchivedDifficultyAdjustMania,
        ArchivedDifficultyAdjustOsu, ArchivedDifficultyAdjustTaiko, ArchivedDoubleTimeCatch,
        ArchivedDoubleTimeMania, ArchivedDoubleTimeOsu, ArchivedDoubleTimeTaiko, ArchivedEasyCatch,
        ArchivedEasyMania, ArchivedEasyOsu, ArchivedFlashlightCatch, ArchivedFlashlightMania,
        ArchivedFlashlightOsu, ArchivedFlashlightTaiko, ArchivedGrowOsu, ArchivedHalfTimeCatch,
        ArchivedHalfTimeMania, ArchivedHalfTimeOsu, ArchivedHalfTimeTaiko, ArchivedHiddenOsu,
        ArchivedMagnetisedOsu, ArchivedMirrorOsu, ArchivedMutedCatch, ArchivedMutedMania,
        ArchivedMutedOsu, ArchivedMutedTaiko, ArchivedNightcoreCatch, ArchivedNightcoreMania,
        ArchivedNightcoreOsu, ArchivedNightcoreTaiko, ArchivedNoScopeCatch, ArchivedNoScopeOsu,
        ArchivedPerfectCatch, ArchivedPerfectMania, ArchivedPerfectOsu, ArchivedPerfectTaiko,
        ArchivedRandomMania, ArchivedRandomOsu, ArchivedRandomTaiko, ArchivedRepelOsu,
        ArchivedSuddenDeathCatch, ArchivedSuddenDeathMania, ArchivedSuddenDeathOsu,
        ArchivedSuddenDeathTaiko, ArchivedTargetPracticeOsu, ArchivedWiggleOsu,
        ArchivedWindDownCatch, ArchivedWindDownMania, ArchivedWindDownOsu, ArchivedWindDownTaiko,
        ArchivedWindUpCatch, ArchivedWindUpMania, ArchivedWindUpOsu, ArchivedWindUpTaiko,
        AutopilotOsuResolver, AutoplayCatchResolver, AutoplayManiaResolver, AutoplayOsuResolver,
        AutoplayTaikoResolver, BarrelRollOsuResolver, BlindsOsuResolver, BloomOsuResolver,
        BubblesOsuResolver, CinemaCatchResolver, CinemaManiaResolver, CinemaOsuResolver,
        CinemaTaikoResolver, ClassicCatchResolver, ClassicManiaResolver, ClassicOsuResolver,
        ClassicTaikoResolver, ConstantSpeedManiaResolver, ConstantSpeedTaikoResolver,
        CoverManiaResolver, DaycoreCatchResolver, DaycoreManiaResolver, DaycoreOsuResolver,
        DaycoreTaikoResolver, DeflateOsuResolver, DepthOsuResolver, DifficultyAdjustCatchResolver,
        DifficultyAdjustManiaResolver, DifficultyAdjustOsuResolver, DifficultyAdjustTaikoResolver,
        DoubleTimeCatchResolver, DoubleTimeManiaResolver, DoubleTimeOsuResolver,
        DoubleTimeTaikoResolver, DualStagesManiaResolver, EasyCatchResolver, EasyManiaResolver,
        EasyOsuResolver, EasyTaikoResolver, EightKeysManiaResolver, FadeInManiaResolver,
        FiveKeysManiaResolver, FlashlightCatchResolver, FlashlightManiaResolver,
        FlashlightOsuResolver, FlashlightTaikoResolver, FloatingFruitsCatchResolver,
        FourKeysManiaResolver, FreezeFrameOsuResolver, GrowOsuResolver, HalfTimeCatchResolver,
        HalfTimeManiaResolver, HalfTimeOsuResolver, HalfTimeTaikoResolver, HardRockCatchResolver,
        HardRockManiaResolver, HardRockOsuResolver, HardRockTaikoResolver, HiddenCatchResolver,
        HiddenManiaResolver, HiddenOsuResolver, HiddenTaikoResolver, HoldOffManiaResolver,
        InvertManiaResolver, MagnetisedOsuResolver, MirrorCatchResolver, MirrorManiaResolver,
        MirrorOsuResolver, MutedCatchResolver, MutedManiaResolver, MutedOsuResolver,
        MutedTaikoResolver, NightcoreCatchResolver, NightcoreManiaResolver, NightcoreOsuResolver,
        NightcoreTaikoResolver, NineKeysManiaResolver, NoFailCatchResolver, NoFailManiaResolver,
        NoFailOsuResolver, NoFailTaikoResolver, NoReleaseManiaResolver, NoScopeCatchResolver,
        NoScopeOsuResolver, OneKeyManiaResolver, PerfectCatchResolver, PerfectManiaResolver,
        PerfectOsuResolver, PerfectTaikoResolver, RandomManiaResolver, RandomOsuResolver,
        RandomTaikoResolver, RelaxCatchResolver, RelaxOsuResolver, RelaxTaikoResolver,
        RepelOsuResolver, ScoreV2CatchResolver, ScoreV2ManiaResolver, ScoreV2OsuResolver,
        ScoreV2TaikoResolver, SevenKeysManiaResolver, SingleTapOsuResolver, SingleTapTaikoResolver,
        SixKeysManiaResolver, SpinInOsuResolver, SpunOutOsuResolver, StrictTrackingOsuResolver,
        SuddenDeathCatchResolver, SuddenDeathManiaResolver, SuddenDeathOsuResolver,
        SuddenDeathTaikoResolver, SwapTaikoResolver, SynesthesiaOsuResolver,
        TargetPracticeOsuResolver, TenKeysManiaResolver, ThreeKeysManiaResolver,
        TouchDeviceOsuResolver, TraceableOsuResolver, TransformOsuResolver, TwoKeysManiaResolver,
        UnknownModResolver, WiggleOsuResolver, WindDownCatchResolver, WindDownManiaResolver,
        WindDownOsuResolver, WindDownTaikoResolver, WindUpCatchResolver, WindUpManiaResolver,
        WindUpOsuResolver, WindUpTaikoResolver,
    };
    pub use super::gamemod::{ArchivedGameMod, GameModResolver};
    pub use super::intermode::GameModIntermodeResolver;
    pub use crate::kind::GameModKindResolver;
}
impl EasyOsu {
    /// The acronym of [`EasyOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("EZ") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`EasyOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HR"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("DA"),
            ]
        }
        .into_iter()
    }
    /// The description of [`EasyOsu`]
    pub const fn description() -> &'static str {
        "Larger circles, more forgiving HP drain, less accuracy required, and three lives!"
    }
    /// The [`GameModKind`] of [`EasyOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
    /// Bit value of [`EasyOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        2
    }
}
impl NoFailOsu {
    /// The acronym of [`NoFailOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("NF") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`NoFailOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("CN"),
            ]
        }
        .into_iter()
    }
    /// The description of [`NoFailOsu`]
    pub const fn description() -> &'static str {
        "You can't fail, no matter what."
    }
    /// The [`GameModKind`] of [`NoFailOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
    /// Bit value of [`NoFailOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        1
    }
}
impl HalfTimeOsu {
    /// The acronym of [`HalfTimeOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HT") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HalfTimeOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`HalfTimeOsu`]
    pub const fn description() -> &'static str {
        "Less zoom..."
    }
    /// The [`GameModKind`] of [`HalfTimeOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
    /// Bit value of [`HalfTimeOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        256
    }
}
impl DaycoreOsu {
    /// The acronym of [`DaycoreOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DaycoreOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DaycoreOsu`]
    pub const fn description() -> &'static str {
        "Whoaaaaa..."
    }
    /// The [`GameModKind`] of [`DaycoreOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
}
impl HardRockOsu {
    /// The acronym of [`HardRockOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HR") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HardRockOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("EZ"),
                Acronym::from_str_unchecked("DA"),
                Acronym::from_str_unchecked("MR"),
            ]
        }
        .into_iter()
    }
    /// The description of [`HardRockOsu`]
    pub const fn description() -> &'static str {
        "Everything just got a bit harder..."
    }
    /// The [`GameModKind`] of [`HardRockOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`HardRockOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        16
    }
}
impl SuddenDeathOsu {
    /// The acronym of [`SuddenDeathOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("SD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`SuddenDeathOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("TP"),
                Acronym::from_str_unchecked("CN"),
            ]
        }
        .into_iter()
    }
    /// The description of [`SuddenDeathOsu`]
    pub const fn description() -> &'static str {
        "Miss and fail."
    }
    /// The [`GameModKind`] of [`SuddenDeathOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`SuddenDeathOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        32
    }
}
impl PerfectOsu {
    /// The acronym of [`PerfectOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("PF") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`PerfectOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("CN"),
            ]
        }
        .into_iter()
    }
    /// The description of [`PerfectOsu`]
    pub const fn description() -> &'static str {
        "SS or quit."
    }
    /// The [`GameModKind`] of [`PerfectOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`PerfectOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        16416
    }
}
impl DoubleTimeOsu {
    /// The acronym of [`DoubleTimeOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DT") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DoubleTimeOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DoubleTimeOsu`]
    pub const fn description() -> &'static str {
        "Zoooooooooom..."
    }
    /// The [`GameModKind`] of [`DoubleTimeOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`DoubleTimeOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        64
    }
}
impl NightcoreOsu {
    /// The acronym of [`NightcoreOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("NC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`NightcoreOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`NightcoreOsu`]
    pub const fn description() -> &'static str {
        "Uguuuuuuuu..."
    }
    /// The [`GameModKind`] of [`NightcoreOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`NightcoreOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        576
    }
}
impl HiddenOsu {
    /// The acronym of [`HiddenOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HiddenOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("SI"),
                Acronym::from_str_unchecked("TC"),
                Acronym::from_str_unchecked("AD"),
                Acronym::from_str_unchecked("DP"),
            ]
        }
        .into_iter()
    }
    /// The description of [`HiddenOsu`]
    pub const fn description() -> &'static str {
        "Play with no approach circles and fading circles/sliders."
    }
    /// The [`GameModKind`] of [`HiddenOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`HiddenOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        8
    }
}
impl FlashlightOsu {
    /// The acronym of [`FlashlightOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("FL") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`FlashlightOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("BL"),
                Acronym::from_str_unchecked("BM"),
            ]
        }
        .into_iter()
    }
    /// The description of [`FlashlightOsu`]
    pub const fn description() -> &'static str {
        "Restricted view area."
    }
    /// The [`GameModKind`] of [`FlashlightOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`FlashlightOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        1024
    }
}
impl BlindsOsu {
    /// The acronym of [`BlindsOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("BL") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`BlindsOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe { [Acronym::from_str_unchecked("FL")] }.into_iter()
    }
    /// The description of [`BlindsOsu`]
    pub const fn description() -> &'static str {
        "Play with blinds on your screen."
    }
    /// The [`GameModKind`] of [`BlindsOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
}
impl StrictTrackingOsu {
    /// The acronym of [`StrictTrackingOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("ST") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`StrictTrackingOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("TP"),
                Acronym::from_str_unchecked("CL"),
            ]
        }
        .into_iter()
    }
    /// The description of [`StrictTrackingOsu`]
    pub const fn description() -> &'static str {
        "Once you start a slider, follow precisely or get a miss."
    }
    /// The [`GameModKind`] of [`StrictTrackingOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
}
impl AccuracyChallengeOsu {
    /// The acronym of [`AccuracyChallengeOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AccuracyChallengeOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("EZ"),
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("CN"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AccuracyChallengeOsu`]
    pub const fn description() -> &'static str {
        "Fail if your accuracy drops too low!"
    }
    /// The [`GameModKind`] of [`AccuracyChallengeOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
}
impl TargetPracticeOsu {
    /// The acronym of [`TargetPracticeOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("TP") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`TargetPracticeOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("ST"),
                Acronym::from_str_unchecked("RD"),
                Acronym::from_str_unchecked("SO"),
                Acronym::from_str_unchecked("TC"),
                Acronym::from_str_unchecked("AD"),
                Acronym::from_str_unchecked("DP"),
            ]
        }
        .into_iter()
    }
    /// The description of [`TargetPracticeOsu`]
    pub const fn description() -> &'static str {
        "Practice keeping up with the beat of the song."
    }
    /// The [`GameModKind`] of [`TargetPracticeOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`TargetPracticeOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        8388608
    }
}
impl DifficultyAdjustOsu {
    /// The acronym of [`DifficultyAdjustOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DA") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DifficultyAdjustOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("EZ"),
                Acronym::from_str_unchecked("HR"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DifficultyAdjustOsu`]
    pub const fn description() -> &'static str {
        "Override a beatmap's difficulty settings."
    }
    /// The [`GameModKind`] of [`DifficultyAdjustOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl ClassicOsu {
    /// The acronym of [`ClassicOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("CL") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`ClassicOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe { [Acronym::from_str_unchecked("ST")] }.into_iter()
    }
    /// The description of [`ClassicOsu`]
    pub const fn description() -> &'static str {
        "Feeling nostalgic?"
    }
    /// The [`GameModKind`] of [`ClassicOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl RandomOsu {
    /// The acronym of [`RandomOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("RD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`RandomOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe { [Acronym::from_str_unchecked("TP")] }.into_iter()
    }
    /// The description of [`RandomOsu`]
    pub const fn description() -> &'static str {
        "It never gets boring!"
    }
    /// The [`GameModKind`] of [`RandomOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`RandomOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        2097152
    }
}
impl MirrorOsu {
    /// The acronym of [`MirrorOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("MR") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`MirrorOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe { [Acronym::from_str_unchecked("HR")] }.into_iter()
    }
    /// The description of [`MirrorOsu`]
    pub const fn description() -> &'static str {
        "Flip objects on the chosen axes."
    }
    /// The [`GameModKind`] of [`MirrorOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`MirrorOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        1073741824
    }
}
impl AlternateOsu {
    /// The acronym of [`AlternateOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AL") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AlternateOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("SG"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AlternateOsu`]
    pub const fn description() -> &'static str {
        "Don't use the same key twice in a row!"
    }
    /// The [`GameModKind`] of [`AlternateOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl SingleTapOsu {
    /// The acronym of [`SingleTapOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("SG") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`SingleTapOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("AL"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
            ]
        }
        .into_iter()
    }
    /// The description of [`SingleTapOsu`]
    pub const fn description() -> &'static str {
        "You must only use one key!"
    }
    /// The [`GameModKind`] of [`SingleTapOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl AutoplayOsu {
    /// The acronym of [`AutoplayOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AT") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AutoplayOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("AL"),
                Acronym::from_str_unchecked("SG"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
                Acronym::from_str_unchecked("AP"),
                Acronym::from_str_unchecked("SO"),
                Acronym::from_str_unchecked("MG"),
                Acronym::from_str_unchecked("RP"),
                Acronym::from_str_unchecked("AS"),
                Acronym::from_str_unchecked("TD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AutoplayOsu`]
    pub const fn description() -> &'static str {
        "Watch a perfect automated play through the song."
    }
    /// The [`GameModKind`] of [`AutoplayOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`AutoplayOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        2048
    }
}
impl CinemaOsu {
    /// The acronym of [`CinemaOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("CN") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`CinemaOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("AL"),
                Acronym::from_str_unchecked("SG"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("RX"),
                Acronym::from_str_unchecked("AP"),
                Acronym::from_str_unchecked("SO"),
                Acronym::from_str_unchecked("MG"),
                Acronym::from_str_unchecked("RP"),
                Acronym::from_str_unchecked("AS"),
                Acronym::from_str_unchecked("TD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`CinemaOsu`]
    pub const fn description() -> &'static str {
        "Watch the video without visual distractions."
    }
    /// The [`GameModKind`] of [`CinemaOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`CinemaOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        4194304
    }
}
impl RelaxOsu {
    /// The acronym of [`RelaxOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("RX") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`RelaxOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("AL"),
                Acronym::from_str_unchecked("SG"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("AP"),
                Acronym::from_str_unchecked("MG"),
            ]
        }
        .into_iter()
    }
    /// The description of [`RelaxOsu`]
    pub const fn description() -> &'static str {
        "You don't need to click. Give your clicking/tapping fingers a break from the heat of things."
    }
    /// The [`GameModKind`] of [`RelaxOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`RelaxOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        128
    }
}
impl AutopilotOsu {
    /// The acronym of [`AutopilotOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AP") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AutopilotOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
                Acronym::from_str_unchecked("SO"),
                Acronym::from_str_unchecked("MG"),
                Acronym::from_str_unchecked("RP"),
                Acronym::from_str_unchecked("TD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AutopilotOsu`]
    pub const fn description() -> &'static str {
        "Automatic cursor movement - just follow the rhythm."
    }
    /// The [`GameModKind`] of [`AutopilotOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`AutopilotOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        8192
    }
}
impl SpunOutOsu {
    /// The acronym of [`SpunOutOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("SO") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`SpunOutOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("TP"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("AP"),
            ]
        }
        .into_iter()
    }
    /// The description of [`SpunOutOsu`]
    pub const fn description() -> &'static str {
        "Spinners will be automatically completed."
    }
    /// The [`GameModKind`] of [`SpunOutOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`SpunOutOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        4096
    }
}
impl TransformOsu {
    /// The acronym of [`TransformOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("TR") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`TransformOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("WG"),
                Acronym::from_str_unchecked("MG"),
                Acronym::from_str_unchecked("RP"),
                Acronym::from_str_unchecked("FR"),
                Acronym::from_str_unchecked("DP"),
            ]
        }
        .into_iter()
    }
    /// The description of [`TransformOsu`]
    pub const fn description() -> &'static str {
        "Everything rotates. EVERYTHING."
    }
    /// The [`GameModKind`] of [`TransformOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl WiggleOsu {
    /// The acronym of [`WiggleOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("WG") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`WiggleOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("TR"),
                Acronym::from_str_unchecked("MG"),
                Acronym::from_str_unchecked("RP"),
                Acronym::from_str_unchecked("DP"),
            ]
        }
        .into_iter()
    }
    /// The description of [`WiggleOsu`]
    pub const fn description() -> &'static str {
        "They just won't stay still..."
    }
    /// The [`GameModKind`] of [`WiggleOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl SpinInOsu {
    /// The acronym of [`SpinInOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("SI") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`SpinInOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HD"),
                Acronym::from_str_unchecked("GR"),
                Acronym::from_str_unchecked("DF"),
                Acronym::from_str_unchecked("TC"),
                Acronym::from_str_unchecked("AD"),
                Acronym::from_str_unchecked("DP"),
            ]
        }
        .into_iter()
    }
    /// The description of [`SpinInOsu`]
    pub const fn description() -> &'static str {
        "Circles spin in. No approach circles."
    }
    /// The [`GameModKind`] of [`SpinInOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl GrowOsu {
    /// The acronym of [`GrowOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("GR") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`GrowOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("SI"),
                Acronym::from_str_unchecked("DF"),
                Acronym::from_str_unchecked("TC"),
                Acronym::from_str_unchecked("AD"),
                Acronym::from_str_unchecked("DP"),
            ]
        }
        .into_iter()
    }
    /// The description of [`GrowOsu`]
    pub const fn description() -> &'static str {
        "Hit them at the right size!"
    }
    /// The [`GameModKind`] of [`GrowOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl DeflateOsu {
    /// The acronym of [`DeflateOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DF") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DeflateOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("SI"),
                Acronym::from_str_unchecked("GR"),
                Acronym::from_str_unchecked("TC"),
                Acronym::from_str_unchecked("AD"),
                Acronym::from_str_unchecked("DP"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DeflateOsu`]
    pub const fn description() -> &'static str {
        "Hit them at the right size!"
    }
    /// The [`GameModKind`] of [`DeflateOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl WindUpOsu {
    /// The acronym of [`WindUpOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("WU") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`WindUpOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`WindUpOsu`]
    pub const fn description() -> &'static str {
        "Can you keep up?"
    }
    /// The [`GameModKind`] of [`WindUpOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl WindDownOsu {
    /// The acronym of [`WindDownOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("WD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`WindDownOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`WindDownOsu`]
    pub const fn description() -> &'static str {
        "Sloooow doooown..."
    }
    /// The [`GameModKind`] of [`WindDownOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl TraceableOsu {
    /// The acronym of [`TraceableOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("TC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`TraceableOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HD"),
                Acronym::from_str_unchecked("TP"),
                Acronym::from_str_unchecked("SI"),
                Acronym::from_str_unchecked("GR"),
                Acronym::from_str_unchecked("DF"),
                Acronym::from_str_unchecked("DP"),
            ]
        }
        .into_iter()
    }
    /// The description of [`TraceableOsu`]
    pub const fn description() -> &'static str {
        "Put your faith in the approach circles..."
    }
    /// The [`GameModKind`] of [`TraceableOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl BarrelRollOsu {
    /// The acronym of [`BarrelRollOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("BR") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`BarrelRollOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe { [Acronym::from_str_unchecked("BU")] }.into_iter()
    }
    /// The description of [`BarrelRollOsu`]
    pub const fn description() -> &'static str {
        "The whole playfield is on a wheel!"
    }
    /// The [`GameModKind`] of [`BarrelRollOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl ApproachDifferentOsu {
    /// The acronym of [`ApproachDifferentOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`ApproachDifferentOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HD"),
                Acronym::from_str_unchecked("TP"),
                Acronym::from_str_unchecked("SI"),
                Acronym::from_str_unchecked("GR"),
                Acronym::from_str_unchecked("DF"),
                Acronym::from_str_unchecked("FR"),
            ]
        }
        .into_iter()
    }
    /// The description of [`ApproachDifferentOsu`]
    pub const fn description() -> &'static str {
        "Never trust the approach circles..."
    }
    /// The [`GameModKind`] of [`ApproachDifferentOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl MutedOsu {
    /// The acronym of [`MutedOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("MU") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`MutedOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`MutedOsu`]
    pub const fn description() -> &'static str {
        "Can you still feel the rhythm without music?"
    }
    /// The [`GameModKind`] of [`MutedOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl NoScopeOsu {
    /// The acronym of [`NoScopeOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("NS") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`NoScopeOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe { [Acronym::from_str_unchecked("BM")] }.into_iter()
    }
    /// The description of [`NoScopeOsu`]
    pub const fn description() -> &'static str {
        "Where's the cursor?"
    }
    /// The [`GameModKind`] of [`NoScopeOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl MagnetisedOsu {
    /// The acronym of [`MagnetisedOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("MG") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`MagnetisedOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
                Acronym::from_str_unchecked("AP"),
                Acronym::from_str_unchecked("TR"),
                Acronym::from_str_unchecked("WG"),
                Acronym::from_str_unchecked("RP"),
                Acronym::from_str_unchecked("BU"),
                Acronym::from_str_unchecked("DP"),
            ]
        }
        .into_iter()
    }
    /// The description of [`MagnetisedOsu`]
    pub const fn description() -> &'static str {
        "No need to chase the circles – your cursor is a magnet!"
    }
    /// The [`GameModKind`] of [`MagnetisedOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl RepelOsu {
    /// The acronym of [`RepelOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("RP") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`RepelOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("AP"),
                Acronym::from_str_unchecked("TR"),
                Acronym::from_str_unchecked("WG"),
                Acronym::from_str_unchecked("MG"),
                Acronym::from_str_unchecked("BU"),
                Acronym::from_str_unchecked("DP"),
            ]
        }
        .into_iter()
    }
    /// The description of [`RepelOsu`]
    pub const fn description() -> &'static str {
        "Hit objects run away!"
    }
    /// The [`GameModKind`] of [`RepelOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl AdaptiveSpeedOsu {
    /// The acronym of [`AdaptiveSpeedOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AS") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AdaptiveSpeedOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AdaptiveSpeedOsu`]
    pub const fn description() -> &'static str {
        "Let track speed adapt to you."
    }
    /// The [`GameModKind`] of [`AdaptiveSpeedOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl FreezeFrameOsu {
    /// The acronym of [`FreezeFrameOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("FR") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`FreezeFrameOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("TR"),
                Acronym::from_str_unchecked("AD"),
                Acronym::from_str_unchecked("DP"),
            ]
        }
        .into_iter()
    }
    /// The description of [`FreezeFrameOsu`]
    pub const fn description() -> &'static str {
        "Burn the notes into your memory."
    }
    /// The [`GameModKind`] of [`FreezeFrameOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl BubblesOsu {
    /// The acronym of [`BubblesOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("BU") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`BubblesOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("BR"),
                Acronym::from_str_unchecked("MG"),
                Acronym::from_str_unchecked("RP"),
            ]
        }
        .into_iter()
    }
    /// The description of [`BubblesOsu`]
    pub const fn description() -> &'static str {
        "Don't let their popping distract you!"
    }
    /// The [`GameModKind`] of [`BubblesOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl SynesthesiaOsu {
    /// The acronym of [`SynesthesiaOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("SY") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`SynesthesiaOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`SynesthesiaOsu`]
    pub const fn description() -> &'static str {
        "Colours hit objects based on the rhythm."
    }
    /// The [`GameModKind`] of [`SynesthesiaOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl DepthOsu {
    /// The acronym of [`DepthOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DP") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DepthOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HD"),
                Acronym::from_str_unchecked("TP"),
                Acronym::from_str_unchecked("TR"),
                Acronym::from_str_unchecked("WG"),
                Acronym::from_str_unchecked("SI"),
                Acronym::from_str_unchecked("GR"),
                Acronym::from_str_unchecked("DF"),
                Acronym::from_str_unchecked("TC"),
                Acronym::from_str_unchecked("MG"),
                Acronym::from_str_unchecked("RP"),
                Acronym::from_str_unchecked("FR"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DepthOsu`]
    pub const fn description() -> &'static str {
        "3D. Almost."
    }
    /// The [`GameModKind`] of [`DepthOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl BloomOsu {
    /// The acronym of [`BloomOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("BM") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`BloomOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("FL"),
                Acronym::from_str_unchecked("NS"),
                Acronym::from_str_unchecked("TD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`BloomOsu`]
    pub const fn description() -> &'static str {
        "The cursor blooms into.. a larger cursor!"
    }
    /// The [`GameModKind`] of [`BloomOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl TouchDeviceOsu {
    /// The acronym of [`TouchDeviceOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("TD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`TouchDeviceOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("AP"),
                Acronym::from_str_unchecked("BM"),
            ]
        }
        .into_iter()
    }
    /// The description of [`TouchDeviceOsu`]
    pub const fn description() -> &'static str {
        "Automatically applied to plays on devices with a touchscreen."
    }
    /// The [`GameModKind`] of [`TouchDeviceOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::System
    }
    /// Bit value of [`TouchDeviceOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        4
    }
}
impl ScoreV2Osu {
    /// The acronym of [`ScoreV2Osu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("SV2") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`ScoreV2Osu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`ScoreV2Osu`]
    pub const fn description() -> &'static str {
        "Score set on earlier osu! versions with the V2 scoring algorithm active."
    }
    /// The [`GameModKind`] of [`ScoreV2Osu`]
    pub const fn kind() -> GameModKind {
        GameModKind::System
    }
    /// Bit value of [`ScoreV2Osu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        536870912
    }
}
impl EasyTaiko {
    /// The acronym of [`EasyTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("EZ") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`EasyTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HR"),
                Acronym::from_str_unchecked("DA"),
            ]
        }
        .into_iter()
    }
    /// The description of [`EasyTaiko`]
    pub const fn description() -> &'static str {
        "Beats move slower, and less accuracy required!"
    }
    /// The [`GameModKind`] of [`EasyTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
    /// Bit value of [`EasyTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        2
    }
}
impl NoFailTaiko {
    /// The acronym of [`NoFailTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("NF") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`NoFailTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("CN"),
            ]
        }
        .into_iter()
    }
    /// The description of [`NoFailTaiko`]
    pub const fn description() -> &'static str {
        "You can't fail, no matter what."
    }
    /// The [`GameModKind`] of [`NoFailTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
    /// Bit value of [`NoFailTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        1
    }
}
impl HalfTimeTaiko {
    /// The acronym of [`HalfTimeTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HT") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HalfTimeTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`HalfTimeTaiko`]
    pub const fn description() -> &'static str {
        "Less zoom..."
    }
    /// The [`GameModKind`] of [`HalfTimeTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
    /// Bit value of [`HalfTimeTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        256
    }
}
impl DaycoreTaiko {
    /// The acronym of [`DaycoreTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DaycoreTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DaycoreTaiko`]
    pub const fn description() -> &'static str {
        "Whoaaaaa..."
    }
    /// The [`GameModKind`] of [`DaycoreTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
}
impl HardRockTaiko {
    /// The acronym of [`HardRockTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HR") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HardRockTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("EZ"),
                Acronym::from_str_unchecked("DA"),
            ]
        }
        .into_iter()
    }
    /// The description of [`HardRockTaiko`]
    pub const fn description() -> &'static str {
        "Everything just got a bit harder..."
    }
    /// The [`GameModKind`] of [`HardRockTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`HardRockTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        16
    }
}
impl SuddenDeathTaiko {
    /// The acronym of [`SuddenDeathTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("SD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`SuddenDeathTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("CN"),
            ]
        }
        .into_iter()
    }
    /// The description of [`SuddenDeathTaiko`]
    pub const fn description() -> &'static str {
        "Miss and fail."
    }
    /// The [`GameModKind`] of [`SuddenDeathTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`SuddenDeathTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        32
    }
}
impl PerfectTaiko {
    /// The acronym of [`PerfectTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("PF") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`PerfectTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("CN"),
            ]
        }
        .into_iter()
    }
    /// The description of [`PerfectTaiko`]
    pub const fn description() -> &'static str {
        "SS or quit."
    }
    /// The [`GameModKind`] of [`PerfectTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`PerfectTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        16416
    }
}
impl DoubleTimeTaiko {
    /// The acronym of [`DoubleTimeTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DT") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DoubleTimeTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DoubleTimeTaiko`]
    pub const fn description() -> &'static str {
        "Zoooooooooom..."
    }
    /// The [`GameModKind`] of [`DoubleTimeTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`DoubleTimeTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        64
    }
}
impl NightcoreTaiko {
    /// The acronym of [`NightcoreTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("NC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`NightcoreTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`NightcoreTaiko`]
    pub const fn description() -> &'static str {
        "Uguuuuuuuu..."
    }
    /// The [`GameModKind`] of [`NightcoreTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`NightcoreTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        576
    }
}
impl HiddenTaiko {
    /// The acronym of [`HiddenTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HiddenTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`HiddenTaiko`]
    pub const fn description() -> &'static str {
        "Beats fade out before you hit them!"
    }
    /// The [`GameModKind`] of [`HiddenTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`HiddenTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        8
    }
}
impl FlashlightTaiko {
    /// The acronym of [`FlashlightTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("FL") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`FlashlightTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`FlashlightTaiko`]
    pub const fn description() -> &'static str {
        "Restricted view area."
    }
    /// The [`GameModKind`] of [`FlashlightTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`FlashlightTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        1024
    }
}
impl AccuracyChallengeTaiko {
    /// The acronym of [`AccuracyChallengeTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AccuracyChallengeTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("CN"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AccuracyChallengeTaiko`]
    pub const fn description() -> &'static str {
        "Fail if your accuracy drops too low!"
    }
    /// The [`GameModKind`] of [`AccuracyChallengeTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
}
impl RandomTaiko {
    /// The acronym of [`RandomTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("RD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`RandomTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe { [Acronym::from_str_unchecked("SW")] }.into_iter()
    }
    /// The description of [`RandomTaiko`]
    pub const fn description() -> &'static str {
        "Shuffle around the colours!"
    }
    /// The [`GameModKind`] of [`RandomTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`RandomTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        2097152
    }
}
impl DifficultyAdjustTaiko {
    /// The acronym of [`DifficultyAdjustTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DA") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DifficultyAdjustTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("EZ"),
                Acronym::from_str_unchecked("HR"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DifficultyAdjustTaiko`]
    pub const fn description() -> &'static str {
        "Override a beatmap's difficulty settings."
    }
    /// The [`GameModKind`] of [`DifficultyAdjustTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl ClassicTaiko {
    /// The acronym of [`ClassicTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("CL") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`ClassicTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`ClassicTaiko`]
    pub const fn description() -> &'static str {
        "Feeling nostalgic?"
    }
    /// The [`GameModKind`] of [`ClassicTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl SwapTaiko {
    /// The acronym of [`SwapTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("SW") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`SwapTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe { [Acronym::from_str_unchecked("RD")] }.into_iter()
    }
    /// The description of [`SwapTaiko`]
    pub const fn description() -> &'static str {
        "Dons become kats, kats become dons"
    }
    /// The [`GameModKind`] of [`SwapTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl SingleTapTaiko {
    /// The acronym of [`SingleTapTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("SG") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`SingleTapTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
            ]
        }
        .into_iter()
    }
    /// The description of [`SingleTapTaiko`]
    pub const fn description() -> &'static str {
        "One key for dons, one key for kats."
    }
    /// The [`GameModKind`] of [`SingleTapTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl ConstantSpeedTaiko {
    /// The acronym of [`ConstantSpeedTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("CS") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`ConstantSpeedTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`ConstantSpeedTaiko`]
    pub const fn description() -> &'static str {
        "No more tricky speed changes!"
    }
    /// The [`GameModKind`] of [`ConstantSpeedTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl AutoplayTaiko {
    /// The acronym of [`AutoplayTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AT") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AutoplayTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("SG"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AutoplayTaiko`]
    pub const fn description() -> &'static str {
        "Watch a perfect automated play through the song."
    }
    /// The [`GameModKind`] of [`AutoplayTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`AutoplayTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        2048
    }
}
impl CinemaTaiko {
    /// The acronym of [`CinemaTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("CN") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`CinemaTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("SG"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("RX"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`CinemaTaiko`]
    pub const fn description() -> &'static str {
        "Watch the video without visual distractions."
    }
    /// The [`GameModKind`] of [`CinemaTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`CinemaTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        4194304
    }
}
impl RelaxTaiko {
    /// The acronym of [`RelaxTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("RX") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`RelaxTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("SG"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
            ]
        }
        .into_iter()
    }
    /// The description of [`RelaxTaiko`]
    pub const fn description() -> &'static str {
        "No need to remember which key is correct anymore!"
    }
    /// The [`GameModKind`] of [`RelaxTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`RelaxTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        128
    }
}
impl WindUpTaiko {
    /// The acronym of [`WindUpTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("WU") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`WindUpTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`WindUpTaiko`]
    pub const fn description() -> &'static str {
        "Can you keep up?"
    }
    /// The [`GameModKind`] of [`WindUpTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl WindDownTaiko {
    /// The acronym of [`WindDownTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("WD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`WindDownTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`WindDownTaiko`]
    pub const fn description() -> &'static str {
        "Sloooow doooown..."
    }
    /// The [`GameModKind`] of [`WindDownTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl MutedTaiko {
    /// The acronym of [`MutedTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("MU") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`MutedTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`MutedTaiko`]
    pub const fn description() -> &'static str {
        "Can you still feel the rhythm without music?"
    }
    /// The [`GameModKind`] of [`MutedTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl AdaptiveSpeedTaiko {
    /// The acronym of [`AdaptiveSpeedTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AS") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AdaptiveSpeedTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AdaptiveSpeedTaiko`]
    pub const fn description() -> &'static str {
        "Let track speed adapt to you."
    }
    /// The [`GameModKind`] of [`AdaptiveSpeedTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl ScoreV2Taiko {
    /// The acronym of [`ScoreV2Taiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("SV2") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`ScoreV2Taiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`ScoreV2Taiko`]
    pub const fn description() -> &'static str {
        "Score set on earlier osu! versions with the V2 scoring algorithm active."
    }
    /// The [`GameModKind`] of [`ScoreV2Taiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::System
    }
    /// Bit value of [`ScoreV2Taiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        536870912
    }
}
impl EasyCatch {
    /// The acronym of [`EasyCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("EZ") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`EasyCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HR"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("DA"),
            ]
        }
        .into_iter()
    }
    /// The description of [`EasyCatch`]
    pub const fn description() -> &'static str {
        "Larger fruits, more forgiving HP drain, less accuracy required, and three lives!"
    }
    /// The [`GameModKind`] of [`EasyCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
    /// Bit value of [`EasyCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        2
    }
}
impl NoFailCatch {
    /// The acronym of [`NoFailCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("NF") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`NoFailCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("CN"),
            ]
        }
        .into_iter()
    }
    /// The description of [`NoFailCatch`]
    pub const fn description() -> &'static str {
        "You can't fail, no matter what."
    }
    /// The [`GameModKind`] of [`NoFailCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
    /// Bit value of [`NoFailCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        1
    }
}
impl HalfTimeCatch {
    /// The acronym of [`HalfTimeCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HT") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HalfTimeCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`HalfTimeCatch`]
    pub const fn description() -> &'static str {
        "Less zoom..."
    }
    /// The [`GameModKind`] of [`HalfTimeCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
    /// Bit value of [`HalfTimeCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        256
    }
}
impl DaycoreCatch {
    /// The acronym of [`DaycoreCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DaycoreCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DaycoreCatch`]
    pub const fn description() -> &'static str {
        "Whoaaaaa..."
    }
    /// The [`GameModKind`] of [`DaycoreCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
}
impl HardRockCatch {
    /// The acronym of [`HardRockCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HR") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HardRockCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("EZ"),
                Acronym::from_str_unchecked("DA"),
            ]
        }
        .into_iter()
    }
    /// The description of [`HardRockCatch`]
    pub const fn description() -> &'static str {
        "Everything just got a bit harder..."
    }
    /// The [`GameModKind`] of [`HardRockCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`HardRockCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        16
    }
}
impl SuddenDeathCatch {
    /// The acronym of [`SuddenDeathCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("SD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`SuddenDeathCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("CN"),
            ]
        }
        .into_iter()
    }
    /// The description of [`SuddenDeathCatch`]
    pub const fn description() -> &'static str {
        "Miss and fail."
    }
    /// The [`GameModKind`] of [`SuddenDeathCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`SuddenDeathCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        32
    }
}
impl PerfectCatch {
    /// The acronym of [`PerfectCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("PF") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`PerfectCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("CN"),
            ]
        }
        .into_iter()
    }
    /// The description of [`PerfectCatch`]
    pub const fn description() -> &'static str {
        "SS or quit."
    }
    /// The [`GameModKind`] of [`PerfectCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`PerfectCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        16416
    }
}
impl DoubleTimeCatch {
    /// The acronym of [`DoubleTimeCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DT") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DoubleTimeCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DoubleTimeCatch`]
    pub const fn description() -> &'static str {
        "Zoooooooooom..."
    }
    /// The [`GameModKind`] of [`DoubleTimeCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`DoubleTimeCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        64
    }
}
impl NightcoreCatch {
    /// The acronym of [`NightcoreCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("NC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`NightcoreCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`NightcoreCatch`]
    pub const fn description() -> &'static str {
        "Uguuuuuuuu..."
    }
    /// The [`GameModKind`] of [`NightcoreCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`NightcoreCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        576
    }
}
impl HiddenCatch {
    /// The acronym of [`HiddenCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HiddenCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`HiddenCatch`]
    pub const fn description() -> &'static str {
        "Play with fading fruits."
    }
    /// The [`GameModKind`] of [`HiddenCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`HiddenCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        8
    }
}
impl FlashlightCatch {
    /// The acronym of [`FlashlightCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("FL") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`FlashlightCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`FlashlightCatch`]
    pub const fn description() -> &'static str {
        "Restricted view area."
    }
    /// The [`GameModKind`] of [`FlashlightCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`FlashlightCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        1024
    }
}
impl AccuracyChallengeCatch {
    /// The acronym of [`AccuracyChallengeCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AccuracyChallengeCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("EZ"),
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("CN"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AccuracyChallengeCatch`]
    pub const fn description() -> &'static str {
        "Fail if your accuracy drops too low!"
    }
    /// The [`GameModKind`] of [`AccuracyChallengeCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
}
impl DifficultyAdjustCatch {
    /// The acronym of [`DifficultyAdjustCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DA") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DifficultyAdjustCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("EZ"),
                Acronym::from_str_unchecked("HR"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DifficultyAdjustCatch`]
    pub const fn description() -> &'static str {
        "Override a beatmap's difficulty settings."
    }
    /// The [`GameModKind`] of [`DifficultyAdjustCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl ClassicCatch {
    /// The acronym of [`ClassicCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("CL") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`ClassicCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`ClassicCatch`]
    pub const fn description() -> &'static str {
        "Feeling nostalgic?"
    }
    /// The [`GameModKind`] of [`ClassicCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl MirrorCatch {
    /// The acronym of [`MirrorCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("MR") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`MirrorCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`MirrorCatch`]
    pub const fn description() -> &'static str {
        "Fruits are flipped horizontally."
    }
    /// The [`GameModKind`] of [`MirrorCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`MirrorCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        1073741824
    }
}
impl AutoplayCatch {
    /// The acronym of [`AutoplayCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AT") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AutoplayCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AutoplayCatch`]
    pub const fn description() -> &'static str {
        "Watch a perfect automated play through the song."
    }
    /// The [`GameModKind`] of [`AutoplayCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`AutoplayCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        2048
    }
}
impl CinemaCatch {
    /// The acronym of [`CinemaCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("CN") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`CinemaCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("RX"),
            ]
        }
        .into_iter()
    }
    /// The description of [`CinemaCatch`]
    pub const fn description() -> &'static str {
        "Watch the video without visual distractions."
    }
    /// The [`GameModKind`] of [`CinemaCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`CinemaCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        4194304
    }
}
impl RelaxCatch {
    /// The acronym of [`RelaxCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("RX") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`RelaxCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
            ]
        }
        .into_iter()
    }
    /// The description of [`RelaxCatch`]
    pub const fn description() -> &'static str {
        "Use the mouse to control the catcher."
    }
    /// The [`GameModKind`] of [`RelaxCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`RelaxCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        128
    }
}
impl WindUpCatch {
    /// The acronym of [`WindUpCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("WU") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`WindUpCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`WindUpCatch`]
    pub const fn description() -> &'static str {
        "Can you keep up?"
    }
    /// The [`GameModKind`] of [`WindUpCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl WindDownCatch {
    /// The acronym of [`WindDownCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("WD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`WindDownCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
            ]
        }
        .into_iter()
    }
    /// The description of [`WindDownCatch`]
    pub const fn description() -> &'static str {
        "Sloooow doooown..."
    }
    /// The [`GameModKind`] of [`WindDownCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl FloatingFruitsCatch {
    /// The acronym of [`FloatingFruitsCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("FF") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`FloatingFruitsCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`FloatingFruitsCatch`]
    pub const fn description() -> &'static str {
        "The fruits are... floating?"
    }
    /// The [`GameModKind`] of [`FloatingFruitsCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl MutedCatch {
    /// The acronym of [`MutedCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("MU") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`MutedCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`MutedCatch`]
    pub const fn description() -> &'static str {
        "Can you still feel the rhythm without music?"
    }
    /// The [`GameModKind`] of [`MutedCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl NoScopeCatch {
    /// The acronym of [`NoScopeCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("NS") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`NoScopeCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`NoScopeCatch`]
    pub const fn description() -> &'static str {
        "Where's the catcher?"
    }
    /// The [`GameModKind`] of [`NoScopeCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl ScoreV2Catch {
    /// The acronym of [`ScoreV2Catch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("SV2") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`ScoreV2Catch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`ScoreV2Catch`]
    pub const fn description() -> &'static str {
        "Score set on earlier osu! versions with the V2 scoring algorithm active."
    }
    /// The [`GameModKind`] of [`ScoreV2Catch`]
    pub const fn kind() -> GameModKind {
        GameModKind::System
    }
    /// Bit value of [`ScoreV2Catch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        536870912
    }
}
impl EasyMania {
    /// The acronym of [`EasyMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("EZ") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`EasyMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HR"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("DA"),
            ]
        }
        .into_iter()
    }
    /// The description of [`EasyMania`]
    pub const fn description() -> &'static str {
        "More forgiving HP drain, less accuracy required, and three lives!"
    }
    /// The [`GameModKind`] of [`EasyMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
    /// Bit value of [`EasyMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        2
    }
}
impl NoFailMania {
    /// The acronym of [`NoFailMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("NF") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`NoFailMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("CN"),
            ]
        }
        .into_iter()
    }
    /// The description of [`NoFailMania`]
    pub const fn description() -> &'static str {
        "You can't fail, no matter what."
    }
    /// The [`GameModKind`] of [`NoFailMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
    /// Bit value of [`NoFailMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        1
    }
}
impl HalfTimeMania {
    /// The acronym of [`HalfTimeMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HT") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HalfTimeMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`HalfTimeMania`]
    pub const fn description() -> &'static str {
        "Less zoom..."
    }
    /// The [`GameModKind`] of [`HalfTimeMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
    /// Bit value of [`HalfTimeMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        256
    }
}
impl DaycoreMania {
    /// The acronym of [`DaycoreMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DaycoreMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DaycoreMania`]
    pub const fn description() -> &'static str {
        "Whoaaaaa..."
    }
    /// The [`GameModKind`] of [`DaycoreMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
}
impl NoReleaseMania {
    /// The acronym of [`NoReleaseMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("NR") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`NoReleaseMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe { [Acronym::from_str_unchecked("HO")] }.into_iter()
    }
    /// The description of [`NoReleaseMania`]
    pub const fn description() -> &'static str {
        "No more timing the end of hold notes."
    }
    /// The [`GameModKind`] of [`NoReleaseMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
}
impl HardRockMania {
    /// The acronym of [`HardRockMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HR") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HardRockMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("EZ"),
                Acronym::from_str_unchecked("DA"),
            ]
        }
        .into_iter()
    }
    /// The description of [`HardRockMania`]
    pub const fn description() -> &'static str {
        "Everything just got a bit harder..."
    }
    /// The [`GameModKind`] of [`HardRockMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`HardRockMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        16
    }
}
impl SuddenDeathMania {
    /// The acronym of [`SuddenDeathMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("SD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`SuddenDeathMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("CN"),
            ]
        }
        .into_iter()
    }
    /// The description of [`SuddenDeathMania`]
    pub const fn description() -> &'static str {
        "Miss and fail."
    }
    /// The [`GameModKind`] of [`SuddenDeathMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`SuddenDeathMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        32
    }
}
impl PerfectMania {
    /// The acronym of [`PerfectMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("PF") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`PerfectMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("CN"),
            ]
        }
        .into_iter()
    }
    /// The description of [`PerfectMania`]
    pub const fn description() -> &'static str {
        "SS or quit."
    }
    /// The [`GameModKind`] of [`PerfectMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`PerfectMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        16416
    }
}
impl DoubleTimeMania {
    /// The acronym of [`DoubleTimeMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DT") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DoubleTimeMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DoubleTimeMania`]
    pub const fn description() -> &'static str {
        "Zoooooooooom..."
    }
    /// The [`GameModKind`] of [`DoubleTimeMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`DoubleTimeMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        64
    }
}
impl NightcoreMania {
    /// The acronym of [`NightcoreMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("NC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`NightcoreMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`NightcoreMania`]
    pub const fn description() -> &'static str {
        "Uguuuuuuuu..."
    }
    /// The [`GameModKind`] of [`NightcoreMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`NightcoreMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        576
    }
}
impl FadeInMania {
    /// The acronym of [`FadeInMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("FI") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`FadeInMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HD"),
                Acronym::from_str_unchecked("CO"),
                Acronym::from_str_unchecked("FL"),
            ]
        }
        .into_iter()
    }
    /// The description of [`FadeInMania`]
    pub const fn description() -> &'static str {
        "Keys appear out of nowhere!"
    }
    /// The [`GameModKind`] of [`FadeInMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`FadeInMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        1048576
    }
}
impl HiddenMania {
    /// The acronym of [`HiddenMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HiddenMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("FI"),
                Acronym::from_str_unchecked("CO"),
                Acronym::from_str_unchecked("FL"),
            ]
        }
        .into_iter()
    }
    /// The description of [`HiddenMania`]
    pub const fn description() -> &'static str {
        "Keys fade out before you hit them!"
    }
    /// The [`GameModKind`] of [`HiddenMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`HiddenMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        8
    }
}
impl CoverMania {
    /// The acronym of [`CoverMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("CO") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`CoverMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("FI"),
                Acronym::from_str_unchecked("HD"),
                Acronym::from_str_unchecked("FL"),
            ]
        }
        .into_iter()
    }
    /// The description of [`CoverMania`]
    pub const fn description() -> &'static str {
        "Decrease the playfield's viewing area."
    }
    /// The [`GameModKind`] of [`CoverMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
}
impl FlashlightMania {
    /// The acronym of [`FlashlightMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("FL") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`FlashlightMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("FI"),
                Acronym::from_str_unchecked("HD"),
                Acronym::from_str_unchecked("CO"),
            ]
        }
        .into_iter()
    }
    /// The description of [`FlashlightMania`]
    pub const fn description() -> &'static str {
        "Restricted view area."
    }
    /// The [`GameModKind`] of [`FlashlightMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`FlashlightMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        1024
    }
}
impl AccuracyChallengeMania {
    /// The acronym of [`AccuracyChallengeMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AccuracyChallengeMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("EZ"),
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("CN"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AccuracyChallengeMania`]
    pub const fn description() -> &'static str {
        "Fail if your accuracy drops too low!"
    }
    /// The [`GameModKind`] of [`AccuracyChallengeMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
}
impl RandomMania {
    /// The acronym of [`RandomMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("RD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`RandomMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`RandomMania`]
    pub const fn description() -> &'static str {
        "Shuffle around the keys!"
    }
    /// The [`GameModKind`] of [`RandomMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`RandomMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        2097152
    }
}
impl DualStagesMania {
    /// The acronym of [`DualStagesMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DS") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DualStagesMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`DualStagesMania`]
    pub const fn description() -> &'static str {
        "Double the stages, double the fun!"
    }
    /// The [`GameModKind`] of [`DualStagesMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`DualStagesMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        33554432
    }
}
impl MirrorMania {
    /// The acronym of [`MirrorMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("MR") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`MirrorMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`MirrorMania`]
    pub const fn description() -> &'static str {
        "Notes are flipped horizontally."
    }
    /// The [`GameModKind`] of [`MirrorMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`MirrorMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        1073741824
    }
}
impl DifficultyAdjustMania {
    /// The acronym of [`DifficultyAdjustMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DA") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DifficultyAdjustMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("EZ"),
                Acronym::from_str_unchecked("HR"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DifficultyAdjustMania`]
    pub const fn description() -> &'static str {
        "Override a beatmap's difficulty settings."
    }
    /// The [`GameModKind`] of [`DifficultyAdjustMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl ClassicMania {
    /// The acronym of [`ClassicMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("CL") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`ClassicMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`ClassicMania`]
    pub const fn description() -> &'static str {
        "Feeling nostalgic?"
    }
    /// The [`GameModKind`] of [`ClassicMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl InvertMania {
    /// The acronym of [`InvertMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("IN") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`InvertMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe { [Acronym::from_str_unchecked("HO")] }.into_iter()
    }
    /// The description of [`InvertMania`]
    pub const fn description() -> &'static str {
        "Hold the keys. To the beat."
    }
    /// The [`GameModKind`] of [`InvertMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl ConstantSpeedMania {
    /// The acronym of [`ConstantSpeedMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("CS") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`ConstantSpeedMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`ConstantSpeedMania`]
    pub const fn description() -> &'static str {
        "No more tricky speed changes!"
    }
    /// The [`GameModKind`] of [`ConstantSpeedMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl HoldOffMania {
    /// The acronym of [`HoldOffMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HO") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HoldOffMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NR"),
                Acronym::from_str_unchecked("IN"),
            ]
        }
        .into_iter()
    }
    /// The description of [`HoldOffMania`]
    pub const fn description() -> &'static str {
        "Replaces all hold notes with normal notes."
    }
    /// The [`GameModKind`] of [`HoldOffMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl OneKeyMania {
    /// The acronym of [`OneKeyMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("1K") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`OneKeyMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("2K"),
                Acronym::from_str_unchecked("3K"),
                Acronym::from_str_unchecked("4K"),
                Acronym::from_str_unchecked("5K"),
                Acronym::from_str_unchecked("6K"),
                Acronym::from_str_unchecked("7K"),
                Acronym::from_str_unchecked("8K"),
                Acronym::from_str_unchecked("9K"),
                Acronym::from_str_unchecked("10K"),
            ]
        }
        .into_iter()
    }
    /// The description of [`OneKeyMania`]
    pub const fn description() -> &'static str {
        "Play with one key."
    }
    /// The [`GameModKind`] of [`OneKeyMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`OneKeyMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        67108864
    }
}
impl TwoKeysMania {
    /// The acronym of [`TwoKeysMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("2K") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`TwoKeysMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("1K"),
                Acronym::from_str_unchecked("3K"),
                Acronym::from_str_unchecked("4K"),
                Acronym::from_str_unchecked("5K"),
                Acronym::from_str_unchecked("6K"),
                Acronym::from_str_unchecked("7K"),
                Acronym::from_str_unchecked("8K"),
                Acronym::from_str_unchecked("9K"),
                Acronym::from_str_unchecked("10K"),
            ]
        }
        .into_iter()
    }
    /// The description of [`TwoKeysMania`]
    pub const fn description() -> &'static str {
        "Play with two keys."
    }
    /// The [`GameModKind`] of [`TwoKeysMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`TwoKeysMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        268435456
    }
}
impl ThreeKeysMania {
    /// The acronym of [`ThreeKeysMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("3K") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`ThreeKeysMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("1K"),
                Acronym::from_str_unchecked("2K"),
                Acronym::from_str_unchecked("4K"),
                Acronym::from_str_unchecked("5K"),
                Acronym::from_str_unchecked("6K"),
                Acronym::from_str_unchecked("7K"),
                Acronym::from_str_unchecked("8K"),
                Acronym::from_str_unchecked("9K"),
                Acronym::from_str_unchecked("10K"),
            ]
        }
        .into_iter()
    }
    /// The description of [`ThreeKeysMania`]
    pub const fn description() -> &'static str {
        "Play with three keys."
    }
    /// The [`GameModKind`] of [`ThreeKeysMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`ThreeKeysMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        134217728
    }
}
impl FourKeysMania {
    /// The acronym of [`FourKeysMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("4K") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`FourKeysMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("1K"),
                Acronym::from_str_unchecked("2K"),
                Acronym::from_str_unchecked("3K"),
                Acronym::from_str_unchecked("5K"),
                Acronym::from_str_unchecked("6K"),
                Acronym::from_str_unchecked("7K"),
                Acronym::from_str_unchecked("8K"),
                Acronym::from_str_unchecked("9K"),
                Acronym::from_str_unchecked("10K"),
            ]
        }
        .into_iter()
    }
    /// The description of [`FourKeysMania`]
    pub const fn description() -> &'static str {
        "Play with four keys."
    }
    /// The [`GameModKind`] of [`FourKeysMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`FourKeysMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        32768
    }
}
impl FiveKeysMania {
    /// The acronym of [`FiveKeysMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("5K") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`FiveKeysMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("1K"),
                Acronym::from_str_unchecked("2K"),
                Acronym::from_str_unchecked("3K"),
                Acronym::from_str_unchecked("4K"),
                Acronym::from_str_unchecked("6K"),
                Acronym::from_str_unchecked("7K"),
                Acronym::from_str_unchecked("8K"),
                Acronym::from_str_unchecked("9K"),
                Acronym::from_str_unchecked("10K"),
            ]
        }
        .into_iter()
    }
    /// The description of [`FiveKeysMania`]
    pub const fn description() -> &'static str {
        "Play with five keys."
    }
    /// The [`GameModKind`] of [`FiveKeysMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`FiveKeysMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        65536
    }
}
impl SixKeysMania {
    /// The acronym of [`SixKeysMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("6K") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`SixKeysMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("1K"),
                Acronym::from_str_unchecked("2K"),
                Acronym::from_str_unchecked("3K"),
                Acronym::from_str_unchecked("4K"),
                Acronym::from_str_unchecked("5K"),
                Acronym::from_str_unchecked("7K"),
                Acronym::from_str_unchecked("8K"),
                Acronym::from_str_unchecked("9K"),
                Acronym::from_str_unchecked("10K"),
            ]
        }
        .into_iter()
    }
    /// The description of [`SixKeysMania`]
    pub const fn description() -> &'static str {
        "Play with six keys."
    }
    /// The [`GameModKind`] of [`SixKeysMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`SixKeysMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        131072
    }
}
impl SevenKeysMania {
    /// The acronym of [`SevenKeysMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("7K") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`SevenKeysMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("1K"),
                Acronym::from_str_unchecked("2K"),
                Acronym::from_str_unchecked("3K"),
                Acronym::from_str_unchecked("4K"),
                Acronym::from_str_unchecked("5K"),
                Acronym::from_str_unchecked("6K"),
                Acronym::from_str_unchecked("8K"),
                Acronym::from_str_unchecked("9K"),
                Acronym::from_str_unchecked("10K"),
            ]
        }
        .into_iter()
    }
    /// The description of [`SevenKeysMania`]
    pub const fn description() -> &'static str {
        "Play with seven keys."
    }
    /// The [`GameModKind`] of [`SevenKeysMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`SevenKeysMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        262144
    }
}
impl EightKeysMania {
    /// The acronym of [`EightKeysMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("8K") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`EightKeysMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("1K"),
                Acronym::from_str_unchecked("2K"),
                Acronym::from_str_unchecked("3K"),
                Acronym::from_str_unchecked("4K"),
                Acronym::from_str_unchecked("5K"),
                Acronym::from_str_unchecked("6K"),
                Acronym::from_str_unchecked("7K"),
                Acronym::from_str_unchecked("9K"),
                Acronym::from_str_unchecked("10K"),
            ]
        }
        .into_iter()
    }
    /// The description of [`EightKeysMania`]
    pub const fn description() -> &'static str {
        "Play with eight keys."
    }
    /// The [`GameModKind`] of [`EightKeysMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`EightKeysMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        524288
    }
}
impl NineKeysMania {
    /// The acronym of [`NineKeysMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("9K") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`NineKeysMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("1K"),
                Acronym::from_str_unchecked("2K"),
                Acronym::from_str_unchecked("3K"),
                Acronym::from_str_unchecked("4K"),
                Acronym::from_str_unchecked("5K"),
                Acronym::from_str_unchecked("6K"),
                Acronym::from_str_unchecked("7K"),
                Acronym::from_str_unchecked("8K"),
                Acronym::from_str_unchecked("10K"),
            ]
        }
        .into_iter()
    }
    /// The description of [`NineKeysMania`]
    pub const fn description() -> &'static str {
        "Play with nine keys."
    }
    /// The [`GameModKind`] of [`NineKeysMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`NineKeysMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        16777216
    }
}
impl TenKeysMania {
    /// The acronym of [`TenKeysMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("10K") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`TenKeysMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("1K"),
                Acronym::from_str_unchecked("2K"),
                Acronym::from_str_unchecked("3K"),
                Acronym::from_str_unchecked("4K"),
                Acronym::from_str_unchecked("5K"),
                Acronym::from_str_unchecked("6K"),
                Acronym::from_str_unchecked("7K"),
                Acronym::from_str_unchecked("8K"),
                Acronym::from_str_unchecked("9K"),
            ]
        }
        .into_iter()
    }
    /// The description of [`TenKeysMania`]
    pub const fn description() -> &'static str {
        "Play with ten keys."
    }
    /// The [`GameModKind`] of [`TenKeysMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl AutoplayMania {
    /// The acronym of [`AutoplayMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AT") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AutoplayMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AutoplayMania`]
    pub const fn description() -> &'static str {
        "Watch a perfect automated play through the song."
    }
    /// The [`GameModKind`] of [`AutoplayMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`AutoplayMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        2048
    }
}
impl CinemaMania {
    /// The acronym of [`CinemaMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("CN") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`CinemaMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`CinemaMania`]
    pub const fn description() -> &'static str {
        "Watch the video without visual distractions."
    }
    /// The [`GameModKind`] of [`CinemaMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`CinemaMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        4194304
    }
}
impl WindUpMania {
    /// The acronym of [`WindUpMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("WU") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`WindUpMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`WindUpMania`]
    pub const fn description() -> &'static str {
        "Can you keep up?"
    }
    /// The [`GameModKind`] of [`WindUpMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl WindDownMania {
    /// The acronym of [`WindDownMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("WD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`WindDownMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`WindDownMania`]
    pub const fn description() -> &'static str {
        "Sloooow doooown..."
    }
    /// The [`GameModKind`] of [`WindDownMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl MutedMania {
    /// The acronym of [`MutedMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("MU") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`MutedMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`MutedMania`]
    pub const fn description() -> &'static str {
        "Can you still feel the rhythm without music?"
    }
    /// The [`GameModKind`] of [`MutedMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl AdaptiveSpeedMania {
    /// The acronym of [`AdaptiveSpeedMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AS") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AdaptiveSpeedMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AdaptiveSpeedMania`]
    pub const fn description() -> &'static str {
        "Let track speed adapt to you."
    }
    /// The [`GameModKind`] of [`AdaptiveSpeedMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl ScoreV2Mania {
    /// The acronym of [`ScoreV2Mania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("SV2") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`ScoreV2Mania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`ScoreV2Mania`]
    pub const fn description() -> &'static str {
        "Score set on earlier osu! versions with the V2 scoring algorithm active."
    }
    /// The [`GameModKind`] of [`ScoreV2Mania`]
    pub const fn kind() -> GameModKind {
        GameModKind::System
    }
    /// Bit value of [`ScoreV2Mania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        536870912
    }
}
impl UnknownMod {
    /// The default [`Acronym`] for an unknown mod without specific
    /// acronym.
    pub const UNKNOWN_ACRONYM: Acronym = unsafe { Acronym::from_str_unchecked("??") };
    /// A custom [`Acronym`] for any unknown mod
    pub const fn acronym(self) -> Acronym {
        self.acronym
    }
    /// Returns an empty iterator
    pub const fn incompatible_mods() -> std::iter::Empty<Acronym> {
        std::iter::empty()
    }
    /// A custom description for any unknown mod
    pub const fn description() -> &'static str {
        "Some unknown mod"
    }
    /// A manually assigned [`GameModKind`] for any unknown mod
    pub const fn kind() -> GameModKind {
        GameModKind::System
    }
}
impl Default for UnknownMod {
    fn default() -> Self {
        Self {
            acronym: Self::UNKNOWN_ACRONYM,
        }
    }
}
pub(crate) mod intermode {
    /// A single game mod when the mode is ignored
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    #[cfg_attr(feature = "rkyv",derive(rkyv::Archive,rkyv::Serialize,rkyv::Deserialize,rkyv::Portable,rkyv::bytecheck::CheckBytes,),rkyv(as = Self),bytecheck(crate = rkyv::bytecheck),repr(u8),)]
    #[non_exhaustive]
    pub enum GameModIntermode {
        AccuracyChallenge,
        AdaptiveSpeed,
        Alternate,
        ApproachDifferent,
        Autopilot,
        Autoplay,
        BarrelRoll,
        Blinds,
        Bloom,
        Bubbles,
        Cinema,
        Classic,
        ConstantSpeed,
        Cover,
        Daycore,
        Deflate,
        Depth,
        DifficultyAdjust,
        DoubleTime,
        DualStages,
        Easy,
        EightKeys,
        FadeIn,
        FiveKeys,
        Flashlight,
        FloatingFruits,
        FourKeys,
        FreezeFrame,
        Grow,
        HalfTime,
        HardRock,
        Hidden,
        HoldOff,
        Invert,
        Magnetised,
        Mirror,
        Muted,
        Nightcore,
        NineKeys,
        NoFail,
        NoRelease,
        NoScope,
        OneKey,
        Perfect,
        Random,
        Relax,
        Repel,
        ScoreV2,
        SevenKeys,
        SingleTap,
        SixKeys,
        SpinIn,
        SpunOut,
        StrictTracking,
        SuddenDeath,
        Swap,
        Synesthesia,
        TargetPractice,
        TenKeys,
        ThreeKeys,
        TouchDevice,
        Traceable,
        Transform,
        TwoKeys,
        Wiggle,
        WindDown,
        WindUp,
        Unknown(super::UnknownMod),
    }
}
impl GameModIntermode {
    /// The [`Acronym`] of this [`GameModIntermode`]
    pub const fn acronym(&self) -> Acronym {
        unsafe {
            match self {
                Self::AccuracyChallenge => Acronym::from_str_unchecked("AC"),
                Self::AdaptiveSpeed => Acronym::from_str_unchecked("AS"),
                Self::Alternate => Acronym::from_str_unchecked("AL"),
                Self::ApproachDifferent => Acronym::from_str_unchecked("AD"),
                Self::Autopilot => Acronym::from_str_unchecked("AP"),
                Self::Autoplay => Acronym::from_str_unchecked("AT"),
                Self::BarrelRoll => Acronym::from_str_unchecked("BR"),
                Self::Blinds => Acronym::from_str_unchecked("BL"),
                Self::Bloom => Acronym::from_str_unchecked("BM"),
                Self::Bubbles => Acronym::from_str_unchecked("BU"),
                Self::Cinema => Acronym::from_str_unchecked("CN"),
                Self::Classic => Acronym::from_str_unchecked("CL"),
                Self::ConstantSpeed => Acronym::from_str_unchecked("CS"),
                Self::Cover => Acronym::from_str_unchecked("CO"),
                Self::Daycore => Acronym::from_str_unchecked("DC"),
                Self::Deflate => Acronym::from_str_unchecked("DF"),
                Self::Depth => Acronym::from_str_unchecked("DP"),
                Self::DifficultyAdjust => Acronym::from_str_unchecked("DA"),
                Self::DoubleTime => Acronym::from_str_unchecked("DT"),
                Self::DualStages => Acronym::from_str_unchecked("DS"),
                Self::Easy => Acronym::from_str_unchecked("EZ"),
                Self::EightKeys => Acronym::from_str_unchecked("8K"),
                Self::FadeIn => Acronym::from_str_unchecked("FI"),
                Self::FiveKeys => Acronym::from_str_unchecked("5K"),
                Self::Flashlight => Acronym::from_str_unchecked("FL"),
                Self::FloatingFruits => Acronym::from_str_unchecked("FF"),
                Self::FourKeys => Acronym::from_str_unchecked("4K"),
                Self::FreezeFrame => Acronym::from_str_unchecked("FR"),
                Self::Grow => Acronym::from_str_unchecked("GR"),
                Self::HalfTime => Acronym::from_str_unchecked("HT"),
                Self::HardRock => Acronym::from_str_unchecked("HR"),
                Self::Hidden => Acronym::from_str_unchecked("HD"),
                Self::HoldOff => Acronym::from_str_unchecked("HO"),
                Self::Invert => Acronym::from_str_unchecked("IN"),
                Self::Magnetised => Acronym::from_str_unchecked("MG"),
                Self::Mirror => Acronym::from_str_unchecked("MR"),
                Self::Muted => Acronym::from_str_unchecked("MU"),
                Self::Nightcore => Acronym::from_str_unchecked("NC"),
                Self::NineKeys => Acronym::from_str_unchecked("9K"),
                Self::NoFail => Acronym::from_str_unchecked("NF"),
                Self::NoRelease => Acronym::from_str_unchecked("NR"),
                Self::NoScope => Acronym::from_str_unchecked("NS"),
                Self::OneKey => Acronym::from_str_unchecked("1K"),
                Self::Perfect => Acronym::from_str_unchecked("PF"),
                Self::Random => Acronym::from_str_unchecked("RD"),
                Self::Relax => Acronym::from_str_unchecked("RX"),
                Self::Repel => Acronym::from_str_unchecked("RP"),
                Self::ScoreV2 => Acronym::from_str_unchecked("SV2"),
                Self::SevenKeys => Acronym::from_str_unchecked("7K"),
                Self::SingleTap => Acronym::from_str_unchecked("SG"),
                Self::SixKeys => Acronym::from_str_unchecked("6K"),
                Self::SpinIn => Acronym::from_str_unchecked("SI"),
                Self::SpunOut => Acronym::from_str_unchecked("SO"),
                Self::StrictTracking => Acronym::from_str_unchecked("ST"),
                Self::SuddenDeath => Acronym::from_str_unchecked("SD"),
                Self::Swap => Acronym::from_str_unchecked("SW"),
                Self::Synesthesia => Acronym::from_str_unchecked("SY"),
                Self::TargetPractice => Acronym::from_str_unchecked("TP"),
                Self::TenKeys => Acronym::from_str_unchecked("10K"),
                Self::ThreeKeys => Acronym::from_str_unchecked("3K"),
                Self::TouchDevice => Acronym::from_str_unchecked("TD"),
                Self::Traceable => Acronym::from_str_unchecked("TC"),
                Self::Transform => Acronym::from_str_unchecked("TR"),
                Self::TwoKeys => Acronym::from_str_unchecked("2K"),
                Self::Wiggle => Acronym::from_str_unchecked("WG"),
                Self::WindDown => Acronym::from_str_unchecked("WD"),
                Self::WindUp => Acronym::from_str_unchecked("WU"),
                Self::Unknown(m) => m.acronym(),
            }
        }
    }
    /// Bit value of the [`GameModIntermode`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits(self) -> Option<u32> {
        match self {
            Self::AccuracyChallenge => None,
            Self::AdaptiveSpeed => None,
            Self::Alternate => None,
            Self::ApproachDifferent => None,
            Self::Autopilot => Some(8192),
            Self::Autoplay => Some(2048),
            Self::BarrelRoll => None,
            Self::Blinds => None,
            Self::Bloom => None,
            Self::Bubbles => None,
            Self::Cinema => Some(4194304),
            Self::Classic => None,
            Self::ConstantSpeed => None,
            Self::Cover => None,
            Self::Daycore => None,
            Self::Deflate => None,
            Self::Depth => None,
            Self::DifficultyAdjust => None,
            Self::DoubleTime => Some(64),
            Self::DualStages => Some(33554432),
            Self::Easy => Some(2),
            Self::EightKeys => Some(524288),
            Self::FadeIn => Some(1048576),
            Self::FiveKeys => Some(65536),
            Self::Flashlight => Some(1024),
            Self::FloatingFruits => None,
            Self::FourKeys => Some(32768),
            Self::FreezeFrame => None,
            Self::Grow => None,
            Self::HalfTime => Some(256),
            Self::HardRock => Some(16),
            Self::Hidden => Some(8),
            Self::HoldOff => None,
            Self::Invert => None,
            Self::Magnetised => None,
            Self::Mirror => Some(1073741824),
            Self::Muted => None,
            Self::Nightcore => Some(576),
            Self::NineKeys => Some(16777216),
            Self::NoFail => Some(1),
            Self::NoRelease => None,
            Self::NoScope => None,
            Self::OneKey => Some(67108864),
            Self::Perfect => Some(16416),
            Self::Random => Some(2097152),
            Self::Relax => Some(128),
            Self::Repel => None,
            Self::ScoreV2 => Some(536870912),
            Self::SevenKeys => Some(262144),
            Self::SingleTap => None,
            Self::SixKeys => Some(131072),
            Self::SpinIn => None,
            Self::SpunOut => Some(4096),
            Self::StrictTracking => None,
            Self::SuddenDeath => Some(32),
            Self::Swap => None,
            Self::Synesthesia => None,
            Self::TargetPractice => Some(8388608),
            Self::TenKeys => None,
            Self::ThreeKeys => Some(134217728),
            Self::TouchDevice => Some(4),
            Self::Traceable => None,
            Self::Transform => None,
            Self::TwoKeys => Some(268435456),
            Self::Wiggle => None,
            Self::WindDown => None,
            Self::WindUp => None,
            Self::Unknown(_) => None,
        }
    }
    /// The [`GameModKind`] of this [`GameModIntermode`]
    pub const fn kind(&self) -> GameModKind {
        match self {
            Self::AccuracyChallenge => GameModKind::DifficultyIncrease,
            Self::AdaptiveSpeed => GameModKind::Fun,
            Self::Alternate => GameModKind::Conversion,
            Self::ApproachDifferent => GameModKind::Fun,
            Self::Autopilot => GameModKind::Automation,
            Self::Autoplay => GameModKind::Automation,
            Self::BarrelRoll => GameModKind::Fun,
            Self::Blinds => GameModKind::DifficultyIncrease,
            Self::Bloom => GameModKind::Fun,
            Self::Bubbles => GameModKind::Fun,
            Self::Cinema => GameModKind::Automation,
            Self::Classic => GameModKind::Conversion,
            Self::ConstantSpeed => GameModKind::Conversion,
            Self::Cover => GameModKind::DifficultyIncrease,
            Self::Daycore => GameModKind::DifficultyReduction,
            Self::Deflate => GameModKind::Fun,
            Self::Depth => GameModKind::Fun,
            Self::DifficultyAdjust => GameModKind::Conversion,
            Self::DoubleTime => GameModKind::DifficultyIncrease,
            Self::DualStages => GameModKind::Conversion,
            Self::Easy => GameModKind::DifficultyReduction,
            Self::EightKeys => GameModKind::Conversion,
            Self::FadeIn => GameModKind::DifficultyIncrease,
            Self::FiveKeys => GameModKind::Conversion,
            Self::Flashlight => GameModKind::DifficultyIncrease,
            Self::FloatingFruits => GameModKind::Fun,
            Self::FourKeys => GameModKind::Conversion,
            Self::FreezeFrame => GameModKind::Fun,
            Self::Grow => GameModKind::Fun,
            Self::HalfTime => GameModKind::DifficultyReduction,
            Self::HardRock => GameModKind::DifficultyIncrease,
            Self::Hidden => GameModKind::DifficultyIncrease,
            Self::HoldOff => GameModKind::Conversion,
            Self::Invert => GameModKind::Conversion,
            Self::Magnetised => GameModKind::Fun,
            Self::Mirror => GameModKind::Conversion,
            Self::Muted => GameModKind::Fun,
            Self::Nightcore => GameModKind::DifficultyIncrease,
            Self::NineKeys => GameModKind::Conversion,
            Self::NoFail => GameModKind::DifficultyReduction,
            Self::NoRelease => GameModKind::DifficultyReduction,
            Self::NoScope => GameModKind::Fun,
            Self::OneKey => GameModKind::Conversion,
            Self::Perfect => GameModKind::DifficultyIncrease,
            Self::Random => GameModKind::Conversion,
            Self::Relax => GameModKind::Automation,
            Self::Repel => GameModKind::Fun,
            Self::ScoreV2 => GameModKind::System,
            Self::SevenKeys => GameModKind::Conversion,
            Self::SingleTap => GameModKind::Conversion,
            Self::SixKeys => GameModKind::Conversion,
            Self::SpinIn => GameModKind::Fun,
            Self::SpunOut => GameModKind::Automation,
            Self::StrictTracking => GameModKind::DifficultyIncrease,
            Self::SuddenDeath => GameModKind::DifficultyIncrease,
            Self::Swap => GameModKind::Conversion,
            Self::Synesthesia => GameModKind::Fun,
            Self::TargetPractice => GameModKind::Conversion,
            Self::TenKeys => GameModKind::Conversion,
            Self::ThreeKeys => GameModKind::Conversion,
            Self::TouchDevice => GameModKind::System,
            Self::Traceable => GameModKind::Fun,
            Self::Transform => GameModKind::Fun,
            Self::TwoKeys => GameModKind::Conversion,
            Self::Wiggle => GameModKind::Fun,
            Self::WindDown => GameModKind::Fun,
            Self::WindUp => GameModKind::Fun,
            Self::Unknown(_) => GameModKind::System,
        }
    }
    /// Parse an [`Acronym`] into a [`GameModIntermode`]
    pub fn from_acronym(acronym: Acronym) -> Self {
        match acronym.as_str() {
            "AC" => Self::AccuracyChallenge,
            "AS" => Self::AdaptiveSpeed,
            "AL" => Self::Alternate,
            "AD" => Self::ApproachDifferent,
            "AP" => Self::Autopilot,
            "AT" => Self::Autoplay,
            "BR" => Self::BarrelRoll,
            "BL" => Self::Blinds,
            "BM" => Self::Bloom,
            "BU" => Self::Bubbles,
            "CN" => Self::Cinema,
            "CL" => Self::Classic,
            "CS" => Self::ConstantSpeed,
            "CO" => Self::Cover,
            "DC" => Self::Daycore,
            "DF" => Self::Deflate,
            "DP" => Self::Depth,
            "DA" => Self::DifficultyAdjust,
            "DT" => Self::DoubleTime,
            "DS" => Self::DualStages,
            "EZ" => Self::Easy,
            "8K" => Self::EightKeys,
            "FI" => Self::FadeIn,
            "5K" => Self::FiveKeys,
            "FL" => Self::Flashlight,
            "FF" => Self::FloatingFruits,
            "4K" => Self::FourKeys,
            "FR" => Self::FreezeFrame,
            "GR" => Self::Grow,
            "HT" => Self::HalfTime,
            "HR" => Self::HardRock,
            "HD" => Self::Hidden,
            "HO" => Self::HoldOff,
            "IN" => Self::Invert,
            "MG" => Self::Magnetised,
            "MR" => Self::Mirror,
            "MU" => Self::Muted,
            "NC" => Self::Nightcore,
            "9K" => Self::NineKeys,
            "NF" => Self::NoFail,
            "NR" => Self::NoRelease,
            "NS" => Self::NoScope,
            "1K" => Self::OneKey,
            "PF" => Self::Perfect,
            "RD" => Self::Random,
            "RX" => Self::Relax,
            "RP" => Self::Repel,
            "SV2" => Self::ScoreV2,
            "7K" => Self::SevenKeys,
            "SG" => Self::SingleTap,
            "6K" => Self::SixKeys,
            "SI" => Self::SpinIn,
            "SO" => Self::SpunOut,
            "ST" => Self::StrictTracking,
            "SD" => Self::SuddenDeath,
            "SW" => Self::Swap,
            "SY" => Self::Synesthesia,
            "TP" => Self::TargetPractice,
            "10K" => Self::TenKeys,
            "3K" => Self::ThreeKeys,
            "TD" => Self::TouchDevice,
            "TC" => Self::Traceable,
            "TR" => Self::Transform,
            "2K" => Self::TwoKeys,
            "WG" => Self::Wiggle,
            "WD" => Self::WindDown,
            "WU" => Self::WindUp,
            _ => Self::Unknown(UnknownMod { acronym }),
        }
    }
    /// Try to convert bitvalues into a [`GameModIntermode`]
    pub const fn try_from_bits(bits: u32) -> Option<Self> {
        match bits {
            8192 => Some(Self::Autopilot),
            2048 => Some(Self::Autoplay),
            4194304 => Some(Self::Cinema),
            64 => Some(Self::DoubleTime),
            33554432 => Some(Self::DualStages),
            2 => Some(Self::Easy),
            524288 => Some(Self::EightKeys),
            1048576 => Some(Self::FadeIn),
            65536 => Some(Self::FiveKeys),
            1024 => Some(Self::Flashlight),
            32768 => Some(Self::FourKeys),
            256 => Some(Self::HalfTime),
            16 => Some(Self::HardRock),
            8 => Some(Self::Hidden),
            1073741824 => Some(Self::Mirror),
            576 => Some(Self::Nightcore),
            16777216 => Some(Self::NineKeys),
            1 => Some(Self::NoFail),
            67108864 => Some(Self::OneKey),
            16416 => Some(Self::Perfect),
            2097152 => Some(Self::Random),
            128 => Some(Self::Relax),
            536870912 => Some(Self::ScoreV2),
            262144 => Some(Self::SevenKeys),
            131072 => Some(Self::SixKeys),
            4096 => Some(Self::SpunOut),
            32 => Some(Self::SuddenDeath),
            8388608 => Some(Self::TargetPractice),
            134217728 => Some(Self::ThreeKeys),
            4 => Some(Self::TouchDevice),
            268435456 => Some(Self::TwoKeys),
            _ => None,
        }
    }
}
impl PartialOrd for GameModIntermode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for GameModIntermode {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.bits(), other.bits()) {
            (Some(self_bits), Some(other_bits)) => self_bits.cmp(&other_bits),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => self.acronym().as_str().cmp(other.acronym().as_str()),
        }
    }
}
impl Display for GameModIntermode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.acronym().as_str())
    }
}
impl From<&GameModIntermode> for GameModIntermode {
    fn from(gamemod: &GameModIntermode) -> Self {
        *gamemod
    }
}
impl From<GameMod> for GameModIntermode {
    fn from(gamemod: GameMod) -> Self {
        gamemod.intermode()
    }
}
#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) struct GameModOrder {
    mode: GameMode,
    index: Option<NonZeroU8>,
    intermode: GameModIntermode,
}
impl From<&GameMod> for GameModOrder {
    fn from(gamemod: &GameMod) -> Self {
        const fn inner(gamemod: &GameMod) -> GameModOrder {
            macro_rules! arm {
                ($mode:ident, $gamemod:ident, Some($discriminant:literal), $intermode:ident) => {
                    arm!(
                        $mode,
                        $gamemod,
                        Some(unsafe { NonZeroU8::new_unchecked($discriminant) }),
                        $intermode,
                    )
                };
                ($mode:ident, $gamemod:ident, $index:expr, $intermode:ident $(,)?) => {
                    GameModOrder {
                        mode: GameMode::$mode,
                        index: $index,
                        intermode: GameModIntermode::$intermode,
                    }
                };
            }
            match gamemod {
                GameMod::EasyOsu(_) => arm!(Osu, EasyOsu, Some(2), Easy),
                GameMod::NoFailOsu(_) => arm!(Osu, NoFailOsu, Some(1), NoFail),
                GameMod::HalfTimeOsu(_) => arm!(Osu, HalfTimeOsu, Some(9), HalfTime),
                GameMod::DaycoreOsu(_) => arm!(Osu, DaycoreOsu, None, Daycore),
                GameMod::HardRockOsu(_) => arm!(Osu, HardRockOsu, Some(5), HardRock),
                GameMod::SuddenDeathOsu(_) => arm!(Osu, SuddenDeathOsu, Some(6), SuddenDeath),
                GameMod::PerfectOsu(_) => arm!(Osu, PerfectOsu, Some(15), Perfect),
                GameMod::DoubleTimeOsu(_) => arm!(Osu, DoubleTimeOsu, Some(7), DoubleTime),
                GameMod::NightcoreOsu(_) => arm!(Osu, NightcoreOsu, Some(10), Nightcore),
                GameMod::HiddenOsu(_) => arm!(Osu, HiddenOsu, Some(4), Hidden),
                GameMod::FlashlightOsu(_) => arm!(Osu, FlashlightOsu, Some(11), Flashlight),
                GameMod::BlindsOsu(_) => arm!(Osu, BlindsOsu, None, Blinds),
                GameMod::StrictTrackingOsu(_) => arm!(Osu, StrictTrackingOsu, None, StrictTracking),
                GameMod::AccuracyChallengeOsu(_) => {
                    arm!(Osu, AccuracyChallengeOsu, None, AccuracyChallenge)
                }
                GameMod::TargetPracticeOsu(_) => {
                    arm!(Osu, TargetPracticeOsu, Some(24), TargetPractice)
                }
                GameMod::DifficultyAdjustOsu(_) => {
                    arm!(Osu, DifficultyAdjustOsu, None, DifficultyAdjust)
                }
                GameMod::ClassicOsu(_) => arm!(Osu, ClassicOsu, None, Classic),
                GameMod::RandomOsu(_) => arm!(Osu, RandomOsu, Some(22), Random),
                GameMod::MirrorOsu(_) => arm!(Osu, MirrorOsu, Some(31), Mirror),
                GameMod::AlternateOsu(_) => arm!(Osu, AlternateOsu, None, Alternate),
                GameMod::SingleTapOsu(_) => arm!(Osu, SingleTapOsu, None, SingleTap),
                GameMod::AutoplayOsu(_) => arm!(Osu, AutoplayOsu, Some(12), Autoplay),
                GameMod::CinemaOsu(_) => arm!(Osu, CinemaOsu, Some(23), Cinema),
                GameMod::RelaxOsu(_) => arm!(Osu, RelaxOsu, Some(8), Relax),
                GameMod::AutopilotOsu(_) => arm!(Osu, AutopilotOsu, Some(14), Autopilot),
                GameMod::SpunOutOsu(_) => arm!(Osu, SpunOutOsu, Some(13), SpunOut),
                GameMod::TransformOsu(_) => arm!(Osu, TransformOsu, None, Transform),
                GameMod::WiggleOsu(_) => arm!(Osu, WiggleOsu, None, Wiggle),
                GameMod::SpinInOsu(_) => arm!(Osu, SpinInOsu, None, SpinIn),
                GameMod::GrowOsu(_) => arm!(Osu, GrowOsu, None, Grow),
                GameMod::DeflateOsu(_) => arm!(Osu, DeflateOsu, None, Deflate),
                GameMod::WindUpOsu(_) => arm!(Osu, WindUpOsu, None, WindUp),
                GameMod::WindDownOsu(_) => arm!(Osu, WindDownOsu, None, WindDown),
                GameMod::TraceableOsu(_) => arm!(Osu, TraceableOsu, None, Traceable),
                GameMod::BarrelRollOsu(_) => arm!(Osu, BarrelRollOsu, None, BarrelRoll),
                GameMod::ApproachDifferentOsu(_) => {
                    arm!(Osu, ApproachDifferentOsu, None, ApproachDifferent)
                }
                GameMod::MutedOsu(_) => arm!(Osu, MutedOsu, None, Muted),
                GameMod::NoScopeOsu(_) => arm!(Osu, NoScopeOsu, None, NoScope),
                GameMod::MagnetisedOsu(_) => arm!(Osu, MagnetisedOsu, None, Magnetised),
                GameMod::RepelOsu(_) => arm!(Osu, RepelOsu, None, Repel),
                GameMod::AdaptiveSpeedOsu(_) => arm!(Osu, AdaptiveSpeedOsu, None, AdaptiveSpeed),
                GameMod::FreezeFrameOsu(_) => arm!(Osu, FreezeFrameOsu, None, FreezeFrame),
                GameMod::BubblesOsu(_) => arm!(Osu, BubblesOsu, None, Bubbles),
                GameMod::SynesthesiaOsu(_) => arm!(Osu, SynesthesiaOsu, None, Synesthesia),
                GameMod::DepthOsu(_) => arm!(Osu, DepthOsu, None, Depth),
                GameMod::BloomOsu(_) => arm!(Osu, BloomOsu, None, Bloom),
                GameMod::TouchDeviceOsu(_) => arm!(Osu, TouchDeviceOsu, Some(3), TouchDevice),
                GameMod::ScoreV2Osu(_) => arm!(Osu, ScoreV2Osu, Some(30), ScoreV2),
                GameMod::UnknownOsu(m) => GameModOrder {
                    mode: GameMode::Osu,
                    index: None,
                    intermode: GameModIntermode::Unknown(*m),
                },
                GameMod::EasyTaiko(_) => arm!(Taiko, EasyTaiko, Some(2), Easy),
                GameMod::NoFailTaiko(_) => arm!(Taiko, NoFailTaiko, Some(1), NoFail),
                GameMod::HalfTimeTaiko(_) => arm!(Taiko, HalfTimeTaiko, Some(9), HalfTime),
                GameMod::DaycoreTaiko(_) => arm!(Taiko, DaycoreTaiko, None, Daycore),
                GameMod::HardRockTaiko(_) => arm!(Taiko, HardRockTaiko, Some(5), HardRock),
                GameMod::SuddenDeathTaiko(_) => arm!(Taiko, SuddenDeathTaiko, Some(6), SuddenDeath),
                GameMod::PerfectTaiko(_) => arm!(Taiko, PerfectTaiko, Some(15), Perfect),
                GameMod::DoubleTimeTaiko(_) => arm!(Taiko, DoubleTimeTaiko, Some(7), DoubleTime),
                GameMod::NightcoreTaiko(_) => arm!(Taiko, NightcoreTaiko, Some(10), Nightcore),
                GameMod::HiddenTaiko(_) => arm!(Taiko, HiddenTaiko, Some(4), Hidden),
                GameMod::FlashlightTaiko(_) => arm!(Taiko, FlashlightTaiko, Some(11), Flashlight),
                GameMod::AccuracyChallengeTaiko(_) => {
                    arm!(Taiko, AccuracyChallengeTaiko, None, AccuracyChallenge)
                }
                GameMod::RandomTaiko(_) => arm!(Taiko, RandomTaiko, Some(22), Random),
                GameMod::DifficultyAdjustTaiko(_) => {
                    arm!(Taiko, DifficultyAdjustTaiko, None, DifficultyAdjust)
                }
                GameMod::ClassicTaiko(_) => arm!(Taiko, ClassicTaiko, None, Classic),
                GameMod::SwapTaiko(_) => arm!(Taiko, SwapTaiko, None, Swap),
                GameMod::SingleTapTaiko(_) => arm!(Taiko, SingleTapTaiko, None, SingleTap),
                GameMod::ConstantSpeedTaiko(_) => {
                    arm!(Taiko, ConstantSpeedTaiko, None, ConstantSpeed)
                }
                GameMod::AutoplayTaiko(_) => arm!(Taiko, AutoplayTaiko, Some(12), Autoplay),
                GameMod::CinemaTaiko(_) => arm!(Taiko, CinemaTaiko, Some(23), Cinema),
                GameMod::RelaxTaiko(_) => arm!(Taiko, RelaxTaiko, Some(8), Relax),
                GameMod::WindUpTaiko(_) => arm!(Taiko, WindUpTaiko, None, WindUp),
                GameMod::WindDownTaiko(_) => arm!(Taiko, WindDownTaiko, None, WindDown),
                GameMod::MutedTaiko(_) => arm!(Taiko, MutedTaiko, None, Muted),
                GameMod::AdaptiveSpeedTaiko(_) => {
                    arm!(Taiko, AdaptiveSpeedTaiko, None, AdaptiveSpeed)
                }
                GameMod::ScoreV2Taiko(_) => arm!(Taiko, ScoreV2Taiko, Some(30), ScoreV2),
                GameMod::UnknownTaiko(m) => GameModOrder {
                    mode: GameMode::Taiko,
                    index: None,
                    intermode: GameModIntermode::Unknown(*m),
                },
                GameMod::EasyCatch(_) => arm!(Catch, EasyCatch, Some(2), Easy),
                GameMod::NoFailCatch(_) => arm!(Catch, NoFailCatch, Some(1), NoFail),
                GameMod::HalfTimeCatch(_) => arm!(Catch, HalfTimeCatch, Some(9), HalfTime),
                GameMod::DaycoreCatch(_) => arm!(Catch, DaycoreCatch, None, Daycore),
                GameMod::HardRockCatch(_) => arm!(Catch, HardRockCatch, Some(5), HardRock),
                GameMod::SuddenDeathCatch(_) => arm!(Catch, SuddenDeathCatch, Some(6), SuddenDeath),
                GameMod::PerfectCatch(_) => arm!(Catch, PerfectCatch, Some(15), Perfect),
                GameMod::DoubleTimeCatch(_) => arm!(Catch, DoubleTimeCatch, Some(7), DoubleTime),
                GameMod::NightcoreCatch(_) => arm!(Catch, NightcoreCatch, Some(10), Nightcore),
                GameMod::HiddenCatch(_) => arm!(Catch, HiddenCatch, Some(4), Hidden),
                GameMod::FlashlightCatch(_) => arm!(Catch, FlashlightCatch, Some(11), Flashlight),
                GameMod::AccuracyChallengeCatch(_) => {
                    arm!(Catch, AccuracyChallengeCatch, None, AccuracyChallenge)
                }
                GameMod::DifficultyAdjustCatch(_) => {
                    arm!(Catch, DifficultyAdjustCatch, None, DifficultyAdjust)
                }
                GameMod::ClassicCatch(_) => arm!(Catch, ClassicCatch, None, Classic),
                GameMod::MirrorCatch(_) => arm!(Catch, MirrorCatch, Some(31), Mirror),
                GameMod::AutoplayCatch(_) => arm!(Catch, AutoplayCatch, Some(12), Autoplay),
                GameMod::CinemaCatch(_) => arm!(Catch, CinemaCatch, Some(23), Cinema),
                GameMod::RelaxCatch(_) => arm!(Catch, RelaxCatch, Some(8), Relax),
                GameMod::WindUpCatch(_) => arm!(Catch, WindUpCatch, None, WindUp),
                GameMod::WindDownCatch(_) => arm!(Catch, WindDownCatch, None, WindDown),
                GameMod::FloatingFruitsCatch(_) => {
                    arm!(Catch, FloatingFruitsCatch, None, FloatingFruits)
                }
                GameMod::MutedCatch(_) => arm!(Catch, MutedCatch, None, Muted),
                GameMod::NoScopeCatch(_) => arm!(Catch, NoScopeCatch, None, NoScope),
                GameMod::ScoreV2Catch(_) => arm!(Catch, ScoreV2Catch, Some(30), ScoreV2),
                GameMod::UnknownCatch(m) => GameModOrder {
                    mode: GameMode::Catch,
                    index: None,
                    intermode: GameModIntermode::Unknown(*m),
                },
                GameMod::EasyMania(_) => arm!(Mania, EasyMania, Some(2), Easy),
                GameMod::NoFailMania(_) => arm!(Mania, NoFailMania, Some(1), NoFail),
                GameMod::HalfTimeMania(_) => arm!(Mania, HalfTimeMania, Some(9), HalfTime),
                GameMod::DaycoreMania(_) => arm!(Mania, DaycoreMania, None, Daycore),
                GameMod::NoReleaseMania(_) => arm!(Mania, NoReleaseMania, None, NoRelease),
                GameMod::HardRockMania(_) => arm!(Mania, HardRockMania, Some(5), HardRock),
                GameMod::SuddenDeathMania(_) => arm!(Mania, SuddenDeathMania, Some(6), SuddenDeath),
                GameMod::PerfectMania(_) => arm!(Mania, PerfectMania, Some(15), Perfect),
                GameMod::DoubleTimeMania(_) => arm!(Mania, DoubleTimeMania, Some(7), DoubleTime),
                GameMod::NightcoreMania(_) => arm!(Mania, NightcoreMania, Some(10), Nightcore),
                GameMod::FadeInMania(_) => arm!(Mania, FadeInMania, Some(21), FadeIn),
                GameMod::HiddenMania(_) => arm!(Mania, HiddenMania, Some(4), Hidden),
                GameMod::CoverMania(_) => arm!(Mania, CoverMania, None, Cover),
                GameMod::FlashlightMania(_) => arm!(Mania, FlashlightMania, Some(11), Flashlight),
                GameMod::AccuracyChallengeMania(_) => {
                    arm!(Mania, AccuracyChallengeMania, None, AccuracyChallenge)
                }
                GameMod::RandomMania(_) => arm!(Mania, RandomMania, Some(22), Random),
                GameMod::DualStagesMania(_) => arm!(Mania, DualStagesMania, Some(26), DualStages),
                GameMod::MirrorMania(_) => arm!(Mania, MirrorMania, Some(31), Mirror),
                GameMod::DifficultyAdjustMania(_) => {
                    arm!(Mania, DifficultyAdjustMania, None, DifficultyAdjust)
                }
                GameMod::ClassicMania(_) => arm!(Mania, ClassicMania, None, Classic),
                GameMod::InvertMania(_) => arm!(Mania, InvertMania, None, Invert),
                GameMod::ConstantSpeedMania(_) => {
                    arm!(Mania, ConstantSpeedMania, None, ConstantSpeed)
                }
                GameMod::HoldOffMania(_) => arm!(Mania, HoldOffMania, None, HoldOff),
                GameMod::OneKeyMania(_) => arm!(Mania, OneKeyMania, Some(27), OneKey),
                GameMod::TwoKeysMania(_) => arm!(Mania, TwoKeysMania, Some(29), TwoKeys),
                GameMod::ThreeKeysMania(_) => arm!(Mania, ThreeKeysMania, Some(28), ThreeKeys),
                GameMod::FourKeysMania(_) => arm!(Mania, FourKeysMania, Some(16), FourKeys),
                GameMod::FiveKeysMania(_) => arm!(Mania, FiveKeysMania, Some(17), FiveKeys),
                GameMod::SixKeysMania(_) => arm!(Mania, SixKeysMania, Some(18), SixKeys),
                GameMod::SevenKeysMania(_) => arm!(Mania, SevenKeysMania, Some(19), SevenKeys),
                GameMod::EightKeysMania(_) => arm!(Mania, EightKeysMania, Some(20), EightKeys),
                GameMod::NineKeysMania(_) => arm!(Mania, NineKeysMania, Some(25), NineKeys),
                GameMod::TenKeysMania(_) => arm!(Mania, TenKeysMania, None, TenKeys),
                GameMod::AutoplayMania(_) => arm!(Mania, AutoplayMania, Some(12), Autoplay),
                GameMod::CinemaMania(_) => arm!(Mania, CinemaMania, Some(23), Cinema),
                GameMod::WindUpMania(_) => arm!(Mania, WindUpMania, None, WindUp),
                GameMod::WindDownMania(_) => arm!(Mania, WindDownMania, None, WindDown),
                GameMod::MutedMania(_) => arm!(Mania, MutedMania, None, Muted),
                GameMod::AdaptiveSpeedMania(_) => {
                    arm!(Mania, AdaptiveSpeedMania, None, AdaptiveSpeed)
                }
                GameMod::ScoreV2Mania(_) => arm!(Mania, ScoreV2Mania, Some(30), ScoreV2),
                GameMod::UnknownMania(m) => GameModOrder {
                    mode: GameMode::Mania,
                    index: None,
                    intermode: GameModIntermode::Unknown(*m),
                },
            }
        }
        inner(gamemod)
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
            .then_with(|| match (self.index, other.index) {
                (Some(self_idx), Some(other_idx)) => self_idx.cmp(&other_idx),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => self
                    .intermode
                    .acronym()
                    .as_str()
                    .cmp(other.intermode.acronym().as_str()),
            })
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
pub(crate) mod gamemod {
    use super::*;
    /// A single game mod
    #[derive(Clone, Debug, PartialEq)]
    #[cfg_attr(
        feature = "rkyv",
        derive(::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize)
    )]
    #[non_exhaustive]
    pub enum GameMod {
        EasyOsu(EasyOsu),
        NoFailOsu(NoFailOsu),
        HalfTimeOsu(HalfTimeOsu),
        DaycoreOsu(DaycoreOsu),
        HardRockOsu(HardRockOsu),
        SuddenDeathOsu(SuddenDeathOsu),
        PerfectOsu(PerfectOsu),
        DoubleTimeOsu(DoubleTimeOsu),
        NightcoreOsu(NightcoreOsu),
        HiddenOsu(HiddenOsu),
        FlashlightOsu(FlashlightOsu),
        BlindsOsu(BlindsOsu),
        StrictTrackingOsu(StrictTrackingOsu),
        AccuracyChallengeOsu(AccuracyChallengeOsu),
        TargetPracticeOsu(TargetPracticeOsu),
        DifficultyAdjustOsu(DifficultyAdjustOsu),
        ClassicOsu(ClassicOsu),
        RandomOsu(RandomOsu),
        MirrorOsu(MirrorOsu),
        AlternateOsu(AlternateOsu),
        SingleTapOsu(SingleTapOsu),
        AutoplayOsu(AutoplayOsu),
        CinemaOsu(CinemaOsu),
        RelaxOsu(RelaxOsu),
        AutopilotOsu(AutopilotOsu),
        SpunOutOsu(SpunOutOsu),
        TransformOsu(TransformOsu),
        WiggleOsu(WiggleOsu),
        SpinInOsu(SpinInOsu),
        GrowOsu(GrowOsu),
        DeflateOsu(DeflateOsu),
        WindUpOsu(WindUpOsu),
        WindDownOsu(WindDownOsu),
        TraceableOsu(TraceableOsu),
        BarrelRollOsu(BarrelRollOsu),
        ApproachDifferentOsu(ApproachDifferentOsu),
        MutedOsu(MutedOsu),
        NoScopeOsu(NoScopeOsu),
        MagnetisedOsu(MagnetisedOsu),
        RepelOsu(RepelOsu),
        AdaptiveSpeedOsu(AdaptiveSpeedOsu),
        FreezeFrameOsu(FreezeFrameOsu),
        BubblesOsu(BubblesOsu),
        SynesthesiaOsu(SynesthesiaOsu),
        DepthOsu(DepthOsu),
        BloomOsu(BloomOsu),
        TouchDeviceOsu(TouchDeviceOsu),
        ScoreV2Osu(ScoreV2Osu),
        UnknownOsu(UnknownMod),
        EasyTaiko(EasyTaiko),
        NoFailTaiko(NoFailTaiko),
        HalfTimeTaiko(HalfTimeTaiko),
        DaycoreTaiko(DaycoreTaiko),
        HardRockTaiko(HardRockTaiko),
        SuddenDeathTaiko(SuddenDeathTaiko),
        PerfectTaiko(PerfectTaiko),
        DoubleTimeTaiko(DoubleTimeTaiko),
        NightcoreTaiko(NightcoreTaiko),
        HiddenTaiko(HiddenTaiko),
        FlashlightTaiko(FlashlightTaiko),
        AccuracyChallengeTaiko(AccuracyChallengeTaiko),
        RandomTaiko(RandomTaiko),
        DifficultyAdjustTaiko(DifficultyAdjustTaiko),
        ClassicTaiko(ClassicTaiko),
        SwapTaiko(SwapTaiko),
        SingleTapTaiko(SingleTapTaiko),
        ConstantSpeedTaiko(ConstantSpeedTaiko),
        AutoplayTaiko(AutoplayTaiko),
        CinemaTaiko(CinemaTaiko),
        RelaxTaiko(RelaxTaiko),
        WindUpTaiko(WindUpTaiko),
        WindDownTaiko(WindDownTaiko),
        MutedTaiko(MutedTaiko),
        AdaptiveSpeedTaiko(AdaptiveSpeedTaiko),
        ScoreV2Taiko(ScoreV2Taiko),
        UnknownTaiko(UnknownMod),
        EasyCatch(EasyCatch),
        NoFailCatch(NoFailCatch),
        HalfTimeCatch(HalfTimeCatch),
        DaycoreCatch(DaycoreCatch),
        HardRockCatch(HardRockCatch),
        SuddenDeathCatch(SuddenDeathCatch),
        PerfectCatch(PerfectCatch),
        DoubleTimeCatch(DoubleTimeCatch),
        NightcoreCatch(NightcoreCatch),
        HiddenCatch(HiddenCatch),
        FlashlightCatch(FlashlightCatch),
        AccuracyChallengeCatch(AccuracyChallengeCatch),
        DifficultyAdjustCatch(DifficultyAdjustCatch),
        ClassicCatch(ClassicCatch),
        MirrorCatch(MirrorCatch),
        AutoplayCatch(AutoplayCatch),
        CinemaCatch(CinemaCatch),
        RelaxCatch(RelaxCatch),
        WindUpCatch(WindUpCatch),
        WindDownCatch(WindDownCatch),
        FloatingFruitsCatch(FloatingFruitsCatch),
        MutedCatch(MutedCatch),
        NoScopeCatch(NoScopeCatch),
        ScoreV2Catch(ScoreV2Catch),
        UnknownCatch(UnknownMod),
        EasyMania(EasyMania),
        NoFailMania(NoFailMania),
        HalfTimeMania(HalfTimeMania),
        DaycoreMania(DaycoreMania),
        NoReleaseMania(NoReleaseMania),
        HardRockMania(HardRockMania),
        SuddenDeathMania(SuddenDeathMania),
        PerfectMania(PerfectMania),
        DoubleTimeMania(DoubleTimeMania),
        NightcoreMania(NightcoreMania),
        FadeInMania(FadeInMania),
        HiddenMania(HiddenMania),
        CoverMania(CoverMania),
        FlashlightMania(FlashlightMania),
        AccuracyChallengeMania(AccuracyChallengeMania),
        RandomMania(RandomMania),
        DualStagesMania(DualStagesMania),
        MirrorMania(MirrorMania),
        DifficultyAdjustMania(DifficultyAdjustMania),
        ClassicMania(ClassicMania),
        InvertMania(InvertMania),
        ConstantSpeedMania(ConstantSpeedMania),
        HoldOffMania(HoldOffMania),
        OneKeyMania(OneKeyMania),
        TwoKeysMania(TwoKeysMania),
        ThreeKeysMania(ThreeKeysMania),
        FourKeysMania(FourKeysMania),
        FiveKeysMania(FiveKeysMania),
        SixKeysMania(SixKeysMania),
        SevenKeysMania(SevenKeysMania),
        EightKeysMania(EightKeysMania),
        NineKeysMania(NineKeysMania),
        TenKeysMania(TenKeysMania),
        AutoplayMania(AutoplayMania),
        CinemaMania(CinemaMania),
        WindUpMania(WindUpMania),
        WindDownMania(WindDownMania),
        MutedMania(MutedMania),
        AdaptiveSpeedMania(AdaptiveSpeedMania),
        ScoreV2Mania(ScoreV2Mania),
        UnknownMania(UnknownMod),
    }
}
impl GameMod {
    /// Create a new [`GameMod`]
    pub fn new(acronym: &str, mode: GameMode) -> Self {
        match (acronym, mode) {
            ("EZ", GameMode::Osu) => Self::EasyOsu(Default::default()),
            ("NF", GameMode::Osu) => Self::NoFailOsu(Default::default()),
            ("HT", GameMode::Osu) => Self::HalfTimeOsu(Default::default()),
            ("DC", GameMode::Osu) => Self::DaycoreOsu(Default::default()),
            ("HR", GameMode::Osu) => Self::HardRockOsu(Default::default()),
            ("SD", GameMode::Osu) => Self::SuddenDeathOsu(Default::default()),
            ("PF", GameMode::Osu) => Self::PerfectOsu(Default::default()),
            ("DT", GameMode::Osu) => Self::DoubleTimeOsu(Default::default()),
            ("NC", GameMode::Osu) => Self::NightcoreOsu(Default::default()),
            ("HD", GameMode::Osu) => Self::HiddenOsu(Default::default()),
            ("FL", GameMode::Osu) => Self::FlashlightOsu(Default::default()),
            ("BL", GameMode::Osu) => Self::BlindsOsu(Default::default()),
            ("ST", GameMode::Osu) => Self::StrictTrackingOsu(Default::default()),
            ("AC", GameMode::Osu) => Self::AccuracyChallengeOsu(Default::default()),
            ("TP", GameMode::Osu) => Self::TargetPracticeOsu(Default::default()),
            ("DA", GameMode::Osu) => Self::DifficultyAdjustOsu(Default::default()),
            ("CL", GameMode::Osu) => Self::ClassicOsu(Default::default()),
            ("RD", GameMode::Osu) => Self::RandomOsu(Default::default()),
            ("MR", GameMode::Osu) => Self::MirrorOsu(Default::default()),
            ("AL", GameMode::Osu) => Self::AlternateOsu(Default::default()),
            ("SG", GameMode::Osu) => Self::SingleTapOsu(Default::default()),
            ("AT", GameMode::Osu) => Self::AutoplayOsu(Default::default()),
            ("CN", GameMode::Osu) => Self::CinemaOsu(Default::default()),
            ("RX", GameMode::Osu) => Self::RelaxOsu(Default::default()),
            ("AP", GameMode::Osu) => Self::AutopilotOsu(Default::default()),
            ("SO", GameMode::Osu) => Self::SpunOutOsu(Default::default()),
            ("TR", GameMode::Osu) => Self::TransformOsu(Default::default()),
            ("WG", GameMode::Osu) => Self::WiggleOsu(Default::default()),
            ("SI", GameMode::Osu) => Self::SpinInOsu(Default::default()),
            ("GR", GameMode::Osu) => Self::GrowOsu(Default::default()),
            ("DF", GameMode::Osu) => Self::DeflateOsu(Default::default()),
            ("WU", GameMode::Osu) => Self::WindUpOsu(Default::default()),
            ("WD", GameMode::Osu) => Self::WindDownOsu(Default::default()),
            ("TC", GameMode::Osu) => Self::TraceableOsu(Default::default()),
            ("BR", GameMode::Osu) => Self::BarrelRollOsu(Default::default()),
            ("AD", GameMode::Osu) => Self::ApproachDifferentOsu(Default::default()),
            ("MU", GameMode::Osu) => Self::MutedOsu(Default::default()),
            ("NS", GameMode::Osu) => Self::NoScopeOsu(Default::default()),
            ("MG", GameMode::Osu) => Self::MagnetisedOsu(Default::default()),
            ("RP", GameMode::Osu) => Self::RepelOsu(Default::default()),
            ("AS", GameMode::Osu) => Self::AdaptiveSpeedOsu(Default::default()),
            ("FR", GameMode::Osu) => Self::FreezeFrameOsu(Default::default()),
            ("BU", GameMode::Osu) => Self::BubblesOsu(Default::default()),
            ("SY", GameMode::Osu) => Self::SynesthesiaOsu(Default::default()),
            ("DP", GameMode::Osu) => Self::DepthOsu(Default::default()),
            ("BM", GameMode::Osu) => Self::BloomOsu(Default::default()),
            ("TD", GameMode::Osu) => Self::TouchDeviceOsu(Default::default()),
            ("SV2", GameMode::Osu) => Self::ScoreV2Osu(Default::default()),
            ("EZ", GameMode::Taiko) => Self::EasyTaiko(Default::default()),
            ("NF", GameMode::Taiko) => Self::NoFailTaiko(Default::default()),
            ("HT", GameMode::Taiko) => Self::HalfTimeTaiko(Default::default()),
            ("DC", GameMode::Taiko) => Self::DaycoreTaiko(Default::default()),
            ("HR", GameMode::Taiko) => Self::HardRockTaiko(Default::default()),
            ("SD", GameMode::Taiko) => Self::SuddenDeathTaiko(Default::default()),
            ("PF", GameMode::Taiko) => Self::PerfectTaiko(Default::default()),
            ("DT", GameMode::Taiko) => Self::DoubleTimeTaiko(Default::default()),
            ("NC", GameMode::Taiko) => Self::NightcoreTaiko(Default::default()),
            ("HD", GameMode::Taiko) => Self::HiddenTaiko(Default::default()),
            ("FL", GameMode::Taiko) => Self::FlashlightTaiko(Default::default()),
            ("AC", GameMode::Taiko) => Self::AccuracyChallengeTaiko(Default::default()),
            ("RD", GameMode::Taiko) => Self::RandomTaiko(Default::default()),
            ("DA", GameMode::Taiko) => Self::DifficultyAdjustTaiko(Default::default()),
            ("CL", GameMode::Taiko) => Self::ClassicTaiko(Default::default()),
            ("SW", GameMode::Taiko) => Self::SwapTaiko(Default::default()),
            ("SG", GameMode::Taiko) => Self::SingleTapTaiko(Default::default()),
            ("CS", GameMode::Taiko) => Self::ConstantSpeedTaiko(Default::default()),
            ("AT", GameMode::Taiko) => Self::AutoplayTaiko(Default::default()),
            ("CN", GameMode::Taiko) => Self::CinemaTaiko(Default::default()),
            ("RX", GameMode::Taiko) => Self::RelaxTaiko(Default::default()),
            ("WU", GameMode::Taiko) => Self::WindUpTaiko(Default::default()),
            ("WD", GameMode::Taiko) => Self::WindDownTaiko(Default::default()),
            ("MU", GameMode::Taiko) => Self::MutedTaiko(Default::default()),
            ("AS", GameMode::Taiko) => Self::AdaptiveSpeedTaiko(Default::default()),
            ("SV2", GameMode::Taiko) => Self::ScoreV2Taiko(Default::default()),
            ("EZ", GameMode::Catch) => Self::EasyCatch(Default::default()),
            ("NF", GameMode::Catch) => Self::NoFailCatch(Default::default()),
            ("HT", GameMode::Catch) => Self::HalfTimeCatch(Default::default()),
            ("DC", GameMode::Catch) => Self::DaycoreCatch(Default::default()),
            ("HR", GameMode::Catch) => Self::HardRockCatch(Default::default()),
            ("SD", GameMode::Catch) => Self::SuddenDeathCatch(Default::default()),
            ("PF", GameMode::Catch) => Self::PerfectCatch(Default::default()),
            ("DT", GameMode::Catch) => Self::DoubleTimeCatch(Default::default()),
            ("NC", GameMode::Catch) => Self::NightcoreCatch(Default::default()),
            ("HD", GameMode::Catch) => Self::HiddenCatch(Default::default()),
            ("FL", GameMode::Catch) => Self::FlashlightCatch(Default::default()),
            ("AC", GameMode::Catch) => Self::AccuracyChallengeCatch(Default::default()),
            ("DA", GameMode::Catch) => Self::DifficultyAdjustCatch(Default::default()),
            ("CL", GameMode::Catch) => Self::ClassicCatch(Default::default()),
            ("MR", GameMode::Catch) => Self::MirrorCatch(Default::default()),
            ("AT", GameMode::Catch) => Self::AutoplayCatch(Default::default()),
            ("CN", GameMode::Catch) => Self::CinemaCatch(Default::default()),
            ("RX", GameMode::Catch) => Self::RelaxCatch(Default::default()),
            ("WU", GameMode::Catch) => Self::WindUpCatch(Default::default()),
            ("WD", GameMode::Catch) => Self::WindDownCatch(Default::default()),
            ("FF", GameMode::Catch) => Self::FloatingFruitsCatch(Default::default()),
            ("MU", GameMode::Catch) => Self::MutedCatch(Default::default()),
            ("NS", GameMode::Catch) => Self::NoScopeCatch(Default::default()),
            ("SV2", GameMode::Catch) => Self::ScoreV2Catch(Default::default()),
            ("EZ", GameMode::Mania) => Self::EasyMania(Default::default()),
            ("NF", GameMode::Mania) => Self::NoFailMania(Default::default()),
            ("HT", GameMode::Mania) => Self::HalfTimeMania(Default::default()),
            ("DC", GameMode::Mania) => Self::DaycoreMania(Default::default()),
            ("NR", GameMode::Mania) => Self::NoReleaseMania(Default::default()),
            ("HR", GameMode::Mania) => Self::HardRockMania(Default::default()),
            ("SD", GameMode::Mania) => Self::SuddenDeathMania(Default::default()),
            ("PF", GameMode::Mania) => Self::PerfectMania(Default::default()),
            ("DT", GameMode::Mania) => Self::DoubleTimeMania(Default::default()),
            ("NC", GameMode::Mania) => Self::NightcoreMania(Default::default()),
            ("FI", GameMode::Mania) => Self::FadeInMania(Default::default()),
            ("HD", GameMode::Mania) => Self::HiddenMania(Default::default()),
            ("CO", GameMode::Mania) => Self::CoverMania(Default::default()),
            ("FL", GameMode::Mania) => Self::FlashlightMania(Default::default()),
            ("AC", GameMode::Mania) => Self::AccuracyChallengeMania(Default::default()),
            ("RD", GameMode::Mania) => Self::RandomMania(Default::default()),
            ("DS", GameMode::Mania) => Self::DualStagesMania(Default::default()),
            ("MR", GameMode::Mania) => Self::MirrorMania(Default::default()),
            ("DA", GameMode::Mania) => Self::DifficultyAdjustMania(Default::default()),
            ("CL", GameMode::Mania) => Self::ClassicMania(Default::default()),
            ("IN", GameMode::Mania) => Self::InvertMania(Default::default()),
            ("CS", GameMode::Mania) => Self::ConstantSpeedMania(Default::default()),
            ("HO", GameMode::Mania) => Self::HoldOffMania(Default::default()),
            ("1K", GameMode::Mania) => Self::OneKeyMania(Default::default()),
            ("2K", GameMode::Mania) => Self::TwoKeysMania(Default::default()),
            ("3K", GameMode::Mania) => Self::ThreeKeysMania(Default::default()),
            ("4K", GameMode::Mania) => Self::FourKeysMania(Default::default()),
            ("5K", GameMode::Mania) => Self::FiveKeysMania(Default::default()),
            ("6K", GameMode::Mania) => Self::SixKeysMania(Default::default()),
            ("7K", GameMode::Mania) => Self::SevenKeysMania(Default::default()),
            ("8K", GameMode::Mania) => Self::EightKeysMania(Default::default()),
            ("9K", GameMode::Mania) => Self::NineKeysMania(Default::default()),
            ("10K", GameMode::Mania) => Self::TenKeysMania(Default::default()),
            ("AT", GameMode::Mania) => Self::AutoplayMania(Default::default()),
            ("CN", GameMode::Mania) => Self::CinemaMania(Default::default()),
            ("WU", GameMode::Mania) => Self::WindUpMania(Default::default()),
            ("WD", GameMode::Mania) => Self::WindDownMania(Default::default()),
            ("MU", GameMode::Mania) => Self::MutedMania(Default::default()),
            ("AS", GameMode::Mania) => Self::AdaptiveSpeedMania(Default::default()),
            ("SV2", GameMode::Mania) => Self::ScoreV2Mania(Default::default()),
            _ => {
                let acronym = <Acronym as std::str::FromStr>::from_str(acronym)
                    .unwrap_or(UnknownMod::UNKNOWN_ACRONYM);
                let unknown = UnknownMod { acronym };
                match mode {
                    GameMode::Osu => GameMod::UnknownOsu(unknown),
                    GameMode::Taiko => GameMod::UnknownTaiko(unknown),
                    GameMode::Catch => GameMod::UnknownCatch(unknown),
                    GameMode::Mania => GameMod::UnknownMania(unknown),
                }
            }
        }
    }
    /// The acronym of this [`GameMod`]
    pub const fn acronym(&self) -> Acronym {
        match self {
            Self::EasyOsu(_) => EasyOsu::acronym(),
            Self::NoFailOsu(_) => NoFailOsu::acronym(),
            Self::HalfTimeOsu(_) => HalfTimeOsu::acronym(),
            Self::DaycoreOsu(_) => DaycoreOsu::acronym(),
            Self::HardRockOsu(_) => HardRockOsu::acronym(),
            Self::SuddenDeathOsu(_) => SuddenDeathOsu::acronym(),
            Self::PerfectOsu(_) => PerfectOsu::acronym(),
            Self::DoubleTimeOsu(_) => DoubleTimeOsu::acronym(),
            Self::NightcoreOsu(_) => NightcoreOsu::acronym(),
            Self::HiddenOsu(_) => HiddenOsu::acronym(),
            Self::FlashlightOsu(_) => FlashlightOsu::acronym(),
            Self::BlindsOsu(_) => BlindsOsu::acronym(),
            Self::StrictTrackingOsu(_) => StrictTrackingOsu::acronym(),
            Self::AccuracyChallengeOsu(_) => AccuracyChallengeOsu::acronym(),
            Self::TargetPracticeOsu(_) => TargetPracticeOsu::acronym(),
            Self::DifficultyAdjustOsu(_) => DifficultyAdjustOsu::acronym(),
            Self::ClassicOsu(_) => ClassicOsu::acronym(),
            Self::RandomOsu(_) => RandomOsu::acronym(),
            Self::MirrorOsu(_) => MirrorOsu::acronym(),
            Self::AlternateOsu(_) => AlternateOsu::acronym(),
            Self::SingleTapOsu(_) => SingleTapOsu::acronym(),
            Self::AutoplayOsu(_) => AutoplayOsu::acronym(),
            Self::CinemaOsu(_) => CinemaOsu::acronym(),
            Self::RelaxOsu(_) => RelaxOsu::acronym(),
            Self::AutopilotOsu(_) => AutopilotOsu::acronym(),
            Self::SpunOutOsu(_) => SpunOutOsu::acronym(),
            Self::TransformOsu(_) => TransformOsu::acronym(),
            Self::WiggleOsu(_) => WiggleOsu::acronym(),
            Self::SpinInOsu(_) => SpinInOsu::acronym(),
            Self::GrowOsu(_) => GrowOsu::acronym(),
            Self::DeflateOsu(_) => DeflateOsu::acronym(),
            Self::WindUpOsu(_) => WindUpOsu::acronym(),
            Self::WindDownOsu(_) => WindDownOsu::acronym(),
            Self::TraceableOsu(_) => TraceableOsu::acronym(),
            Self::BarrelRollOsu(_) => BarrelRollOsu::acronym(),
            Self::ApproachDifferentOsu(_) => ApproachDifferentOsu::acronym(),
            Self::MutedOsu(_) => MutedOsu::acronym(),
            Self::NoScopeOsu(_) => NoScopeOsu::acronym(),
            Self::MagnetisedOsu(_) => MagnetisedOsu::acronym(),
            Self::RepelOsu(_) => RepelOsu::acronym(),
            Self::AdaptiveSpeedOsu(_) => AdaptiveSpeedOsu::acronym(),
            Self::FreezeFrameOsu(_) => FreezeFrameOsu::acronym(),
            Self::BubblesOsu(_) => BubblesOsu::acronym(),
            Self::SynesthesiaOsu(_) => SynesthesiaOsu::acronym(),
            Self::DepthOsu(_) => DepthOsu::acronym(),
            Self::BloomOsu(_) => BloomOsu::acronym(),
            Self::TouchDeviceOsu(_) => TouchDeviceOsu::acronym(),
            Self::ScoreV2Osu(_) => ScoreV2Osu::acronym(),
            Self::EasyTaiko(_) => EasyTaiko::acronym(),
            Self::NoFailTaiko(_) => NoFailTaiko::acronym(),
            Self::HalfTimeTaiko(_) => HalfTimeTaiko::acronym(),
            Self::DaycoreTaiko(_) => DaycoreTaiko::acronym(),
            Self::HardRockTaiko(_) => HardRockTaiko::acronym(),
            Self::SuddenDeathTaiko(_) => SuddenDeathTaiko::acronym(),
            Self::PerfectTaiko(_) => PerfectTaiko::acronym(),
            Self::DoubleTimeTaiko(_) => DoubleTimeTaiko::acronym(),
            Self::NightcoreTaiko(_) => NightcoreTaiko::acronym(),
            Self::HiddenTaiko(_) => HiddenTaiko::acronym(),
            Self::FlashlightTaiko(_) => FlashlightTaiko::acronym(),
            Self::AccuracyChallengeTaiko(_) => AccuracyChallengeTaiko::acronym(),
            Self::RandomTaiko(_) => RandomTaiko::acronym(),
            Self::DifficultyAdjustTaiko(_) => DifficultyAdjustTaiko::acronym(),
            Self::ClassicTaiko(_) => ClassicTaiko::acronym(),
            Self::SwapTaiko(_) => SwapTaiko::acronym(),
            Self::SingleTapTaiko(_) => SingleTapTaiko::acronym(),
            Self::ConstantSpeedTaiko(_) => ConstantSpeedTaiko::acronym(),
            Self::AutoplayTaiko(_) => AutoplayTaiko::acronym(),
            Self::CinemaTaiko(_) => CinemaTaiko::acronym(),
            Self::RelaxTaiko(_) => RelaxTaiko::acronym(),
            Self::WindUpTaiko(_) => WindUpTaiko::acronym(),
            Self::WindDownTaiko(_) => WindDownTaiko::acronym(),
            Self::MutedTaiko(_) => MutedTaiko::acronym(),
            Self::AdaptiveSpeedTaiko(_) => AdaptiveSpeedTaiko::acronym(),
            Self::ScoreV2Taiko(_) => ScoreV2Taiko::acronym(),
            Self::EasyCatch(_) => EasyCatch::acronym(),
            Self::NoFailCatch(_) => NoFailCatch::acronym(),
            Self::HalfTimeCatch(_) => HalfTimeCatch::acronym(),
            Self::DaycoreCatch(_) => DaycoreCatch::acronym(),
            Self::HardRockCatch(_) => HardRockCatch::acronym(),
            Self::SuddenDeathCatch(_) => SuddenDeathCatch::acronym(),
            Self::PerfectCatch(_) => PerfectCatch::acronym(),
            Self::DoubleTimeCatch(_) => DoubleTimeCatch::acronym(),
            Self::NightcoreCatch(_) => NightcoreCatch::acronym(),
            Self::HiddenCatch(_) => HiddenCatch::acronym(),
            Self::FlashlightCatch(_) => FlashlightCatch::acronym(),
            Self::AccuracyChallengeCatch(_) => AccuracyChallengeCatch::acronym(),
            Self::DifficultyAdjustCatch(_) => DifficultyAdjustCatch::acronym(),
            Self::ClassicCatch(_) => ClassicCatch::acronym(),
            Self::MirrorCatch(_) => MirrorCatch::acronym(),
            Self::AutoplayCatch(_) => AutoplayCatch::acronym(),
            Self::CinemaCatch(_) => CinemaCatch::acronym(),
            Self::RelaxCatch(_) => RelaxCatch::acronym(),
            Self::WindUpCatch(_) => WindUpCatch::acronym(),
            Self::WindDownCatch(_) => WindDownCatch::acronym(),
            Self::FloatingFruitsCatch(_) => FloatingFruitsCatch::acronym(),
            Self::MutedCatch(_) => MutedCatch::acronym(),
            Self::NoScopeCatch(_) => NoScopeCatch::acronym(),
            Self::ScoreV2Catch(_) => ScoreV2Catch::acronym(),
            Self::EasyMania(_) => EasyMania::acronym(),
            Self::NoFailMania(_) => NoFailMania::acronym(),
            Self::HalfTimeMania(_) => HalfTimeMania::acronym(),
            Self::DaycoreMania(_) => DaycoreMania::acronym(),
            Self::NoReleaseMania(_) => NoReleaseMania::acronym(),
            Self::HardRockMania(_) => HardRockMania::acronym(),
            Self::SuddenDeathMania(_) => SuddenDeathMania::acronym(),
            Self::PerfectMania(_) => PerfectMania::acronym(),
            Self::DoubleTimeMania(_) => DoubleTimeMania::acronym(),
            Self::NightcoreMania(_) => NightcoreMania::acronym(),
            Self::FadeInMania(_) => FadeInMania::acronym(),
            Self::HiddenMania(_) => HiddenMania::acronym(),
            Self::CoverMania(_) => CoverMania::acronym(),
            Self::FlashlightMania(_) => FlashlightMania::acronym(),
            Self::AccuracyChallengeMania(_) => AccuracyChallengeMania::acronym(),
            Self::RandomMania(_) => RandomMania::acronym(),
            Self::DualStagesMania(_) => DualStagesMania::acronym(),
            Self::MirrorMania(_) => MirrorMania::acronym(),
            Self::DifficultyAdjustMania(_) => DifficultyAdjustMania::acronym(),
            Self::ClassicMania(_) => ClassicMania::acronym(),
            Self::InvertMania(_) => InvertMania::acronym(),
            Self::ConstantSpeedMania(_) => ConstantSpeedMania::acronym(),
            Self::HoldOffMania(_) => HoldOffMania::acronym(),
            Self::OneKeyMania(_) => OneKeyMania::acronym(),
            Self::TwoKeysMania(_) => TwoKeysMania::acronym(),
            Self::ThreeKeysMania(_) => ThreeKeysMania::acronym(),
            Self::FourKeysMania(_) => FourKeysMania::acronym(),
            Self::FiveKeysMania(_) => FiveKeysMania::acronym(),
            Self::SixKeysMania(_) => SixKeysMania::acronym(),
            Self::SevenKeysMania(_) => SevenKeysMania::acronym(),
            Self::EightKeysMania(_) => EightKeysMania::acronym(),
            Self::NineKeysMania(_) => NineKeysMania::acronym(),
            Self::TenKeysMania(_) => TenKeysMania::acronym(),
            Self::AutoplayMania(_) => AutoplayMania::acronym(),
            Self::CinemaMania(_) => CinemaMania::acronym(),
            Self::WindUpMania(_) => WindUpMania::acronym(),
            Self::WindDownMania(_) => WindDownMania::acronym(),
            Self::MutedMania(_) => MutedMania::acronym(),
            Self::AdaptiveSpeedMania(_) => AdaptiveSpeedMania::acronym(),
            Self::ScoreV2Mania(_) => ScoreV2Mania::acronym(),
            Self::UnknownOsu(m)
            | Self::UnknownTaiko(m)
            | Self::UnknownCatch(m)
            | Self::UnknownMania(m) => m.acronym(),
        }
    }
    /// List of [`Acronym`] for mods that are incompatible with this [`GameMod`]
    pub fn incompatible_mods(&self) -> Box<[Acronym]> {
        match self {
            Self::EasyOsu(_) => EasyOsu::incompatible_mods().collect(),
            Self::NoFailOsu(_) => NoFailOsu::incompatible_mods().collect(),
            Self::HalfTimeOsu(_) => HalfTimeOsu::incompatible_mods().collect(),
            Self::DaycoreOsu(_) => DaycoreOsu::incompatible_mods().collect(),
            Self::HardRockOsu(_) => HardRockOsu::incompatible_mods().collect(),
            Self::SuddenDeathOsu(_) => SuddenDeathOsu::incompatible_mods().collect(),
            Self::PerfectOsu(_) => PerfectOsu::incompatible_mods().collect(),
            Self::DoubleTimeOsu(_) => DoubleTimeOsu::incompatible_mods().collect(),
            Self::NightcoreOsu(_) => NightcoreOsu::incompatible_mods().collect(),
            Self::HiddenOsu(_) => HiddenOsu::incompatible_mods().collect(),
            Self::FlashlightOsu(_) => FlashlightOsu::incompatible_mods().collect(),
            Self::BlindsOsu(_) => BlindsOsu::incompatible_mods().collect(),
            Self::StrictTrackingOsu(_) => StrictTrackingOsu::incompatible_mods().collect(),
            Self::AccuracyChallengeOsu(_) => AccuracyChallengeOsu::incompatible_mods().collect(),
            Self::TargetPracticeOsu(_) => TargetPracticeOsu::incompatible_mods().collect(),
            Self::DifficultyAdjustOsu(_) => DifficultyAdjustOsu::incompatible_mods().collect(),
            Self::ClassicOsu(_) => ClassicOsu::incompatible_mods().collect(),
            Self::RandomOsu(_) => RandomOsu::incompatible_mods().collect(),
            Self::MirrorOsu(_) => MirrorOsu::incompatible_mods().collect(),
            Self::AlternateOsu(_) => AlternateOsu::incompatible_mods().collect(),
            Self::SingleTapOsu(_) => SingleTapOsu::incompatible_mods().collect(),
            Self::AutoplayOsu(_) => AutoplayOsu::incompatible_mods().collect(),
            Self::CinemaOsu(_) => CinemaOsu::incompatible_mods().collect(),
            Self::RelaxOsu(_) => RelaxOsu::incompatible_mods().collect(),
            Self::AutopilotOsu(_) => AutopilotOsu::incompatible_mods().collect(),
            Self::SpunOutOsu(_) => SpunOutOsu::incompatible_mods().collect(),
            Self::TransformOsu(_) => TransformOsu::incompatible_mods().collect(),
            Self::WiggleOsu(_) => WiggleOsu::incompatible_mods().collect(),
            Self::SpinInOsu(_) => SpinInOsu::incompatible_mods().collect(),
            Self::GrowOsu(_) => GrowOsu::incompatible_mods().collect(),
            Self::DeflateOsu(_) => DeflateOsu::incompatible_mods().collect(),
            Self::WindUpOsu(_) => WindUpOsu::incompatible_mods().collect(),
            Self::WindDownOsu(_) => WindDownOsu::incompatible_mods().collect(),
            Self::TraceableOsu(_) => TraceableOsu::incompatible_mods().collect(),
            Self::BarrelRollOsu(_) => BarrelRollOsu::incompatible_mods().collect(),
            Self::ApproachDifferentOsu(_) => ApproachDifferentOsu::incompatible_mods().collect(),
            Self::MutedOsu(_) => MutedOsu::incompatible_mods().collect(),
            Self::NoScopeOsu(_) => NoScopeOsu::incompatible_mods().collect(),
            Self::MagnetisedOsu(_) => MagnetisedOsu::incompatible_mods().collect(),
            Self::RepelOsu(_) => RepelOsu::incompatible_mods().collect(),
            Self::AdaptiveSpeedOsu(_) => AdaptiveSpeedOsu::incompatible_mods().collect(),
            Self::FreezeFrameOsu(_) => FreezeFrameOsu::incompatible_mods().collect(),
            Self::BubblesOsu(_) => BubblesOsu::incompatible_mods().collect(),
            Self::SynesthesiaOsu(_) => SynesthesiaOsu::incompatible_mods().collect(),
            Self::DepthOsu(_) => DepthOsu::incompatible_mods().collect(),
            Self::BloomOsu(_) => BloomOsu::incompatible_mods().collect(),
            Self::TouchDeviceOsu(_) => TouchDeviceOsu::incompatible_mods().collect(),
            Self::ScoreV2Osu(_) => ScoreV2Osu::incompatible_mods().collect(),
            Self::EasyTaiko(_) => EasyTaiko::incompatible_mods().collect(),
            Self::NoFailTaiko(_) => NoFailTaiko::incompatible_mods().collect(),
            Self::HalfTimeTaiko(_) => HalfTimeTaiko::incompatible_mods().collect(),
            Self::DaycoreTaiko(_) => DaycoreTaiko::incompatible_mods().collect(),
            Self::HardRockTaiko(_) => HardRockTaiko::incompatible_mods().collect(),
            Self::SuddenDeathTaiko(_) => SuddenDeathTaiko::incompatible_mods().collect(),
            Self::PerfectTaiko(_) => PerfectTaiko::incompatible_mods().collect(),
            Self::DoubleTimeTaiko(_) => DoubleTimeTaiko::incompatible_mods().collect(),
            Self::NightcoreTaiko(_) => NightcoreTaiko::incompatible_mods().collect(),
            Self::HiddenTaiko(_) => HiddenTaiko::incompatible_mods().collect(),
            Self::FlashlightTaiko(_) => FlashlightTaiko::incompatible_mods().collect(),
            Self::AccuracyChallengeTaiko(_) => {
                AccuracyChallengeTaiko::incompatible_mods().collect()
            }
            Self::RandomTaiko(_) => RandomTaiko::incompatible_mods().collect(),
            Self::DifficultyAdjustTaiko(_) => DifficultyAdjustTaiko::incompatible_mods().collect(),
            Self::ClassicTaiko(_) => ClassicTaiko::incompatible_mods().collect(),
            Self::SwapTaiko(_) => SwapTaiko::incompatible_mods().collect(),
            Self::SingleTapTaiko(_) => SingleTapTaiko::incompatible_mods().collect(),
            Self::ConstantSpeedTaiko(_) => ConstantSpeedTaiko::incompatible_mods().collect(),
            Self::AutoplayTaiko(_) => AutoplayTaiko::incompatible_mods().collect(),
            Self::CinemaTaiko(_) => CinemaTaiko::incompatible_mods().collect(),
            Self::RelaxTaiko(_) => RelaxTaiko::incompatible_mods().collect(),
            Self::WindUpTaiko(_) => WindUpTaiko::incompatible_mods().collect(),
            Self::WindDownTaiko(_) => WindDownTaiko::incompatible_mods().collect(),
            Self::MutedTaiko(_) => MutedTaiko::incompatible_mods().collect(),
            Self::AdaptiveSpeedTaiko(_) => AdaptiveSpeedTaiko::incompatible_mods().collect(),
            Self::ScoreV2Taiko(_) => ScoreV2Taiko::incompatible_mods().collect(),
            Self::EasyCatch(_) => EasyCatch::incompatible_mods().collect(),
            Self::NoFailCatch(_) => NoFailCatch::incompatible_mods().collect(),
            Self::HalfTimeCatch(_) => HalfTimeCatch::incompatible_mods().collect(),
            Self::DaycoreCatch(_) => DaycoreCatch::incompatible_mods().collect(),
            Self::HardRockCatch(_) => HardRockCatch::incompatible_mods().collect(),
            Self::SuddenDeathCatch(_) => SuddenDeathCatch::incompatible_mods().collect(),
            Self::PerfectCatch(_) => PerfectCatch::incompatible_mods().collect(),
            Self::DoubleTimeCatch(_) => DoubleTimeCatch::incompatible_mods().collect(),
            Self::NightcoreCatch(_) => NightcoreCatch::incompatible_mods().collect(),
            Self::HiddenCatch(_) => HiddenCatch::incompatible_mods().collect(),
            Self::FlashlightCatch(_) => FlashlightCatch::incompatible_mods().collect(),
            Self::AccuracyChallengeCatch(_) => {
                AccuracyChallengeCatch::incompatible_mods().collect()
            }
            Self::DifficultyAdjustCatch(_) => DifficultyAdjustCatch::incompatible_mods().collect(),
            Self::ClassicCatch(_) => ClassicCatch::incompatible_mods().collect(),
            Self::MirrorCatch(_) => MirrorCatch::incompatible_mods().collect(),
            Self::AutoplayCatch(_) => AutoplayCatch::incompatible_mods().collect(),
            Self::CinemaCatch(_) => CinemaCatch::incompatible_mods().collect(),
            Self::RelaxCatch(_) => RelaxCatch::incompatible_mods().collect(),
            Self::WindUpCatch(_) => WindUpCatch::incompatible_mods().collect(),
            Self::WindDownCatch(_) => WindDownCatch::incompatible_mods().collect(),
            Self::FloatingFruitsCatch(_) => FloatingFruitsCatch::incompatible_mods().collect(),
            Self::MutedCatch(_) => MutedCatch::incompatible_mods().collect(),
            Self::NoScopeCatch(_) => NoScopeCatch::incompatible_mods().collect(),
            Self::ScoreV2Catch(_) => ScoreV2Catch::incompatible_mods().collect(),
            Self::EasyMania(_) => EasyMania::incompatible_mods().collect(),
            Self::NoFailMania(_) => NoFailMania::incompatible_mods().collect(),
            Self::HalfTimeMania(_) => HalfTimeMania::incompatible_mods().collect(),
            Self::DaycoreMania(_) => DaycoreMania::incompatible_mods().collect(),
            Self::NoReleaseMania(_) => NoReleaseMania::incompatible_mods().collect(),
            Self::HardRockMania(_) => HardRockMania::incompatible_mods().collect(),
            Self::SuddenDeathMania(_) => SuddenDeathMania::incompatible_mods().collect(),
            Self::PerfectMania(_) => PerfectMania::incompatible_mods().collect(),
            Self::DoubleTimeMania(_) => DoubleTimeMania::incompatible_mods().collect(),
            Self::NightcoreMania(_) => NightcoreMania::incompatible_mods().collect(),
            Self::FadeInMania(_) => FadeInMania::incompatible_mods().collect(),
            Self::HiddenMania(_) => HiddenMania::incompatible_mods().collect(),
            Self::CoverMania(_) => CoverMania::incompatible_mods().collect(),
            Self::FlashlightMania(_) => FlashlightMania::incompatible_mods().collect(),
            Self::AccuracyChallengeMania(_) => {
                AccuracyChallengeMania::incompatible_mods().collect()
            }
            Self::RandomMania(_) => RandomMania::incompatible_mods().collect(),
            Self::DualStagesMania(_) => DualStagesMania::incompatible_mods().collect(),
            Self::MirrorMania(_) => MirrorMania::incompatible_mods().collect(),
            Self::DifficultyAdjustMania(_) => DifficultyAdjustMania::incompatible_mods().collect(),
            Self::ClassicMania(_) => ClassicMania::incompatible_mods().collect(),
            Self::InvertMania(_) => InvertMania::incompatible_mods().collect(),
            Self::ConstantSpeedMania(_) => ConstantSpeedMania::incompatible_mods().collect(),
            Self::HoldOffMania(_) => HoldOffMania::incompatible_mods().collect(),
            Self::OneKeyMania(_) => OneKeyMania::incompatible_mods().collect(),
            Self::TwoKeysMania(_) => TwoKeysMania::incompatible_mods().collect(),
            Self::ThreeKeysMania(_) => ThreeKeysMania::incompatible_mods().collect(),
            Self::FourKeysMania(_) => FourKeysMania::incompatible_mods().collect(),
            Self::FiveKeysMania(_) => FiveKeysMania::incompatible_mods().collect(),
            Self::SixKeysMania(_) => SixKeysMania::incompatible_mods().collect(),
            Self::SevenKeysMania(_) => SevenKeysMania::incompatible_mods().collect(),
            Self::EightKeysMania(_) => EightKeysMania::incompatible_mods().collect(),
            Self::NineKeysMania(_) => NineKeysMania::incompatible_mods().collect(),
            Self::TenKeysMania(_) => TenKeysMania::incompatible_mods().collect(),
            Self::AutoplayMania(_) => AutoplayMania::incompatible_mods().collect(),
            Self::CinemaMania(_) => CinemaMania::incompatible_mods().collect(),
            Self::WindUpMania(_) => WindUpMania::incompatible_mods().collect(),
            Self::WindDownMania(_) => WindDownMania::incompatible_mods().collect(),
            Self::MutedMania(_) => MutedMania::incompatible_mods().collect(),
            Self::AdaptiveSpeedMania(_) => AdaptiveSpeedMania::incompatible_mods().collect(),
            Self::ScoreV2Mania(_) => ScoreV2Mania::incompatible_mods().collect(),
            _ => UnknownMod::incompatible_mods().collect(),
        }
    }
    /// The description of this [`GameMod`]
    pub const fn description(&self) -> &'static str {
        match self {
            Self::EasyOsu(_) => EasyOsu::description(),
            Self::NoFailOsu(_) => NoFailOsu::description(),
            Self::HalfTimeOsu(_) => HalfTimeOsu::description(),
            Self::DaycoreOsu(_) => DaycoreOsu::description(),
            Self::HardRockOsu(_) => HardRockOsu::description(),
            Self::SuddenDeathOsu(_) => SuddenDeathOsu::description(),
            Self::PerfectOsu(_) => PerfectOsu::description(),
            Self::DoubleTimeOsu(_) => DoubleTimeOsu::description(),
            Self::NightcoreOsu(_) => NightcoreOsu::description(),
            Self::HiddenOsu(_) => HiddenOsu::description(),
            Self::FlashlightOsu(_) => FlashlightOsu::description(),
            Self::BlindsOsu(_) => BlindsOsu::description(),
            Self::StrictTrackingOsu(_) => StrictTrackingOsu::description(),
            Self::AccuracyChallengeOsu(_) => AccuracyChallengeOsu::description(),
            Self::TargetPracticeOsu(_) => TargetPracticeOsu::description(),
            Self::DifficultyAdjustOsu(_) => DifficultyAdjustOsu::description(),
            Self::ClassicOsu(_) => ClassicOsu::description(),
            Self::RandomOsu(_) => RandomOsu::description(),
            Self::MirrorOsu(_) => MirrorOsu::description(),
            Self::AlternateOsu(_) => AlternateOsu::description(),
            Self::SingleTapOsu(_) => SingleTapOsu::description(),
            Self::AutoplayOsu(_) => AutoplayOsu::description(),
            Self::CinemaOsu(_) => CinemaOsu::description(),
            Self::RelaxOsu(_) => RelaxOsu::description(),
            Self::AutopilotOsu(_) => AutopilotOsu::description(),
            Self::SpunOutOsu(_) => SpunOutOsu::description(),
            Self::TransformOsu(_) => TransformOsu::description(),
            Self::WiggleOsu(_) => WiggleOsu::description(),
            Self::SpinInOsu(_) => SpinInOsu::description(),
            Self::GrowOsu(_) => GrowOsu::description(),
            Self::DeflateOsu(_) => DeflateOsu::description(),
            Self::WindUpOsu(_) => WindUpOsu::description(),
            Self::WindDownOsu(_) => WindDownOsu::description(),
            Self::TraceableOsu(_) => TraceableOsu::description(),
            Self::BarrelRollOsu(_) => BarrelRollOsu::description(),
            Self::ApproachDifferentOsu(_) => ApproachDifferentOsu::description(),
            Self::MutedOsu(_) => MutedOsu::description(),
            Self::NoScopeOsu(_) => NoScopeOsu::description(),
            Self::MagnetisedOsu(_) => MagnetisedOsu::description(),
            Self::RepelOsu(_) => RepelOsu::description(),
            Self::AdaptiveSpeedOsu(_) => AdaptiveSpeedOsu::description(),
            Self::FreezeFrameOsu(_) => FreezeFrameOsu::description(),
            Self::BubblesOsu(_) => BubblesOsu::description(),
            Self::SynesthesiaOsu(_) => SynesthesiaOsu::description(),
            Self::DepthOsu(_) => DepthOsu::description(),
            Self::BloomOsu(_) => BloomOsu::description(),
            Self::TouchDeviceOsu(_) => TouchDeviceOsu::description(),
            Self::ScoreV2Osu(_) => ScoreV2Osu::description(),
            Self::EasyTaiko(_) => EasyTaiko::description(),
            Self::NoFailTaiko(_) => NoFailTaiko::description(),
            Self::HalfTimeTaiko(_) => HalfTimeTaiko::description(),
            Self::DaycoreTaiko(_) => DaycoreTaiko::description(),
            Self::HardRockTaiko(_) => HardRockTaiko::description(),
            Self::SuddenDeathTaiko(_) => SuddenDeathTaiko::description(),
            Self::PerfectTaiko(_) => PerfectTaiko::description(),
            Self::DoubleTimeTaiko(_) => DoubleTimeTaiko::description(),
            Self::NightcoreTaiko(_) => NightcoreTaiko::description(),
            Self::HiddenTaiko(_) => HiddenTaiko::description(),
            Self::FlashlightTaiko(_) => FlashlightTaiko::description(),
            Self::AccuracyChallengeTaiko(_) => AccuracyChallengeTaiko::description(),
            Self::RandomTaiko(_) => RandomTaiko::description(),
            Self::DifficultyAdjustTaiko(_) => DifficultyAdjustTaiko::description(),
            Self::ClassicTaiko(_) => ClassicTaiko::description(),
            Self::SwapTaiko(_) => SwapTaiko::description(),
            Self::SingleTapTaiko(_) => SingleTapTaiko::description(),
            Self::ConstantSpeedTaiko(_) => ConstantSpeedTaiko::description(),
            Self::AutoplayTaiko(_) => AutoplayTaiko::description(),
            Self::CinemaTaiko(_) => CinemaTaiko::description(),
            Self::RelaxTaiko(_) => RelaxTaiko::description(),
            Self::WindUpTaiko(_) => WindUpTaiko::description(),
            Self::WindDownTaiko(_) => WindDownTaiko::description(),
            Self::MutedTaiko(_) => MutedTaiko::description(),
            Self::AdaptiveSpeedTaiko(_) => AdaptiveSpeedTaiko::description(),
            Self::ScoreV2Taiko(_) => ScoreV2Taiko::description(),
            Self::EasyCatch(_) => EasyCatch::description(),
            Self::NoFailCatch(_) => NoFailCatch::description(),
            Self::HalfTimeCatch(_) => HalfTimeCatch::description(),
            Self::DaycoreCatch(_) => DaycoreCatch::description(),
            Self::HardRockCatch(_) => HardRockCatch::description(),
            Self::SuddenDeathCatch(_) => SuddenDeathCatch::description(),
            Self::PerfectCatch(_) => PerfectCatch::description(),
            Self::DoubleTimeCatch(_) => DoubleTimeCatch::description(),
            Self::NightcoreCatch(_) => NightcoreCatch::description(),
            Self::HiddenCatch(_) => HiddenCatch::description(),
            Self::FlashlightCatch(_) => FlashlightCatch::description(),
            Self::AccuracyChallengeCatch(_) => AccuracyChallengeCatch::description(),
            Self::DifficultyAdjustCatch(_) => DifficultyAdjustCatch::description(),
            Self::ClassicCatch(_) => ClassicCatch::description(),
            Self::MirrorCatch(_) => MirrorCatch::description(),
            Self::AutoplayCatch(_) => AutoplayCatch::description(),
            Self::CinemaCatch(_) => CinemaCatch::description(),
            Self::RelaxCatch(_) => RelaxCatch::description(),
            Self::WindUpCatch(_) => WindUpCatch::description(),
            Self::WindDownCatch(_) => WindDownCatch::description(),
            Self::FloatingFruitsCatch(_) => FloatingFruitsCatch::description(),
            Self::MutedCatch(_) => MutedCatch::description(),
            Self::NoScopeCatch(_) => NoScopeCatch::description(),
            Self::ScoreV2Catch(_) => ScoreV2Catch::description(),
            Self::EasyMania(_) => EasyMania::description(),
            Self::NoFailMania(_) => NoFailMania::description(),
            Self::HalfTimeMania(_) => HalfTimeMania::description(),
            Self::DaycoreMania(_) => DaycoreMania::description(),
            Self::NoReleaseMania(_) => NoReleaseMania::description(),
            Self::HardRockMania(_) => HardRockMania::description(),
            Self::SuddenDeathMania(_) => SuddenDeathMania::description(),
            Self::PerfectMania(_) => PerfectMania::description(),
            Self::DoubleTimeMania(_) => DoubleTimeMania::description(),
            Self::NightcoreMania(_) => NightcoreMania::description(),
            Self::FadeInMania(_) => FadeInMania::description(),
            Self::HiddenMania(_) => HiddenMania::description(),
            Self::CoverMania(_) => CoverMania::description(),
            Self::FlashlightMania(_) => FlashlightMania::description(),
            Self::AccuracyChallengeMania(_) => AccuracyChallengeMania::description(),
            Self::RandomMania(_) => RandomMania::description(),
            Self::DualStagesMania(_) => DualStagesMania::description(),
            Self::MirrorMania(_) => MirrorMania::description(),
            Self::DifficultyAdjustMania(_) => DifficultyAdjustMania::description(),
            Self::ClassicMania(_) => ClassicMania::description(),
            Self::InvertMania(_) => InvertMania::description(),
            Self::ConstantSpeedMania(_) => ConstantSpeedMania::description(),
            Self::HoldOffMania(_) => HoldOffMania::description(),
            Self::OneKeyMania(_) => OneKeyMania::description(),
            Self::TwoKeysMania(_) => TwoKeysMania::description(),
            Self::ThreeKeysMania(_) => ThreeKeysMania::description(),
            Self::FourKeysMania(_) => FourKeysMania::description(),
            Self::FiveKeysMania(_) => FiveKeysMania::description(),
            Self::SixKeysMania(_) => SixKeysMania::description(),
            Self::SevenKeysMania(_) => SevenKeysMania::description(),
            Self::EightKeysMania(_) => EightKeysMania::description(),
            Self::NineKeysMania(_) => NineKeysMania::description(),
            Self::TenKeysMania(_) => TenKeysMania::description(),
            Self::AutoplayMania(_) => AutoplayMania::description(),
            Self::CinemaMania(_) => CinemaMania::description(),
            Self::WindUpMania(_) => WindUpMania::description(),
            Self::WindDownMania(_) => WindDownMania::description(),
            Self::MutedMania(_) => MutedMania::description(),
            Self::AdaptiveSpeedMania(_) => AdaptiveSpeedMania::description(),
            Self::ScoreV2Mania(_) => ScoreV2Mania::description(),
            _ => UnknownMod::description(),
        }
    }
    /// The [`GameModKind`] of this [`GameMod`]
    pub const fn kind(&self) -> GameModKind {
        match self {
            Self::EasyOsu(_) => EasyOsu::kind(),
            Self::NoFailOsu(_) => NoFailOsu::kind(),
            Self::HalfTimeOsu(_) => HalfTimeOsu::kind(),
            Self::DaycoreOsu(_) => DaycoreOsu::kind(),
            Self::HardRockOsu(_) => HardRockOsu::kind(),
            Self::SuddenDeathOsu(_) => SuddenDeathOsu::kind(),
            Self::PerfectOsu(_) => PerfectOsu::kind(),
            Self::DoubleTimeOsu(_) => DoubleTimeOsu::kind(),
            Self::NightcoreOsu(_) => NightcoreOsu::kind(),
            Self::HiddenOsu(_) => HiddenOsu::kind(),
            Self::FlashlightOsu(_) => FlashlightOsu::kind(),
            Self::BlindsOsu(_) => BlindsOsu::kind(),
            Self::StrictTrackingOsu(_) => StrictTrackingOsu::kind(),
            Self::AccuracyChallengeOsu(_) => AccuracyChallengeOsu::kind(),
            Self::TargetPracticeOsu(_) => TargetPracticeOsu::kind(),
            Self::DifficultyAdjustOsu(_) => DifficultyAdjustOsu::kind(),
            Self::ClassicOsu(_) => ClassicOsu::kind(),
            Self::RandomOsu(_) => RandomOsu::kind(),
            Self::MirrorOsu(_) => MirrorOsu::kind(),
            Self::AlternateOsu(_) => AlternateOsu::kind(),
            Self::SingleTapOsu(_) => SingleTapOsu::kind(),
            Self::AutoplayOsu(_) => AutoplayOsu::kind(),
            Self::CinemaOsu(_) => CinemaOsu::kind(),
            Self::RelaxOsu(_) => RelaxOsu::kind(),
            Self::AutopilotOsu(_) => AutopilotOsu::kind(),
            Self::SpunOutOsu(_) => SpunOutOsu::kind(),
            Self::TransformOsu(_) => TransformOsu::kind(),
            Self::WiggleOsu(_) => WiggleOsu::kind(),
            Self::SpinInOsu(_) => SpinInOsu::kind(),
            Self::GrowOsu(_) => GrowOsu::kind(),
            Self::DeflateOsu(_) => DeflateOsu::kind(),
            Self::WindUpOsu(_) => WindUpOsu::kind(),
            Self::WindDownOsu(_) => WindDownOsu::kind(),
            Self::TraceableOsu(_) => TraceableOsu::kind(),
            Self::BarrelRollOsu(_) => BarrelRollOsu::kind(),
            Self::ApproachDifferentOsu(_) => ApproachDifferentOsu::kind(),
            Self::MutedOsu(_) => MutedOsu::kind(),
            Self::NoScopeOsu(_) => NoScopeOsu::kind(),
            Self::MagnetisedOsu(_) => MagnetisedOsu::kind(),
            Self::RepelOsu(_) => RepelOsu::kind(),
            Self::AdaptiveSpeedOsu(_) => AdaptiveSpeedOsu::kind(),
            Self::FreezeFrameOsu(_) => FreezeFrameOsu::kind(),
            Self::BubblesOsu(_) => BubblesOsu::kind(),
            Self::SynesthesiaOsu(_) => SynesthesiaOsu::kind(),
            Self::DepthOsu(_) => DepthOsu::kind(),
            Self::BloomOsu(_) => BloomOsu::kind(),
            Self::TouchDeviceOsu(_) => TouchDeviceOsu::kind(),
            Self::ScoreV2Osu(_) => ScoreV2Osu::kind(),
            Self::EasyTaiko(_) => EasyTaiko::kind(),
            Self::NoFailTaiko(_) => NoFailTaiko::kind(),
            Self::HalfTimeTaiko(_) => HalfTimeTaiko::kind(),
            Self::DaycoreTaiko(_) => DaycoreTaiko::kind(),
            Self::HardRockTaiko(_) => HardRockTaiko::kind(),
            Self::SuddenDeathTaiko(_) => SuddenDeathTaiko::kind(),
            Self::PerfectTaiko(_) => PerfectTaiko::kind(),
            Self::DoubleTimeTaiko(_) => DoubleTimeTaiko::kind(),
            Self::NightcoreTaiko(_) => NightcoreTaiko::kind(),
            Self::HiddenTaiko(_) => HiddenTaiko::kind(),
            Self::FlashlightTaiko(_) => FlashlightTaiko::kind(),
            Self::AccuracyChallengeTaiko(_) => AccuracyChallengeTaiko::kind(),
            Self::RandomTaiko(_) => RandomTaiko::kind(),
            Self::DifficultyAdjustTaiko(_) => DifficultyAdjustTaiko::kind(),
            Self::ClassicTaiko(_) => ClassicTaiko::kind(),
            Self::SwapTaiko(_) => SwapTaiko::kind(),
            Self::SingleTapTaiko(_) => SingleTapTaiko::kind(),
            Self::ConstantSpeedTaiko(_) => ConstantSpeedTaiko::kind(),
            Self::AutoplayTaiko(_) => AutoplayTaiko::kind(),
            Self::CinemaTaiko(_) => CinemaTaiko::kind(),
            Self::RelaxTaiko(_) => RelaxTaiko::kind(),
            Self::WindUpTaiko(_) => WindUpTaiko::kind(),
            Self::WindDownTaiko(_) => WindDownTaiko::kind(),
            Self::MutedTaiko(_) => MutedTaiko::kind(),
            Self::AdaptiveSpeedTaiko(_) => AdaptiveSpeedTaiko::kind(),
            Self::ScoreV2Taiko(_) => ScoreV2Taiko::kind(),
            Self::EasyCatch(_) => EasyCatch::kind(),
            Self::NoFailCatch(_) => NoFailCatch::kind(),
            Self::HalfTimeCatch(_) => HalfTimeCatch::kind(),
            Self::DaycoreCatch(_) => DaycoreCatch::kind(),
            Self::HardRockCatch(_) => HardRockCatch::kind(),
            Self::SuddenDeathCatch(_) => SuddenDeathCatch::kind(),
            Self::PerfectCatch(_) => PerfectCatch::kind(),
            Self::DoubleTimeCatch(_) => DoubleTimeCatch::kind(),
            Self::NightcoreCatch(_) => NightcoreCatch::kind(),
            Self::HiddenCatch(_) => HiddenCatch::kind(),
            Self::FlashlightCatch(_) => FlashlightCatch::kind(),
            Self::AccuracyChallengeCatch(_) => AccuracyChallengeCatch::kind(),
            Self::DifficultyAdjustCatch(_) => DifficultyAdjustCatch::kind(),
            Self::ClassicCatch(_) => ClassicCatch::kind(),
            Self::MirrorCatch(_) => MirrorCatch::kind(),
            Self::AutoplayCatch(_) => AutoplayCatch::kind(),
            Self::CinemaCatch(_) => CinemaCatch::kind(),
            Self::RelaxCatch(_) => RelaxCatch::kind(),
            Self::WindUpCatch(_) => WindUpCatch::kind(),
            Self::WindDownCatch(_) => WindDownCatch::kind(),
            Self::FloatingFruitsCatch(_) => FloatingFruitsCatch::kind(),
            Self::MutedCatch(_) => MutedCatch::kind(),
            Self::NoScopeCatch(_) => NoScopeCatch::kind(),
            Self::ScoreV2Catch(_) => ScoreV2Catch::kind(),
            Self::EasyMania(_) => EasyMania::kind(),
            Self::NoFailMania(_) => NoFailMania::kind(),
            Self::HalfTimeMania(_) => HalfTimeMania::kind(),
            Self::DaycoreMania(_) => DaycoreMania::kind(),
            Self::NoReleaseMania(_) => NoReleaseMania::kind(),
            Self::HardRockMania(_) => HardRockMania::kind(),
            Self::SuddenDeathMania(_) => SuddenDeathMania::kind(),
            Self::PerfectMania(_) => PerfectMania::kind(),
            Self::DoubleTimeMania(_) => DoubleTimeMania::kind(),
            Self::NightcoreMania(_) => NightcoreMania::kind(),
            Self::FadeInMania(_) => FadeInMania::kind(),
            Self::HiddenMania(_) => HiddenMania::kind(),
            Self::CoverMania(_) => CoverMania::kind(),
            Self::FlashlightMania(_) => FlashlightMania::kind(),
            Self::AccuracyChallengeMania(_) => AccuracyChallengeMania::kind(),
            Self::RandomMania(_) => RandomMania::kind(),
            Self::DualStagesMania(_) => DualStagesMania::kind(),
            Self::MirrorMania(_) => MirrorMania::kind(),
            Self::DifficultyAdjustMania(_) => DifficultyAdjustMania::kind(),
            Self::ClassicMania(_) => ClassicMania::kind(),
            Self::InvertMania(_) => InvertMania::kind(),
            Self::ConstantSpeedMania(_) => ConstantSpeedMania::kind(),
            Self::HoldOffMania(_) => HoldOffMania::kind(),
            Self::OneKeyMania(_) => OneKeyMania::kind(),
            Self::TwoKeysMania(_) => TwoKeysMania::kind(),
            Self::ThreeKeysMania(_) => ThreeKeysMania::kind(),
            Self::FourKeysMania(_) => FourKeysMania::kind(),
            Self::FiveKeysMania(_) => FiveKeysMania::kind(),
            Self::SixKeysMania(_) => SixKeysMania::kind(),
            Self::SevenKeysMania(_) => SevenKeysMania::kind(),
            Self::EightKeysMania(_) => EightKeysMania::kind(),
            Self::NineKeysMania(_) => NineKeysMania::kind(),
            Self::TenKeysMania(_) => TenKeysMania::kind(),
            Self::AutoplayMania(_) => AutoplayMania::kind(),
            Self::CinemaMania(_) => CinemaMania::kind(),
            Self::WindUpMania(_) => WindUpMania::kind(),
            Self::WindDownMania(_) => WindDownMania::kind(),
            Self::MutedMania(_) => MutedMania::kind(),
            Self::AdaptiveSpeedMania(_) => AdaptiveSpeedMania::kind(),
            Self::ScoreV2Mania(_) => ScoreV2Mania::kind(),
            _ => UnknownMod::kind(),
        }
    }
    /// Optional bit value of this [`GameMod`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits(&self) -> Option<u32> {
        match self {
            Self::EasyOsu(_) => Some(EasyOsu::bits()),
            Self::NoFailOsu(_) => Some(NoFailOsu::bits()),
            Self::HalfTimeOsu(_) => Some(HalfTimeOsu::bits()),
            Self::HardRockOsu(_) => Some(HardRockOsu::bits()),
            Self::SuddenDeathOsu(_) => Some(SuddenDeathOsu::bits()),
            Self::PerfectOsu(_) => Some(PerfectOsu::bits()),
            Self::DoubleTimeOsu(_) => Some(DoubleTimeOsu::bits()),
            Self::NightcoreOsu(_) => Some(NightcoreOsu::bits()),
            Self::HiddenOsu(_) => Some(HiddenOsu::bits()),
            Self::FlashlightOsu(_) => Some(FlashlightOsu::bits()),
            Self::TargetPracticeOsu(_) => Some(TargetPracticeOsu::bits()),
            Self::RandomOsu(_) => Some(RandomOsu::bits()),
            Self::MirrorOsu(_) => Some(MirrorOsu::bits()),
            Self::AutoplayOsu(_) => Some(AutoplayOsu::bits()),
            Self::CinemaOsu(_) => Some(CinemaOsu::bits()),
            Self::RelaxOsu(_) => Some(RelaxOsu::bits()),
            Self::AutopilotOsu(_) => Some(AutopilotOsu::bits()),
            Self::SpunOutOsu(_) => Some(SpunOutOsu::bits()),
            Self::TouchDeviceOsu(_) => Some(TouchDeviceOsu::bits()),
            Self::ScoreV2Osu(_) => Some(ScoreV2Osu::bits()),
            Self::EasyTaiko(_) => Some(EasyTaiko::bits()),
            Self::NoFailTaiko(_) => Some(NoFailTaiko::bits()),
            Self::HalfTimeTaiko(_) => Some(HalfTimeTaiko::bits()),
            Self::HardRockTaiko(_) => Some(HardRockTaiko::bits()),
            Self::SuddenDeathTaiko(_) => Some(SuddenDeathTaiko::bits()),
            Self::PerfectTaiko(_) => Some(PerfectTaiko::bits()),
            Self::DoubleTimeTaiko(_) => Some(DoubleTimeTaiko::bits()),
            Self::NightcoreTaiko(_) => Some(NightcoreTaiko::bits()),
            Self::HiddenTaiko(_) => Some(HiddenTaiko::bits()),
            Self::FlashlightTaiko(_) => Some(FlashlightTaiko::bits()),
            Self::RandomTaiko(_) => Some(RandomTaiko::bits()),
            Self::AutoplayTaiko(_) => Some(AutoplayTaiko::bits()),
            Self::CinemaTaiko(_) => Some(CinemaTaiko::bits()),
            Self::RelaxTaiko(_) => Some(RelaxTaiko::bits()),
            Self::ScoreV2Taiko(_) => Some(ScoreV2Taiko::bits()),
            Self::EasyCatch(_) => Some(EasyCatch::bits()),
            Self::NoFailCatch(_) => Some(NoFailCatch::bits()),
            Self::HalfTimeCatch(_) => Some(HalfTimeCatch::bits()),
            Self::HardRockCatch(_) => Some(HardRockCatch::bits()),
            Self::SuddenDeathCatch(_) => Some(SuddenDeathCatch::bits()),
            Self::PerfectCatch(_) => Some(PerfectCatch::bits()),
            Self::DoubleTimeCatch(_) => Some(DoubleTimeCatch::bits()),
            Self::NightcoreCatch(_) => Some(NightcoreCatch::bits()),
            Self::HiddenCatch(_) => Some(HiddenCatch::bits()),
            Self::FlashlightCatch(_) => Some(FlashlightCatch::bits()),
            Self::MirrorCatch(_) => Some(MirrorCatch::bits()),
            Self::AutoplayCatch(_) => Some(AutoplayCatch::bits()),
            Self::CinemaCatch(_) => Some(CinemaCatch::bits()),
            Self::RelaxCatch(_) => Some(RelaxCatch::bits()),
            Self::ScoreV2Catch(_) => Some(ScoreV2Catch::bits()),
            Self::EasyMania(_) => Some(EasyMania::bits()),
            Self::NoFailMania(_) => Some(NoFailMania::bits()),
            Self::HalfTimeMania(_) => Some(HalfTimeMania::bits()),
            Self::HardRockMania(_) => Some(HardRockMania::bits()),
            Self::SuddenDeathMania(_) => Some(SuddenDeathMania::bits()),
            Self::PerfectMania(_) => Some(PerfectMania::bits()),
            Self::DoubleTimeMania(_) => Some(DoubleTimeMania::bits()),
            Self::NightcoreMania(_) => Some(NightcoreMania::bits()),
            Self::FadeInMania(_) => Some(FadeInMania::bits()),
            Self::HiddenMania(_) => Some(HiddenMania::bits()),
            Self::FlashlightMania(_) => Some(FlashlightMania::bits()),
            Self::RandomMania(_) => Some(RandomMania::bits()),
            Self::DualStagesMania(_) => Some(DualStagesMania::bits()),
            Self::MirrorMania(_) => Some(MirrorMania::bits()),
            Self::OneKeyMania(_) => Some(OneKeyMania::bits()),
            Self::TwoKeysMania(_) => Some(TwoKeysMania::bits()),
            Self::ThreeKeysMania(_) => Some(ThreeKeysMania::bits()),
            Self::FourKeysMania(_) => Some(FourKeysMania::bits()),
            Self::FiveKeysMania(_) => Some(FiveKeysMania::bits()),
            Self::SixKeysMania(_) => Some(SixKeysMania::bits()),
            Self::SevenKeysMania(_) => Some(SevenKeysMania::bits()),
            Self::EightKeysMania(_) => Some(EightKeysMania::bits()),
            Self::NineKeysMania(_) => Some(NineKeysMania::bits()),
            Self::AutoplayMania(_) => Some(AutoplayMania::bits()),
            Self::CinemaMania(_) => Some(CinemaMania::bits()),
            Self::ScoreV2Mania(_) => Some(ScoreV2Mania::bits()),
            _ => None,
        }
    }
    /// The [`GameMode`] of a [`GameMod`]
    pub const fn mode(&self) -> GameMode {
        match self {
            Self::EasyOsu(_)
            | Self::NoFailOsu(_)
            | Self::HalfTimeOsu(_)
            | Self::DaycoreOsu(_)
            | Self::HardRockOsu(_)
            | Self::SuddenDeathOsu(_)
            | Self::PerfectOsu(_)
            | Self::DoubleTimeOsu(_)
            | Self::NightcoreOsu(_)
            | Self::HiddenOsu(_)
            | Self::FlashlightOsu(_)
            | Self::BlindsOsu(_)
            | Self::StrictTrackingOsu(_)
            | Self::AccuracyChallengeOsu(_)
            | Self::TargetPracticeOsu(_)
            | Self::DifficultyAdjustOsu(_)
            | Self::ClassicOsu(_)
            | Self::RandomOsu(_)
            | Self::MirrorOsu(_)
            | Self::AlternateOsu(_)
            | Self::SingleTapOsu(_)
            | Self::AutoplayOsu(_)
            | Self::CinemaOsu(_)
            | Self::RelaxOsu(_)
            | Self::AutopilotOsu(_)
            | Self::SpunOutOsu(_)
            | Self::TransformOsu(_)
            | Self::WiggleOsu(_)
            | Self::SpinInOsu(_)
            | Self::GrowOsu(_)
            | Self::DeflateOsu(_)
            | Self::WindUpOsu(_)
            | Self::WindDownOsu(_)
            | Self::TraceableOsu(_)
            | Self::BarrelRollOsu(_)
            | Self::ApproachDifferentOsu(_)
            | Self::MutedOsu(_)
            | Self::NoScopeOsu(_)
            | Self::MagnetisedOsu(_)
            | Self::RepelOsu(_)
            | Self::AdaptiveSpeedOsu(_)
            | Self::FreezeFrameOsu(_)
            | Self::BubblesOsu(_)
            | Self::SynesthesiaOsu(_)
            | Self::DepthOsu(_)
            | Self::BloomOsu(_)
            | Self::TouchDeviceOsu(_)
            | Self::ScoreV2Osu(_)
            | Self::UnknownOsu(_) => GameMode::Osu,
            Self::EasyTaiko(_)
            | Self::NoFailTaiko(_)
            | Self::HalfTimeTaiko(_)
            | Self::DaycoreTaiko(_)
            | Self::HardRockTaiko(_)
            | Self::SuddenDeathTaiko(_)
            | Self::PerfectTaiko(_)
            | Self::DoubleTimeTaiko(_)
            | Self::NightcoreTaiko(_)
            | Self::HiddenTaiko(_)
            | Self::FlashlightTaiko(_)
            | Self::AccuracyChallengeTaiko(_)
            | Self::RandomTaiko(_)
            | Self::DifficultyAdjustTaiko(_)
            | Self::ClassicTaiko(_)
            | Self::SwapTaiko(_)
            | Self::SingleTapTaiko(_)
            | Self::ConstantSpeedTaiko(_)
            | Self::AutoplayTaiko(_)
            | Self::CinemaTaiko(_)
            | Self::RelaxTaiko(_)
            | Self::WindUpTaiko(_)
            | Self::WindDownTaiko(_)
            | Self::MutedTaiko(_)
            | Self::AdaptiveSpeedTaiko(_)
            | Self::ScoreV2Taiko(_)
            | Self::UnknownTaiko(_) => GameMode::Taiko,
            Self::EasyCatch(_)
            | Self::NoFailCatch(_)
            | Self::HalfTimeCatch(_)
            | Self::DaycoreCatch(_)
            | Self::HardRockCatch(_)
            | Self::SuddenDeathCatch(_)
            | Self::PerfectCatch(_)
            | Self::DoubleTimeCatch(_)
            | Self::NightcoreCatch(_)
            | Self::HiddenCatch(_)
            | Self::FlashlightCatch(_)
            | Self::AccuracyChallengeCatch(_)
            | Self::DifficultyAdjustCatch(_)
            | Self::ClassicCatch(_)
            | Self::MirrorCatch(_)
            | Self::AutoplayCatch(_)
            | Self::CinemaCatch(_)
            | Self::RelaxCatch(_)
            | Self::WindUpCatch(_)
            | Self::WindDownCatch(_)
            | Self::FloatingFruitsCatch(_)
            | Self::MutedCatch(_)
            | Self::NoScopeCatch(_)
            | Self::ScoreV2Catch(_)
            | Self::UnknownCatch(_) => GameMode::Catch,
            Self::EasyMania(_)
            | Self::NoFailMania(_)
            | Self::HalfTimeMania(_)
            | Self::DaycoreMania(_)
            | Self::NoReleaseMania(_)
            | Self::HardRockMania(_)
            | Self::SuddenDeathMania(_)
            | Self::PerfectMania(_)
            | Self::DoubleTimeMania(_)
            | Self::NightcoreMania(_)
            | Self::FadeInMania(_)
            | Self::HiddenMania(_)
            | Self::CoverMania(_)
            | Self::FlashlightMania(_)
            | Self::AccuracyChallengeMania(_)
            | Self::RandomMania(_)
            | Self::DualStagesMania(_)
            | Self::MirrorMania(_)
            | Self::DifficultyAdjustMania(_)
            | Self::ClassicMania(_)
            | Self::InvertMania(_)
            | Self::ConstantSpeedMania(_)
            | Self::HoldOffMania(_)
            | Self::OneKeyMania(_)
            | Self::TwoKeysMania(_)
            | Self::ThreeKeysMania(_)
            | Self::FourKeysMania(_)
            | Self::FiveKeysMania(_)
            | Self::SixKeysMania(_)
            | Self::SevenKeysMania(_)
            | Self::EightKeysMania(_)
            | Self::NineKeysMania(_)
            | Self::TenKeysMania(_)
            | Self::AutoplayMania(_)
            | Self::CinemaMania(_)
            | Self::WindUpMania(_)
            | Self::WindDownMania(_)
            | Self::MutedMania(_)
            | Self::AdaptiveSpeedMania(_)
            | Self::ScoreV2Mania(_)
            | Self::UnknownMania(_) => GameMode::Mania,
        }
    }
    /// The kind of a [`GameMod`] when ignoring the mode
    pub const fn intermode(&self) -> GameModIntermode {
        match self {
            Self::EasyOsu(_) => GameModIntermode::Easy,
            Self::NoFailOsu(_) => GameModIntermode::NoFail,
            Self::HalfTimeOsu(_) => GameModIntermode::HalfTime,
            Self::DaycoreOsu(_) => GameModIntermode::Daycore,
            Self::HardRockOsu(_) => GameModIntermode::HardRock,
            Self::SuddenDeathOsu(_) => GameModIntermode::SuddenDeath,
            Self::PerfectOsu(_) => GameModIntermode::Perfect,
            Self::DoubleTimeOsu(_) => GameModIntermode::DoubleTime,
            Self::NightcoreOsu(_) => GameModIntermode::Nightcore,
            Self::HiddenOsu(_) => GameModIntermode::Hidden,
            Self::FlashlightOsu(_) => GameModIntermode::Flashlight,
            Self::BlindsOsu(_) => GameModIntermode::Blinds,
            Self::StrictTrackingOsu(_) => GameModIntermode::StrictTracking,
            Self::AccuracyChallengeOsu(_) => GameModIntermode::AccuracyChallenge,
            Self::TargetPracticeOsu(_) => GameModIntermode::TargetPractice,
            Self::DifficultyAdjustOsu(_) => GameModIntermode::DifficultyAdjust,
            Self::ClassicOsu(_) => GameModIntermode::Classic,
            Self::RandomOsu(_) => GameModIntermode::Random,
            Self::MirrorOsu(_) => GameModIntermode::Mirror,
            Self::AlternateOsu(_) => GameModIntermode::Alternate,
            Self::SingleTapOsu(_) => GameModIntermode::SingleTap,
            Self::AutoplayOsu(_) => GameModIntermode::Autoplay,
            Self::CinemaOsu(_) => GameModIntermode::Cinema,
            Self::RelaxOsu(_) => GameModIntermode::Relax,
            Self::AutopilotOsu(_) => GameModIntermode::Autopilot,
            Self::SpunOutOsu(_) => GameModIntermode::SpunOut,
            Self::TransformOsu(_) => GameModIntermode::Transform,
            Self::WiggleOsu(_) => GameModIntermode::Wiggle,
            Self::SpinInOsu(_) => GameModIntermode::SpinIn,
            Self::GrowOsu(_) => GameModIntermode::Grow,
            Self::DeflateOsu(_) => GameModIntermode::Deflate,
            Self::WindUpOsu(_) => GameModIntermode::WindUp,
            Self::WindDownOsu(_) => GameModIntermode::WindDown,
            Self::TraceableOsu(_) => GameModIntermode::Traceable,
            Self::BarrelRollOsu(_) => GameModIntermode::BarrelRoll,
            Self::ApproachDifferentOsu(_) => GameModIntermode::ApproachDifferent,
            Self::MutedOsu(_) => GameModIntermode::Muted,
            Self::NoScopeOsu(_) => GameModIntermode::NoScope,
            Self::MagnetisedOsu(_) => GameModIntermode::Magnetised,
            Self::RepelOsu(_) => GameModIntermode::Repel,
            Self::AdaptiveSpeedOsu(_) => GameModIntermode::AdaptiveSpeed,
            Self::FreezeFrameOsu(_) => GameModIntermode::FreezeFrame,
            Self::BubblesOsu(_) => GameModIntermode::Bubbles,
            Self::SynesthesiaOsu(_) => GameModIntermode::Synesthesia,
            Self::DepthOsu(_) => GameModIntermode::Depth,
            Self::BloomOsu(_) => GameModIntermode::Bloom,
            Self::TouchDeviceOsu(_) => GameModIntermode::TouchDevice,
            Self::ScoreV2Osu(_) => GameModIntermode::ScoreV2,
            Self::EasyTaiko(_) => GameModIntermode::Easy,
            Self::NoFailTaiko(_) => GameModIntermode::NoFail,
            Self::HalfTimeTaiko(_) => GameModIntermode::HalfTime,
            Self::DaycoreTaiko(_) => GameModIntermode::Daycore,
            Self::HardRockTaiko(_) => GameModIntermode::HardRock,
            Self::SuddenDeathTaiko(_) => GameModIntermode::SuddenDeath,
            Self::PerfectTaiko(_) => GameModIntermode::Perfect,
            Self::DoubleTimeTaiko(_) => GameModIntermode::DoubleTime,
            Self::NightcoreTaiko(_) => GameModIntermode::Nightcore,
            Self::HiddenTaiko(_) => GameModIntermode::Hidden,
            Self::FlashlightTaiko(_) => GameModIntermode::Flashlight,
            Self::AccuracyChallengeTaiko(_) => GameModIntermode::AccuracyChallenge,
            Self::RandomTaiko(_) => GameModIntermode::Random,
            Self::DifficultyAdjustTaiko(_) => GameModIntermode::DifficultyAdjust,
            Self::ClassicTaiko(_) => GameModIntermode::Classic,
            Self::SwapTaiko(_) => GameModIntermode::Swap,
            Self::SingleTapTaiko(_) => GameModIntermode::SingleTap,
            Self::ConstantSpeedTaiko(_) => GameModIntermode::ConstantSpeed,
            Self::AutoplayTaiko(_) => GameModIntermode::Autoplay,
            Self::CinemaTaiko(_) => GameModIntermode::Cinema,
            Self::RelaxTaiko(_) => GameModIntermode::Relax,
            Self::WindUpTaiko(_) => GameModIntermode::WindUp,
            Self::WindDownTaiko(_) => GameModIntermode::WindDown,
            Self::MutedTaiko(_) => GameModIntermode::Muted,
            Self::AdaptiveSpeedTaiko(_) => GameModIntermode::AdaptiveSpeed,
            Self::ScoreV2Taiko(_) => GameModIntermode::ScoreV2,
            Self::EasyCatch(_) => GameModIntermode::Easy,
            Self::NoFailCatch(_) => GameModIntermode::NoFail,
            Self::HalfTimeCatch(_) => GameModIntermode::HalfTime,
            Self::DaycoreCatch(_) => GameModIntermode::Daycore,
            Self::HardRockCatch(_) => GameModIntermode::HardRock,
            Self::SuddenDeathCatch(_) => GameModIntermode::SuddenDeath,
            Self::PerfectCatch(_) => GameModIntermode::Perfect,
            Self::DoubleTimeCatch(_) => GameModIntermode::DoubleTime,
            Self::NightcoreCatch(_) => GameModIntermode::Nightcore,
            Self::HiddenCatch(_) => GameModIntermode::Hidden,
            Self::FlashlightCatch(_) => GameModIntermode::Flashlight,
            Self::AccuracyChallengeCatch(_) => GameModIntermode::AccuracyChallenge,
            Self::DifficultyAdjustCatch(_) => GameModIntermode::DifficultyAdjust,
            Self::ClassicCatch(_) => GameModIntermode::Classic,
            Self::MirrorCatch(_) => GameModIntermode::Mirror,
            Self::AutoplayCatch(_) => GameModIntermode::Autoplay,
            Self::CinemaCatch(_) => GameModIntermode::Cinema,
            Self::RelaxCatch(_) => GameModIntermode::Relax,
            Self::WindUpCatch(_) => GameModIntermode::WindUp,
            Self::WindDownCatch(_) => GameModIntermode::WindDown,
            Self::FloatingFruitsCatch(_) => GameModIntermode::FloatingFruits,
            Self::MutedCatch(_) => GameModIntermode::Muted,
            Self::NoScopeCatch(_) => GameModIntermode::NoScope,
            Self::ScoreV2Catch(_) => GameModIntermode::ScoreV2,
            Self::EasyMania(_) => GameModIntermode::Easy,
            Self::NoFailMania(_) => GameModIntermode::NoFail,
            Self::HalfTimeMania(_) => GameModIntermode::HalfTime,
            Self::DaycoreMania(_) => GameModIntermode::Daycore,
            Self::NoReleaseMania(_) => GameModIntermode::NoRelease,
            Self::HardRockMania(_) => GameModIntermode::HardRock,
            Self::SuddenDeathMania(_) => GameModIntermode::SuddenDeath,
            Self::PerfectMania(_) => GameModIntermode::Perfect,
            Self::DoubleTimeMania(_) => GameModIntermode::DoubleTime,
            Self::NightcoreMania(_) => GameModIntermode::Nightcore,
            Self::FadeInMania(_) => GameModIntermode::FadeIn,
            Self::HiddenMania(_) => GameModIntermode::Hidden,
            Self::CoverMania(_) => GameModIntermode::Cover,
            Self::FlashlightMania(_) => GameModIntermode::Flashlight,
            Self::AccuracyChallengeMania(_) => GameModIntermode::AccuracyChallenge,
            Self::RandomMania(_) => GameModIntermode::Random,
            Self::DualStagesMania(_) => GameModIntermode::DualStages,
            Self::MirrorMania(_) => GameModIntermode::Mirror,
            Self::DifficultyAdjustMania(_) => GameModIntermode::DifficultyAdjust,
            Self::ClassicMania(_) => GameModIntermode::Classic,
            Self::InvertMania(_) => GameModIntermode::Invert,
            Self::ConstantSpeedMania(_) => GameModIntermode::ConstantSpeed,
            Self::HoldOffMania(_) => GameModIntermode::HoldOff,
            Self::OneKeyMania(_) => GameModIntermode::OneKey,
            Self::TwoKeysMania(_) => GameModIntermode::TwoKeys,
            Self::ThreeKeysMania(_) => GameModIntermode::ThreeKeys,
            Self::FourKeysMania(_) => GameModIntermode::FourKeys,
            Self::FiveKeysMania(_) => GameModIntermode::FiveKeys,
            Self::SixKeysMania(_) => GameModIntermode::SixKeys,
            Self::SevenKeysMania(_) => GameModIntermode::SevenKeys,
            Self::EightKeysMania(_) => GameModIntermode::EightKeys,
            Self::NineKeysMania(_) => GameModIntermode::NineKeys,
            Self::TenKeysMania(_) => GameModIntermode::TenKeys,
            Self::AutoplayMania(_) => GameModIntermode::Autoplay,
            Self::CinemaMania(_) => GameModIntermode::Cinema,
            Self::WindUpMania(_) => GameModIntermode::WindUp,
            Self::WindDownMania(_) => GameModIntermode::WindDown,
            Self::MutedMania(_) => GameModIntermode::Muted,
            Self::AdaptiveSpeedMania(_) => GameModIntermode::AdaptiveSpeed,
            Self::ScoreV2Mania(_) => GameModIntermode::ScoreV2,
            Self::UnknownOsu(m)
            | Self::UnknownTaiko(m)
            | Self::UnknownCatch(m)
            | Self::UnknownMania(m) => GameModIntermode::Unknown(*m),
        }
    }
}
impl PartialOrd for GameMod {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.bits()
            .zip(other.bits())
            .map(|(self_bits, other_bits)| self_bits.cmp(&other_bits))
    }
}
#[cfg(feature = "serde")]
#[cfg_attr(all(docsrs, not(doctest)), doc(cfg(feature = "serde")))]
const _: () = {
    use serde::{
        de::{
            value::MapAccessDeserializer, DeserializeSeed, Deserializer, Error as DeError,
            IgnoredAny, MapAccess, Visitor,
        },
        ser::{Serialize, SerializeMap, Serializer},
        Deserialize,
    };

    use crate::serde::{
        DeserializedGameMod, GameModRaw, GameModRawSeed, GameModSettings, GameModSettingsSeed,
        GameModVisitor, MaybeOwnedStr,
    };

    impl<'de> Visitor<'de> for GameModVisitor<EasyOsu> {
        type Value = DeserializedGameMod<'de, EasyOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("EasyOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["retries"];
            let mut unknown_key__ = None;
            let mut retries = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "retries" => retries = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = EasyOsu {
                retries: retries.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for EasyOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.retries.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.retries {
                map.serialize_entry("retries", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<NoFailOsu> {
        type Value = DeserializedGameMod<'de, NoFailOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("NoFailOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = NoFailOsu {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for NoFailOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<HalfTimeOsu> {
        type Value = DeserializedGameMod<'de, HalfTimeOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("HalfTimeOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["speed_change", "adjust_pitch"];
            let mut unknown_key__ = None;
            let mut speed_change = None;
            let mut adjust_pitch = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "speed_change" => speed_change = Some(map.next_value()?),
                    "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = HalfTimeOsu {
                speed_change: speed_change.unwrap_or_default(),
                adjust_pitch: adjust_pitch.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for HalfTimeOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count =
                self.speed_change.is_some() as usize + self.adjust_pitch.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.speed_change {
                map.serialize_entry("speed_change", x)?;
            }
            if let Some(ref x) = self.adjust_pitch {
                map.serialize_entry("adjust_pitch", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<DaycoreOsu> {
        type Value = DeserializedGameMod<'de, DaycoreOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("DaycoreOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["speed_change"];
            let mut unknown_key__ = None;
            let mut speed_change = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "speed_change" => speed_change = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = DaycoreOsu {
                speed_change: speed_change.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for DaycoreOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.speed_change.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.speed_change {
                map.serialize_entry("speed_change", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<HardRockOsu> {
        type Value = DeserializedGameMod<'de, HardRockOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("HardRockOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = HardRockOsu {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for HardRockOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<SuddenDeathOsu> {
        type Value = DeserializedGameMod<'de, SuddenDeathOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("SuddenDeathOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["fail_on_slider_tail", "restart"];
            let mut unknown_key__ = None;
            let mut fail_on_slider_tail = None;
            let mut restart = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "fail_on_slider_tail" => fail_on_slider_tail = Some(map.next_value()?),
                    "restart" => restart = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = SuddenDeathOsu {
                fail_on_slider_tail: fail_on_slider_tail.unwrap_or_default(),
                restart: restart.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for SuddenDeathOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count =
                self.fail_on_slider_tail.is_some() as usize + self.restart.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.fail_on_slider_tail {
                map.serialize_entry("fail_on_slider_tail", x)?;
            }
            if let Some(ref x) = self.restart {
                map.serialize_entry("restart", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<PerfectOsu> {
        type Value = DeserializedGameMod<'de, PerfectOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("PerfectOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["restart"];
            let mut unknown_key__ = None;
            let mut restart = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "restart" => restart = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = PerfectOsu {
                restart: restart.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for PerfectOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.restart.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.restart {
                map.serialize_entry("restart", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<DoubleTimeOsu> {
        type Value = DeserializedGameMod<'de, DoubleTimeOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("DoubleTimeOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["speed_change", "adjust_pitch"];
            let mut unknown_key__ = None;
            let mut speed_change = None;
            let mut adjust_pitch = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "speed_change" => speed_change = Some(map.next_value()?),
                    "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = DoubleTimeOsu {
                speed_change: speed_change.unwrap_or_default(),
                adjust_pitch: adjust_pitch.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for DoubleTimeOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count =
                self.speed_change.is_some() as usize + self.adjust_pitch.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.speed_change {
                map.serialize_entry("speed_change", x)?;
            }
            if let Some(ref x) = self.adjust_pitch {
                map.serialize_entry("adjust_pitch", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<NightcoreOsu> {
        type Value = DeserializedGameMod<'de, NightcoreOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("NightcoreOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["speed_change"];
            let mut unknown_key__ = None;
            let mut speed_change = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "speed_change" => speed_change = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = NightcoreOsu {
                speed_change: speed_change.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for NightcoreOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.speed_change.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.speed_change {
                map.serialize_entry("speed_change", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<HiddenOsu> {
        type Value = DeserializedGameMod<'de, HiddenOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("HiddenOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["only_fade_approach_circles"];
            let mut unknown_key__ = None;
            let mut only_fade_approach_circles = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "only_fade_approach_circles" => {
                        only_fade_approach_circles = Some(map.next_value()?)
                    }
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = HiddenOsu {
                only_fade_approach_circles: only_fade_approach_circles.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for HiddenOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.only_fade_approach_circles.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.only_fade_approach_circles {
                map.serialize_entry("only_fade_approach_circles", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<FlashlightOsu> {
        type Value = DeserializedGameMod<'de, FlashlightOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("FlashlightOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] =
                &["follow_delay", "size_multiplier", "combo_based_size"];
            let mut unknown_key__ = None;
            let mut follow_delay = None;
            let mut size_multiplier = None;
            let mut combo_based_size = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "follow_delay" => follow_delay = Some(map.next_value()?),
                    "size_multiplier" => size_multiplier = Some(map.next_value()?),
                    "combo_based_size" => combo_based_size = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = FlashlightOsu {
                follow_delay: follow_delay.unwrap_or_default(),
                size_multiplier: size_multiplier.unwrap_or_default(),
                combo_based_size: combo_based_size.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for FlashlightOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.follow_delay.is_some() as usize
                + self.size_multiplier.is_some() as usize
                + self.combo_based_size.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.follow_delay {
                map.serialize_entry("follow_delay", x)?;
            }
            if let Some(ref x) = self.size_multiplier {
                map.serialize_entry("size_multiplier", x)?;
            }
            if let Some(ref x) = self.combo_based_size {
                map.serialize_entry("combo_based_size", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<BlindsOsu> {
        type Value = DeserializedGameMod<'de, BlindsOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("BlindsOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = BlindsOsu {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for BlindsOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<StrictTrackingOsu> {
        type Value = DeserializedGameMod<'de, StrictTrackingOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("StrictTrackingOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = StrictTrackingOsu {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for StrictTrackingOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<AccuracyChallengeOsu> {
        type Value = DeserializedGameMod<'de, AccuracyChallengeOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("AccuracyChallengeOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] =
                &["minimum_accuracy", "accuracy_judge_mode", "restart"];
            let mut unknown_key__ = None;
            let mut minimum_accuracy = None;
            let mut accuracy_judge_mode = None;
            let mut restart = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "minimum_accuracy" => minimum_accuracy = Some(map.next_value()?),
                    "accuracy_judge_mode" => accuracy_judge_mode = Some(map.next_value()?),
                    "restart" => restart = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = AccuracyChallengeOsu {
                minimum_accuracy: minimum_accuracy.unwrap_or_default(),
                accuracy_judge_mode: accuracy_judge_mode.unwrap_or_default(),
                restart: restart.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for AccuracyChallengeOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.minimum_accuracy.is_some() as usize
                + self.accuracy_judge_mode.is_some() as usize
                + self.restart.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.minimum_accuracy {
                map.serialize_entry("minimum_accuracy", x)?;
            }
            if let Some(ref x) = self.accuracy_judge_mode {
                map.serialize_entry("accuracy_judge_mode", x)?;
            }
            if let Some(ref x) = self.restart {
                map.serialize_entry("restart", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<TargetPracticeOsu> {
        type Value = DeserializedGameMod<'de, TargetPracticeOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("TargetPracticeOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["seed", "metronome"];
            let mut unknown_key__ = None;
            let mut seed = None;
            let mut metronome = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "seed" => seed = Some(map.next_value()?),
                    "metronome" => metronome = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = TargetPracticeOsu {
                seed: seed.unwrap_or_default(),
                metronome: metronome.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for TargetPracticeOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.seed.is_some() as usize + self.metronome.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.seed {
                map.serialize_entry("seed", x)?;
            }
            if let Some(ref x) = self.metronome {
                map.serialize_entry("metronome", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<DifficultyAdjustOsu> {
        type Value = DeserializedGameMod<'de, DifficultyAdjustOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("DifficultyAdjustOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[
                "circle_size",
                "approach_rate",
                "drain_rate",
                "overall_difficulty",
                "extended_limits",
            ];
            let mut unknown_key__ = None;
            let mut circle_size = None;
            let mut approach_rate = None;
            let mut drain_rate = None;
            let mut overall_difficulty = None;
            let mut extended_limits = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "circle_size" => circle_size = Some(map.next_value()?),
                    "approach_rate" => approach_rate = Some(map.next_value()?),
                    "drain_rate" => drain_rate = Some(map.next_value()?),
                    "overall_difficulty" => overall_difficulty = Some(map.next_value()?),
                    "extended_limits" => extended_limits = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = DifficultyAdjustOsu {
                circle_size: circle_size.unwrap_or_default(),
                approach_rate: approach_rate.unwrap_or_default(),
                drain_rate: drain_rate.unwrap_or_default(),
                overall_difficulty: overall_difficulty.unwrap_or_default(),
                extended_limits: extended_limits.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for DifficultyAdjustOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.circle_size.is_some() as usize
                + self.approach_rate.is_some() as usize
                + self.drain_rate.is_some() as usize
                + self.overall_difficulty.is_some() as usize
                + self.extended_limits.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.circle_size {
                map.serialize_entry("circle_size", x)?;
            }
            if let Some(ref x) = self.approach_rate {
                map.serialize_entry("approach_rate", x)?;
            }
            if let Some(ref x) = self.drain_rate {
                map.serialize_entry("drain_rate", x)?;
            }
            if let Some(ref x) = self.overall_difficulty {
                map.serialize_entry("overall_difficulty", x)?;
            }
            if let Some(ref x) = self.extended_limits {
                map.serialize_entry("extended_limits", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<ClassicOsu> {
        type Value = DeserializedGameMod<'de, ClassicOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("ClassicOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[
                "no_slider_head_accuracy",
                "classic_note_lock",
                "always_play_tail_sample",
                "fade_hit_circle_early",
                "classic_health",
            ];
            let mut unknown_key__ = None;
            let mut no_slider_head_accuracy = None;
            let mut classic_note_lock = None;
            let mut always_play_tail_sample = None;
            let mut fade_hit_circle_early = None;
            let mut classic_health = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "no_slider_head_accuracy" => no_slider_head_accuracy = Some(map.next_value()?),
                    "classic_note_lock" => classic_note_lock = Some(map.next_value()?),
                    "always_play_tail_sample" => always_play_tail_sample = Some(map.next_value()?),
                    "fade_hit_circle_early" => fade_hit_circle_early = Some(map.next_value()?),
                    "classic_health" => classic_health = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = ClassicOsu {
                no_slider_head_accuracy: no_slider_head_accuracy.unwrap_or_default(),
                classic_note_lock: classic_note_lock.unwrap_or_default(),
                always_play_tail_sample: always_play_tail_sample.unwrap_or_default(),
                fade_hit_circle_early: fade_hit_circle_early.unwrap_or_default(),
                classic_health: classic_health.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for ClassicOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.no_slider_head_accuracy.is_some() as usize
                + self.classic_note_lock.is_some() as usize
                + self.always_play_tail_sample.is_some() as usize
                + self.fade_hit_circle_early.is_some() as usize
                + self.classic_health.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.no_slider_head_accuracy {
                map.serialize_entry("no_slider_head_accuracy", x)?;
            }
            if let Some(ref x) = self.classic_note_lock {
                map.serialize_entry("classic_note_lock", x)?;
            }
            if let Some(ref x) = self.always_play_tail_sample {
                map.serialize_entry("always_play_tail_sample", x)?;
            }
            if let Some(ref x) = self.fade_hit_circle_early {
                map.serialize_entry("fade_hit_circle_early", x)?;
            }
            if let Some(ref x) = self.classic_health {
                map.serialize_entry("classic_health", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<RandomOsu> {
        type Value = DeserializedGameMod<'de, RandomOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("RandomOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["angle_sharpness", "seed"];
            let mut unknown_key__ = None;
            let mut angle_sharpness = None;
            let mut seed = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "angle_sharpness" => angle_sharpness = Some(map.next_value()?),
                    "seed" => seed = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = RandomOsu {
                angle_sharpness: angle_sharpness.unwrap_or_default(),
                seed: seed.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for RandomOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count =
                self.angle_sharpness.is_some() as usize + self.seed.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.angle_sharpness {
                map.serialize_entry("angle_sharpness", x)?;
            }
            if let Some(ref x) = self.seed {
                map.serialize_entry("seed", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<MirrorOsu> {
        type Value = DeserializedGameMod<'de, MirrorOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("MirrorOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["reflection"];
            let mut unknown_key__ = None;
            let mut reflection = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "reflection" => reflection = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = MirrorOsu {
                reflection: reflection.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for MirrorOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.reflection.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.reflection {
                map.serialize_entry("reflection", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<AlternateOsu> {
        type Value = DeserializedGameMod<'de, AlternateOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("AlternateOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = AlternateOsu {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for AlternateOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<SingleTapOsu> {
        type Value = DeserializedGameMod<'de, SingleTapOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("SingleTapOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = SingleTapOsu {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for SingleTapOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<AutoplayOsu> {
        type Value = DeserializedGameMod<'de, AutoplayOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("AutoplayOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = AutoplayOsu {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for AutoplayOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<CinemaOsu> {
        type Value = DeserializedGameMod<'de, CinemaOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("CinemaOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = CinemaOsu {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for CinemaOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<RelaxOsu> {
        type Value = DeserializedGameMod<'de, RelaxOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("RelaxOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = RelaxOsu {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for RelaxOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<AutopilotOsu> {
        type Value = DeserializedGameMod<'de, AutopilotOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("AutopilotOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = AutopilotOsu {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for AutopilotOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<SpunOutOsu> {
        type Value = DeserializedGameMod<'de, SpunOutOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("SpunOutOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = SpunOutOsu {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for SpunOutOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<TransformOsu> {
        type Value = DeserializedGameMod<'de, TransformOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("TransformOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = TransformOsu {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for TransformOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<WiggleOsu> {
        type Value = DeserializedGameMod<'de, WiggleOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("WiggleOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["strength"];
            let mut unknown_key__ = None;
            let mut strength = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "strength" => strength = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = WiggleOsu {
                strength: strength.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for WiggleOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.strength.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.strength {
                map.serialize_entry("strength", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<SpinInOsu> {
        type Value = DeserializedGameMod<'de, SpinInOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("SpinInOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = SpinInOsu {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for SpinInOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<GrowOsu> {
        type Value = DeserializedGameMod<'de, GrowOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("GrowOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["start_scale"];
            let mut unknown_key__ = None;
            let mut start_scale = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "start_scale" => start_scale = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = GrowOsu {
                start_scale: start_scale.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for GrowOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.start_scale.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.start_scale {
                map.serialize_entry("start_scale", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<DeflateOsu> {
        type Value = DeserializedGameMod<'de, DeflateOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("DeflateOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["start_scale"];
            let mut unknown_key__ = None;
            let mut start_scale = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "start_scale" => start_scale = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = DeflateOsu {
                start_scale: start_scale.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for DeflateOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.start_scale.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.start_scale {
                map.serialize_entry("start_scale", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<WindUpOsu> {
        type Value = DeserializedGameMod<'de, WindUpOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("WindUpOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["initial_rate", "final_rate", "adjust_pitch"];
            let mut unknown_key__ = None;
            let mut initial_rate = None;
            let mut final_rate = None;
            let mut adjust_pitch = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "initial_rate" => initial_rate = Some(map.next_value()?),
                    "final_rate" => final_rate = Some(map.next_value()?),
                    "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = WindUpOsu {
                initial_rate: initial_rate.unwrap_or_default(),
                final_rate: final_rate.unwrap_or_default(),
                adjust_pitch: adjust_pitch.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for WindUpOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.initial_rate.is_some() as usize
                + self.final_rate.is_some() as usize
                + self.adjust_pitch.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.initial_rate {
                map.serialize_entry("initial_rate", x)?;
            }
            if let Some(ref x) = self.final_rate {
                map.serialize_entry("final_rate", x)?;
            }
            if let Some(ref x) = self.adjust_pitch {
                map.serialize_entry("adjust_pitch", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<WindDownOsu> {
        type Value = DeserializedGameMod<'de, WindDownOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("WindDownOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["initial_rate", "final_rate", "adjust_pitch"];
            let mut unknown_key__ = None;
            let mut initial_rate = None;
            let mut final_rate = None;
            let mut adjust_pitch = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "initial_rate" => initial_rate = Some(map.next_value()?),
                    "final_rate" => final_rate = Some(map.next_value()?),
                    "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = WindDownOsu {
                initial_rate: initial_rate.unwrap_or_default(),
                final_rate: final_rate.unwrap_or_default(),
                adjust_pitch: adjust_pitch.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for WindDownOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.initial_rate.is_some() as usize
                + self.final_rate.is_some() as usize
                + self.adjust_pitch.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.initial_rate {
                map.serialize_entry("initial_rate", x)?;
            }
            if let Some(ref x) = self.final_rate {
                map.serialize_entry("final_rate", x)?;
            }
            if let Some(ref x) = self.adjust_pitch {
                map.serialize_entry("adjust_pitch", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<TraceableOsu> {
        type Value = DeserializedGameMod<'de, TraceableOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("TraceableOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = TraceableOsu {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for TraceableOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<BarrelRollOsu> {
        type Value = DeserializedGameMod<'de, BarrelRollOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("BarrelRollOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["spin_speed", "direction"];
            let mut unknown_key__ = None;
            let mut spin_speed = None;
            let mut direction = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "spin_speed" => spin_speed = Some(map.next_value()?),
                    "direction" => direction = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = BarrelRollOsu {
                spin_speed: spin_speed.unwrap_or_default(),
                direction: direction.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for BarrelRollOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count =
                self.spin_speed.is_some() as usize + self.direction.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.spin_speed {
                map.serialize_entry("spin_speed", x)?;
            }
            if let Some(ref x) = self.direction {
                map.serialize_entry("direction", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<ApproachDifferentOsu> {
        type Value = DeserializedGameMod<'de, ApproachDifferentOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("ApproachDifferentOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["scale", "style"];
            let mut unknown_key__ = None;
            let mut scale = None;
            let mut style = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "scale" => scale = Some(map.next_value()?),
                    "style" => style = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = ApproachDifferentOsu {
                scale: scale.unwrap_or_default(),
                style: style.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for ApproachDifferentOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.scale.is_some() as usize + self.style.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.scale {
                map.serialize_entry("scale", x)?;
            }
            if let Some(ref x) = self.style {
                map.serialize_entry("style", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<MutedOsu> {
        type Value = DeserializedGameMod<'de, MutedOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("MutedOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[
                "inverse_muting",
                "enable_metronome",
                "mute_combo_count",
                "affects_hit_sounds",
            ];
            let mut unknown_key__ = None;
            let mut inverse_muting = None;
            let mut enable_metronome = None;
            let mut mute_combo_count = None;
            let mut affects_hit_sounds = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "inverse_muting" => inverse_muting = Some(map.next_value()?),
                    "enable_metronome" => enable_metronome = Some(map.next_value()?),
                    "mute_combo_count" => mute_combo_count = Some(map.next_value()?),
                    "affects_hit_sounds" => affects_hit_sounds = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = MutedOsu {
                inverse_muting: inverse_muting.unwrap_or_default(),
                enable_metronome: enable_metronome.unwrap_or_default(),
                mute_combo_count: mute_combo_count.unwrap_or_default(),
                affects_hit_sounds: affects_hit_sounds.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for MutedOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.inverse_muting.is_some() as usize
                + self.enable_metronome.is_some() as usize
                + self.mute_combo_count.is_some() as usize
                + self.affects_hit_sounds.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.inverse_muting {
                map.serialize_entry("inverse_muting", x)?;
            }
            if let Some(ref x) = self.enable_metronome {
                map.serialize_entry("enable_metronome", x)?;
            }
            if let Some(ref x) = self.mute_combo_count {
                map.serialize_entry("mute_combo_count", x)?;
            }
            if let Some(ref x) = self.affects_hit_sounds {
                map.serialize_entry("affects_hit_sounds", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<NoScopeOsu> {
        type Value = DeserializedGameMod<'de, NoScopeOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("NoScopeOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["hidden_combo_count"];
            let mut unknown_key__ = None;
            let mut hidden_combo_count = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "hidden_combo_count" => hidden_combo_count = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = NoScopeOsu {
                hidden_combo_count: hidden_combo_count.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for NoScopeOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.hidden_combo_count.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.hidden_combo_count {
                map.serialize_entry("hidden_combo_count", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<MagnetisedOsu> {
        type Value = DeserializedGameMod<'de, MagnetisedOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("MagnetisedOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["attraction_strength"];
            let mut unknown_key__ = None;
            let mut attraction_strength = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "attraction_strength" => attraction_strength = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = MagnetisedOsu {
                attraction_strength: attraction_strength.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for MagnetisedOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.attraction_strength.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.attraction_strength {
                map.serialize_entry("attraction_strength", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<RepelOsu> {
        type Value = DeserializedGameMod<'de, RepelOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("RepelOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["repulsion_strength"];
            let mut unknown_key__ = None;
            let mut repulsion_strength = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "repulsion_strength" => repulsion_strength = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = RepelOsu {
                repulsion_strength: repulsion_strength.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for RepelOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.repulsion_strength.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.repulsion_strength {
                map.serialize_entry("repulsion_strength", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<AdaptiveSpeedOsu> {
        type Value = DeserializedGameMod<'de, AdaptiveSpeedOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("AdaptiveSpeedOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["initial_rate", "adjust_pitch"];
            let mut unknown_key__ = None;
            let mut initial_rate = None;
            let mut adjust_pitch = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "initial_rate" => initial_rate = Some(map.next_value()?),
                    "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = AdaptiveSpeedOsu {
                initial_rate: initial_rate.unwrap_or_default(),
                adjust_pitch: adjust_pitch.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for AdaptiveSpeedOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count =
                self.initial_rate.is_some() as usize + self.adjust_pitch.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.initial_rate {
                map.serialize_entry("initial_rate", x)?;
            }
            if let Some(ref x) = self.adjust_pitch {
                map.serialize_entry("adjust_pitch", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<FreezeFrameOsu> {
        type Value = DeserializedGameMod<'de, FreezeFrameOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("FreezeFrameOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = FreezeFrameOsu {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for FreezeFrameOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<BubblesOsu> {
        type Value = DeserializedGameMod<'de, BubblesOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("BubblesOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = BubblesOsu {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for BubblesOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<SynesthesiaOsu> {
        type Value = DeserializedGameMod<'de, SynesthesiaOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("SynesthesiaOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = SynesthesiaOsu {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for SynesthesiaOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<DepthOsu> {
        type Value = DeserializedGameMod<'de, DepthOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("DepthOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["max_depth", "show_approach_circles"];
            let mut unknown_key__ = None;
            let mut max_depth = None;
            let mut show_approach_circles = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "max_depth" => max_depth = Some(map.next_value()?),
                    "show_approach_circles" => show_approach_circles = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = DepthOsu {
                max_depth: max_depth.unwrap_or_default(),
                show_approach_circles: show_approach_circles.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for DepthOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count =
                self.max_depth.is_some() as usize + self.show_approach_circles.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.max_depth {
                map.serialize_entry("max_depth", x)?;
            }
            if let Some(ref x) = self.show_approach_circles {
                map.serialize_entry("show_approach_circles", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<BloomOsu> {
        type Value = DeserializedGameMod<'de, BloomOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("BloomOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["max_size_combo_count", "max_cursor_size"];
            let mut unknown_key__ = None;
            let mut max_size_combo_count = None;
            let mut max_cursor_size = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "max_size_combo_count" => max_size_combo_count = Some(map.next_value()?),
                    "max_cursor_size" => max_cursor_size = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = BloomOsu {
                max_size_combo_count: max_size_combo_count.unwrap_or_default(),
                max_cursor_size: max_cursor_size.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for BloomOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.max_size_combo_count.is_some() as usize
                + self.max_cursor_size.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.max_size_combo_count {
                map.serialize_entry("max_size_combo_count", x)?;
            }
            if let Some(ref x) = self.max_cursor_size {
                map.serialize_entry("max_cursor_size", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<TouchDeviceOsu> {
        type Value = DeserializedGameMod<'de, TouchDeviceOsu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("TouchDeviceOsu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = TouchDeviceOsu {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for TouchDeviceOsu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<ScoreV2Osu> {
        type Value = DeserializedGameMod<'de, ScoreV2Osu>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("ScoreV2Osu")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = ScoreV2Osu {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for ScoreV2Osu {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<EasyTaiko> {
        type Value = DeserializedGameMod<'de, EasyTaiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("EasyTaiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = EasyTaiko {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for EasyTaiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<NoFailTaiko> {
        type Value = DeserializedGameMod<'de, NoFailTaiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("NoFailTaiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = NoFailTaiko {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for NoFailTaiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<HalfTimeTaiko> {
        type Value = DeserializedGameMod<'de, HalfTimeTaiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("HalfTimeTaiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["speed_change", "adjust_pitch"];
            let mut unknown_key__ = None;
            let mut speed_change = None;
            let mut adjust_pitch = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "speed_change" => speed_change = Some(map.next_value()?),
                    "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = HalfTimeTaiko {
                speed_change: speed_change.unwrap_or_default(),
                adjust_pitch: adjust_pitch.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for HalfTimeTaiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count =
                self.speed_change.is_some() as usize + self.adjust_pitch.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.speed_change {
                map.serialize_entry("speed_change", x)?;
            }
            if let Some(ref x) = self.adjust_pitch {
                map.serialize_entry("adjust_pitch", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<DaycoreTaiko> {
        type Value = DeserializedGameMod<'de, DaycoreTaiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("DaycoreTaiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["speed_change"];
            let mut unknown_key__ = None;
            let mut speed_change = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "speed_change" => speed_change = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = DaycoreTaiko {
                speed_change: speed_change.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for DaycoreTaiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.speed_change.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.speed_change {
                map.serialize_entry("speed_change", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<HardRockTaiko> {
        type Value = DeserializedGameMod<'de, HardRockTaiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("HardRockTaiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = HardRockTaiko {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for HardRockTaiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<SuddenDeathTaiko> {
        type Value = DeserializedGameMod<'de, SuddenDeathTaiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("SuddenDeathTaiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["restart"];
            let mut unknown_key__ = None;
            let mut restart = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "restart" => restart = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = SuddenDeathTaiko {
                restart: restart.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for SuddenDeathTaiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.restart.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.restart {
                map.serialize_entry("restart", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<PerfectTaiko> {
        type Value = DeserializedGameMod<'de, PerfectTaiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("PerfectTaiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["restart"];
            let mut unknown_key__ = None;
            let mut restart = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "restart" => restart = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = PerfectTaiko {
                restart: restart.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for PerfectTaiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.restart.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.restart {
                map.serialize_entry("restart", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<DoubleTimeTaiko> {
        type Value = DeserializedGameMod<'de, DoubleTimeTaiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("DoubleTimeTaiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["speed_change", "adjust_pitch"];
            let mut unknown_key__ = None;
            let mut speed_change = None;
            let mut adjust_pitch = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "speed_change" => speed_change = Some(map.next_value()?),
                    "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = DoubleTimeTaiko {
                speed_change: speed_change.unwrap_or_default(),
                adjust_pitch: adjust_pitch.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for DoubleTimeTaiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count =
                self.speed_change.is_some() as usize + self.adjust_pitch.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.speed_change {
                map.serialize_entry("speed_change", x)?;
            }
            if let Some(ref x) = self.adjust_pitch {
                map.serialize_entry("adjust_pitch", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<NightcoreTaiko> {
        type Value = DeserializedGameMod<'de, NightcoreTaiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("NightcoreTaiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["speed_change"];
            let mut unknown_key__ = None;
            let mut speed_change = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "speed_change" => speed_change = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = NightcoreTaiko {
                speed_change: speed_change.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for NightcoreTaiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.speed_change.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.speed_change {
                map.serialize_entry("speed_change", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<HiddenTaiko> {
        type Value = DeserializedGameMod<'de, HiddenTaiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("HiddenTaiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = HiddenTaiko {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for HiddenTaiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<FlashlightTaiko> {
        type Value = DeserializedGameMod<'de, FlashlightTaiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("FlashlightTaiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["size_multiplier", "combo_based_size"];
            let mut unknown_key__ = None;
            let mut size_multiplier = None;
            let mut combo_based_size = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "size_multiplier" => size_multiplier = Some(map.next_value()?),
                    "combo_based_size" => combo_based_size = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = FlashlightTaiko {
                size_multiplier: size_multiplier.unwrap_or_default(),
                combo_based_size: combo_based_size.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for FlashlightTaiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count =
                self.size_multiplier.is_some() as usize + self.combo_based_size.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.size_multiplier {
                map.serialize_entry("size_multiplier", x)?;
            }
            if let Some(ref x) = self.combo_based_size {
                map.serialize_entry("combo_based_size", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<AccuracyChallengeTaiko> {
        type Value = DeserializedGameMod<'de, AccuracyChallengeTaiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("AccuracyChallengeTaiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] =
                &["minimum_accuracy", "accuracy_judge_mode", "restart"];
            let mut unknown_key__ = None;
            let mut minimum_accuracy = None;
            let mut accuracy_judge_mode = None;
            let mut restart = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "minimum_accuracy" => minimum_accuracy = Some(map.next_value()?),
                    "accuracy_judge_mode" => accuracy_judge_mode = Some(map.next_value()?),
                    "restart" => restart = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = AccuracyChallengeTaiko {
                minimum_accuracy: minimum_accuracy.unwrap_or_default(),
                accuracy_judge_mode: accuracy_judge_mode.unwrap_or_default(),
                restart: restart.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for AccuracyChallengeTaiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.minimum_accuracy.is_some() as usize
                + self.accuracy_judge_mode.is_some() as usize
                + self.restart.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.minimum_accuracy {
                map.serialize_entry("minimum_accuracy", x)?;
            }
            if let Some(ref x) = self.accuracy_judge_mode {
                map.serialize_entry("accuracy_judge_mode", x)?;
            }
            if let Some(ref x) = self.restart {
                map.serialize_entry("restart", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<RandomTaiko> {
        type Value = DeserializedGameMod<'de, RandomTaiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("RandomTaiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["seed"];
            let mut unknown_key__ = None;
            let mut seed = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "seed" => seed = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = RandomTaiko {
                seed: seed.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for RandomTaiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.seed.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.seed {
                map.serialize_entry("seed", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<DifficultyAdjustTaiko> {
        type Value = DeserializedGameMod<'de, DifficultyAdjustTaiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("DifficultyAdjustTaiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[
                "scroll_speed",
                "drain_rate",
                "overall_difficulty",
                "extended_limits",
            ];
            let mut unknown_key__ = None;
            let mut scroll_speed = None;
            let mut drain_rate = None;
            let mut overall_difficulty = None;
            let mut extended_limits = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "scroll_speed" => scroll_speed = Some(map.next_value()?),
                    "drain_rate" => drain_rate = Some(map.next_value()?),
                    "overall_difficulty" => overall_difficulty = Some(map.next_value()?),
                    "extended_limits" => extended_limits = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = DifficultyAdjustTaiko {
                scroll_speed: scroll_speed.unwrap_or_default(),
                drain_rate: drain_rate.unwrap_or_default(),
                overall_difficulty: overall_difficulty.unwrap_or_default(),
                extended_limits: extended_limits.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for DifficultyAdjustTaiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.scroll_speed.is_some() as usize
                + self.drain_rate.is_some() as usize
                + self.overall_difficulty.is_some() as usize
                + self.extended_limits.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.scroll_speed {
                map.serialize_entry("scroll_speed", x)?;
            }
            if let Some(ref x) = self.drain_rate {
                map.serialize_entry("drain_rate", x)?;
            }
            if let Some(ref x) = self.overall_difficulty {
                map.serialize_entry("overall_difficulty", x)?;
            }
            if let Some(ref x) = self.extended_limits {
                map.serialize_entry("extended_limits", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<ClassicTaiko> {
        type Value = DeserializedGameMod<'de, ClassicTaiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("ClassicTaiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = ClassicTaiko {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for ClassicTaiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<SwapTaiko> {
        type Value = DeserializedGameMod<'de, SwapTaiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("SwapTaiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = SwapTaiko {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for SwapTaiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<SingleTapTaiko> {
        type Value = DeserializedGameMod<'de, SingleTapTaiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("SingleTapTaiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = SingleTapTaiko {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for SingleTapTaiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<ConstantSpeedTaiko> {
        type Value = DeserializedGameMod<'de, ConstantSpeedTaiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("ConstantSpeedTaiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = ConstantSpeedTaiko {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for ConstantSpeedTaiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<AutoplayTaiko> {
        type Value = DeserializedGameMod<'de, AutoplayTaiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("AutoplayTaiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = AutoplayTaiko {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for AutoplayTaiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<CinemaTaiko> {
        type Value = DeserializedGameMod<'de, CinemaTaiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("CinemaTaiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = CinemaTaiko {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for CinemaTaiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<RelaxTaiko> {
        type Value = DeserializedGameMod<'de, RelaxTaiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("RelaxTaiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = RelaxTaiko {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for RelaxTaiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<WindUpTaiko> {
        type Value = DeserializedGameMod<'de, WindUpTaiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("WindUpTaiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["initial_rate", "final_rate", "adjust_pitch"];
            let mut unknown_key__ = None;
            let mut initial_rate = None;
            let mut final_rate = None;
            let mut adjust_pitch = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "initial_rate" => initial_rate = Some(map.next_value()?),
                    "final_rate" => final_rate = Some(map.next_value()?),
                    "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = WindUpTaiko {
                initial_rate: initial_rate.unwrap_or_default(),
                final_rate: final_rate.unwrap_or_default(),
                adjust_pitch: adjust_pitch.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for WindUpTaiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.initial_rate.is_some() as usize
                + self.final_rate.is_some() as usize
                + self.adjust_pitch.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.initial_rate {
                map.serialize_entry("initial_rate", x)?;
            }
            if let Some(ref x) = self.final_rate {
                map.serialize_entry("final_rate", x)?;
            }
            if let Some(ref x) = self.adjust_pitch {
                map.serialize_entry("adjust_pitch", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<WindDownTaiko> {
        type Value = DeserializedGameMod<'de, WindDownTaiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("WindDownTaiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["initial_rate", "final_rate", "adjust_pitch"];
            let mut unknown_key__ = None;
            let mut initial_rate = None;
            let mut final_rate = None;
            let mut adjust_pitch = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "initial_rate" => initial_rate = Some(map.next_value()?),
                    "final_rate" => final_rate = Some(map.next_value()?),
                    "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = WindDownTaiko {
                initial_rate: initial_rate.unwrap_or_default(),
                final_rate: final_rate.unwrap_or_default(),
                adjust_pitch: adjust_pitch.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for WindDownTaiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.initial_rate.is_some() as usize
                + self.final_rate.is_some() as usize
                + self.adjust_pitch.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.initial_rate {
                map.serialize_entry("initial_rate", x)?;
            }
            if let Some(ref x) = self.final_rate {
                map.serialize_entry("final_rate", x)?;
            }
            if let Some(ref x) = self.adjust_pitch {
                map.serialize_entry("adjust_pitch", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<MutedTaiko> {
        type Value = DeserializedGameMod<'de, MutedTaiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("MutedTaiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[
                "inverse_muting",
                "enable_metronome",
                "mute_combo_count",
                "affects_hit_sounds",
            ];
            let mut unknown_key__ = None;
            let mut inverse_muting = None;
            let mut enable_metronome = None;
            let mut mute_combo_count = None;
            let mut affects_hit_sounds = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "inverse_muting" => inverse_muting = Some(map.next_value()?),
                    "enable_metronome" => enable_metronome = Some(map.next_value()?),
                    "mute_combo_count" => mute_combo_count = Some(map.next_value()?),
                    "affects_hit_sounds" => affects_hit_sounds = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = MutedTaiko {
                inverse_muting: inverse_muting.unwrap_or_default(),
                enable_metronome: enable_metronome.unwrap_or_default(),
                mute_combo_count: mute_combo_count.unwrap_or_default(),
                affects_hit_sounds: affects_hit_sounds.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for MutedTaiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.inverse_muting.is_some() as usize
                + self.enable_metronome.is_some() as usize
                + self.mute_combo_count.is_some() as usize
                + self.affects_hit_sounds.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.inverse_muting {
                map.serialize_entry("inverse_muting", x)?;
            }
            if let Some(ref x) = self.enable_metronome {
                map.serialize_entry("enable_metronome", x)?;
            }
            if let Some(ref x) = self.mute_combo_count {
                map.serialize_entry("mute_combo_count", x)?;
            }
            if let Some(ref x) = self.affects_hit_sounds {
                map.serialize_entry("affects_hit_sounds", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<AdaptiveSpeedTaiko> {
        type Value = DeserializedGameMod<'de, AdaptiveSpeedTaiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("AdaptiveSpeedTaiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["initial_rate", "adjust_pitch"];
            let mut unknown_key__ = None;
            let mut initial_rate = None;
            let mut adjust_pitch = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "initial_rate" => initial_rate = Some(map.next_value()?),
                    "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = AdaptiveSpeedTaiko {
                initial_rate: initial_rate.unwrap_or_default(),
                adjust_pitch: adjust_pitch.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for AdaptiveSpeedTaiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count =
                self.initial_rate.is_some() as usize + self.adjust_pitch.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.initial_rate {
                map.serialize_entry("initial_rate", x)?;
            }
            if let Some(ref x) = self.adjust_pitch {
                map.serialize_entry("adjust_pitch", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<ScoreV2Taiko> {
        type Value = DeserializedGameMod<'de, ScoreV2Taiko>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("ScoreV2Taiko")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = ScoreV2Taiko {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for ScoreV2Taiko {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<EasyCatch> {
        type Value = DeserializedGameMod<'de, EasyCatch>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("EasyCatch")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["retries"];
            let mut unknown_key__ = None;
            let mut retries = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "retries" => retries = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = EasyCatch {
                retries: retries.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for EasyCatch {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.retries.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.retries {
                map.serialize_entry("retries", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<NoFailCatch> {
        type Value = DeserializedGameMod<'de, NoFailCatch>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("NoFailCatch")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = NoFailCatch {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for NoFailCatch {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<HalfTimeCatch> {
        type Value = DeserializedGameMod<'de, HalfTimeCatch>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("HalfTimeCatch")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["speed_change", "adjust_pitch"];
            let mut unknown_key__ = None;
            let mut speed_change = None;
            let mut adjust_pitch = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "speed_change" => speed_change = Some(map.next_value()?),
                    "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = HalfTimeCatch {
                speed_change: speed_change.unwrap_or_default(),
                adjust_pitch: adjust_pitch.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for HalfTimeCatch {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count =
                self.speed_change.is_some() as usize + self.adjust_pitch.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.speed_change {
                map.serialize_entry("speed_change", x)?;
            }
            if let Some(ref x) = self.adjust_pitch {
                map.serialize_entry("adjust_pitch", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<DaycoreCatch> {
        type Value = DeserializedGameMod<'de, DaycoreCatch>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("DaycoreCatch")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["speed_change"];
            let mut unknown_key__ = None;
            let mut speed_change = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "speed_change" => speed_change = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = DaycoreCatch {
                speed_change: speed_change.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for DaycoreCatch {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.speed_change.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.speed_change {
                map.serialize_entry("speed_change", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<HardRockCatch> {
        type Value = DeserializedGameMod<'de, HardRockCatch>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("HardRockCatch")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = HardRockCatch {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for HardRockCatch {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<SuddenDeathCatch> {
        type Value = DeserializedGameMod<'de, SuddenDeathCatch>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("SuddenDeathCatch")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["restart"];
            let mut unknown_key__ = None;
            let mut restart = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "restart" => restart = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = SuddenDeathCatch {
                restart: restart.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for SuddenDeathCatch {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.restart.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.restart {
                map.serialize_entry("restart", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<PerfectCatch> {
        type Value = DeserializedGameMod<'de, PerfectCatch>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("PerfectCatch")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["restart"];
            let mut unknown_key__ = None;
            let mut restart = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "restart" => restart = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = PerfectCatch {
                restart: restart.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for PerfectCatch {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.restart.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.restart {
                map.serialize_entry("restart", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<DoubleTimeCatch> {
        type Value = DeserializedGameMod<'de, DoubleTimeCatch>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("DoubleTimeCatch")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["speed_change", "adjust_pitch"];
            let mut unknown_key__ = None;
            let mut speed_change = None;
            let mut adjust_pitch = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "speed_change" => speed_change = Some(map.next_value()?),
                    "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = DoubleTimeCatch {
                speed_change: speed_change.unwrap_or_default(),
                adjust_pitch: adjust_pitch.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for DoubleTimeCatch {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count =
                self.speed_change.is_some() as usize + self.adjust_pitch.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.speed_change {
                map.serialize_entry("speed_change", x)?;
            }
            if let Some(ref x) = self.adjust_pitch {
                map.serialize_entry("adjust_pitch", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<NightcoreCatch> {
        type Value = DeserializedGameMod<'de, NightcoreCatch>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("NightcoreCatch")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["speed_change"];
            let mut unknown_key__ = None;
            let mut speed_change = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "speed_change" => speed_change = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = NightcoreCatch {
                speed_change: speed_change.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for NightcoreCatch {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.speed_change.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.speed_change {
                map.serialize_entry("speed_change", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<HiddenCatch> {
        type Value = DeserializedGameMod<'de, HiddenCatch>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("HiddenCatch")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = HiddenCatch {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for HiddenCatch {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<FlashlightCatch> {
        type Value = DeserializedGameMod<'de, FlashlightCatch>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("FlashlightCatch")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["size_multiplier", "combo_based_size"];
            let mut unknown_key__ = None;
            let mut size_multiplier = None;
            let mut combo_based_size = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "size_multiplier" => size_multiplier = Some(map.next_value()?),
                    "combo_based_size" => combo_based_size = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = FlashlightCatch {
                size_multiplier: size_multiplier.unwrap_or_default(),
                combo_based_size: combo_based_size.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for FlashlightCatch {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count =
                self.size_multiplier.is_some() as usize + self.combo_based_size.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.size_multiplier {
                map.serialize_entry("size_multiplier", x)?;
            }
            if let Some(ref x) = self.combo_based_size {
                map.serialize_entry("combo_based_size", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<AccuracyChallengeCatch> {
        type Value = DeserializedGameMod<'de, AccuracyChallengeCatch>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("AccuracyChallengeCatch")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] =
                &["minimum_accuracy", "accuracy_judge_mode", "restart"];
            let mut unknown_key__ = None;
            let mut minimum_accuracy = None;
            let mut accuracy_judge_mode = None;
            let mut restart = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "minimum_accuracy" => minimum_accuracy = Some(map.next_value()?),
                    "accuracy_judge_mode" => accuracy_judge_mode = Some(map.next_value()?),
                    "restart" => restart = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = AccuracyChallengeCatch {
                minimum_accuracy: minimum_accuracy.unwrap_or_default(),
                accuracy_judge_mode: accuracy_judge_mode.unwrap_or_default(),
                restart: restart.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for AccuracyChallengeCatch {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.minimum_accuracy.is_some() as usize
                + self.accuracy_judge_mode.is_some() as usize
                + self.restart.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.minimum_accuracy {
                map.serialize_entry("minimum_accuracy", x)?;
            }
            if let Some(ref x) = self.accuracy_judge_mode {
                map.serialize_entry("accuracy_judge_mode", x)?;
            }
            if let Some(ref x) = self.restart {
                map.serialize_entry("restart", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<DifficultyAdjustCatch> {
        type Value = DeserializedGameMod<'de, DifficultyAdjustCatch>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("DifficultyAdjustCatch")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[
                "circle_size",
                "approach_rate",
                "hard_rock_offsets",
                "drain_rate",
                "overall_difficulty",
                "extended_limits",
            ];
            let mut unknown_key__ = None;
            let mut circle_size = None;
            let mut approach_rate = None;
            let mut hard_rock_offsets = None;
            let mut drain_rate = None;
            let mut overall_difficulty = None;
            let mut extended_limits = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "circle_size" => circle_size = Some(map.next_value()?),
                    "approach_rate" => approach_rate = Some(map.next_value()?),
                    "hard_rock_offsets" => hard_rock_offsets = Some(map.next_value()?),
                    "drain_rate" => drain_rate = Some(map.next_value()?),
                    "overall_difficulty" => overall_difficulty = Some(map.next_value()?),
                    "extended_limits" => extended_limits = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = DifficultyAdjustCatch {
                circle_size: circle_size.unwrap_or_default(),
                approach_rate: approach_rate.unwrap_or_default(),
                hard_rock_offsets: hard_rock_offsets.unwrap_or_default(),
                drain_rate: drain_rate.unwrap_or_default(),
                overall_difficulty: overall_difficulty.unwrap_or_default(),
                extended_limits: extended_limits.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for DifficultyAdjustCatch {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.circle_size.is_some() as usize
                + self.approach_rate.is_some() as usize
                + self.hard_rock_offsets.is_some() as usize
                + self.drain_rate.is_some() as usize
                + self.overall_difficulty.is_some() as usize
                + self.extended_limits.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.circle_size {
                map.serialize_entry("circle_size", x)?;
            }
            if let Some(ref x) = self.approach_rate {
                map.serialize_entry("approach_rate", x)?;
            }
            if let Some(ref x) = self.hard_rock_offsets {
                map.serialize_entry("hard_rock_offsets", x)?;
            }
            if let Some(ref x) = self.drain_rate {
                map.serialize_entry("drain_rate", x)?;
            }
            if let Some(ref x) = self.overall_difficulty {
                map.serialize_entry("overall_difficulty", x)?;
            }
            if let Some(ref x) = self.extended_limits {
                map.serialize_entry("extended_limits", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<ClassicCatch> {
        type Value = DeserializedGameMod<'de, ClassicCatch>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("ClassicCatch")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = ClassicCatch {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for ClassicCatch {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<MirrorCatch> {
        type Value = DeserializedGameMod<'de, MirrorCatch>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("MirrorCatch")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = MirrorCatch {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for MirrorCatch {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<AutoplayCatch> {
        type Value = DeserializedGameMod<'de, AutoplayCatch>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("AutoplayCatch")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = AutoplayCatch {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for AutoplayCatch {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<CinemaCatch> {
        type Value = DeserializedGameMod<'de, CinemaCatch>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("CinemaCatch")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = CinemaCatch {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for CinemaCatch {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<RelaxCatch> {
        type Value = DeserializedGameMod<'de, RelaxCatch>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("RelaxCatch")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = RelaxCatch {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for RelaxCatch {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<WindUpCatch> {
        type Value = DeserializedGameMod<'de, WindUpCatch>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("WindUpCatch")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["initial_rate", "final_rate", "adjust_pitch"];
            let mut unknown_key__ = None;
            let mut initial_rate = None;
            let mut final_rate = None;
            let mut adjust_pitch = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "initial_rate" => initial_rate = Some(map.next_value()?),
                    "final_rate" => final_rate = Some(map.next_value()?),
                    "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = WindUpCatch {
                initial_rate: initial_rate.unwrap_or_default(),
                final_rate: final_rate.unwrap_or_default(),
                adjust_pitch: adjust_pitch.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for WindUpCatch {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.initial_rate.is_some() as usize
                + self.final_rate.is_some() as usize
                + self.adjust_pitch.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.initial_rate {
                map.serialize_entry("initial_rate", x)?;
            }
            if let Some(ref x) = self.final_rate {
                map.serialize_entry("final_rate", x)?;
            }
            if let Some(ref x) = self.adjust_pitch {
                map.serialize_entry("adjust_pitch", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<WindDownCatch> {
        type Value = DeserializedGameMod<'de, WindDownCatch>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("WindDownCatch")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["initial_rate", "final_rate", "adjust_pitch"];
            let mut unknown_key__ = None;
            let mut initial_rate = None;
            let mut final_rate = None;
            let mut adjust_pitch = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "initial_rate" => initial_rate = Some(map.next_value()?),
                    "final_rate" => final_rate = Some(map.next_value()?),
                    "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = WindDownCatch {
                initial_rate: initial_rate.unwrap_or_default(),
                final_rate: final_rate.unwrap_or_default(),
                adjust_pitch: adjust_pitch.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for WindDownCatch {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.initial_rate.is_some() as usize
                + self.final_rate.is_some() as usize
                + self.adjust_pitch.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.initial_rate {
                map.serialize_entry("initial_rate", x)?;
            }
            if let Some(ref x) = self.final_rate {
                map.serialize_entry("final_rate", x)?;
            }
            if let Some(ref x) = self.adjust_pitch {
                map.serialize_entry("adjust_pitch", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<FloatingFruitsCatch> {
        type Value = DeserializedGameMod<'de, FloatingFruitsCatch>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("FloatingFruitsCatch")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = FloatingFruitsCatch {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for FloatingFruitsCatch {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<MutedCatch> {
        type Value = DeserializedGameMod<'de, MutedCatch>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("MutedCatch")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[
                "inverse_muting",
                "enable_metronome",
                "mute_combo_count",
                "affects_hit_sounds",
            ];
            let mut unknown_key__ = None;
            let mut inverse_muting = None;
            let mut enable_metronome = None;
            let mut mute_combo_count = None;
            let mut affects_hit_sounds = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "inverse_muting" => inverse_muting = Some(map.next_value()?),
                    "enable_metronome" => enable_metronome = Some(map.next_value()?),
                    "mute_combo_count" => mute_combo_count = Some(map.next_value()?),
                    "affects_hit_sounds" => affects_hit_sounds = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = MutedCatch {
                inverse_muting: inverse_muting.unwrap_or_default(),
                enable_metronome: enable_metronome.unwrap_or_default(),
                mute_combo_count: mute_combo_count.unwrap_or_default(),
                affects_hit_sounds: affects_hit_sounds.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for MutedCatch {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.inverse_muting.is_some() as usize
                + self.enable_metronome.is_some() as usize
                + self.mute_combo_count.is_some() as usize
                + self.affects_hit_sounds.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.inverse_muting {
                map.serialize_entry("inverse_muting", x)?;
            }
            if let Some(ref x) = self.enable_metronome {
                map.serialize_entry("enable_metronome", x)?;
            }
            if let Some(ref x) = self.mute_combo_count {
                map.serialize_entry("mute_combo_count", x)?;
            }
            if let Some(ref x) = self.affects_hit_sounds {
                map.serialize_entry("affects_hit_sounds", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<NoScopeCatch> {
        type Value = DeserializedGameMod<'de, NoScopeCatch>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("NoScopeCatch")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["hidden_combo_count"];
            let mut unknown_key__ = None;
            let mut hidden_combo_count = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "hidden_combo_count" => hidden_combo_count = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = NoScopeCatch {
                hidden_combo_count: hidden_combo_count.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for NoScopeCatch {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.hidden_combo_count.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.hidden_combo_count {
                map.serialize_entry("hidden_combo_count", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<ScoreV2Catch> {
        type Value = DeserializedGameMod<'de, ScoreV2Catch>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("ScoreV2Catch")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = ScoreV2Catch {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for ScoreV2Catch {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<EasyMania> {
        type Value = DeserializedGameMod<'de, EasyMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("EasyMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["retries"];
            let mut unknown_key__ = None;
            let mut retries = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "retries" => retries = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = EasyMania {
                retries: retries.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for EasyMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.retries.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.retries {
                map.serialize_entry("retries", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<NoFailMania> {
        type Value = DeserializedGameMod<'de, NoFailMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("NoFailMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = NoFailMania {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for NoFailMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<HalfTimeMania> {
        type Value = DeserializedGameMod<'de, HalfTimeMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("HalfTimeMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["speed_change", "adjust_pitch"];
            let mut unknown_key__ = None;
            let mut speed_change = None;
            let mut adjust_pitch = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "speed_change" => speed_change = Some(map.next_value()?),
                    "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = HalfTimeMania {
                speed_change: speed_change.unwrap_or_default(),
                adjust_pitch: adjust_pitch.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for HalfTimeMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count =
                self.speed_change.is_some() as usize + self.adjust_pitch.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.speed_change {
                map.serialize_entry("speed_change", x)?;
            }
            if let Some(ref x) = self.adjust_pitch {
                map.serialize_entry("adjust_pitch", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<DaycoreMania> {
        type Value = DeserializedGameMod<'de, DaycoreMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("DaycoreMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["speed_change"];
            let mut unknown_key__ = None;
            let mut speed_change = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "speed_change" => speed_change = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = DaycoreMania {
                speed_change: speed_change.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for DaycoreMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.speed_change.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.speed_change {
                map.serialize_entry("speed_change", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<NoReleaseMania> {
        type Value = DeserializedGameMod<'de, NoReleaseMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("NoReleaseMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = NoReleaseMania {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for NoReleaseMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<HardRockMania> {
        type Value = DeserializedGameMod<'de, HardRockMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("HardRockMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = HardRockMania {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for HardRockMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<SuddenDeathMania> {
        type Value = DeserializedGameMod<'de, SuddenDeathMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("SuddenDeathMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["restart"];
            let mut unknown_key__ = None;
            let mut restart = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "restart" => restart = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = SuddenDeathMania {
                restart: restart.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for SuddenDeathMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.restart.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.restart {
                map.serialize_entry("restart", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<PerfectMania> {
        type Value = DeserializedGameMod<'de, PerfectMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("PerfectMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["restart"];
            let mut unknown_key__ = None;
            let mut restart = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "restart" => restart = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = PerfectMania {
                restart: restart.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for PerfectMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.restart.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.restart {
                map.serialize_entry("restart", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<DoubleTimeMania> {
        type Value = DeserializedGameMod<'de, DoubleTimeMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("DoubleTimeMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["speed_change", "adjust_pitch"];
            let mut unknown_key__ = None;
            let mut speed_change = None;
            let mut adjust_pitch = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "speed_change" => speed_change = Some(map.next_value()?),
                    "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = DoubleTimeMania {
                speed_change: speed_change.unwrap_or_default(),
                adjust_pitch: adjust_pitch.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for DoubleTimeMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count =
                self.speed_change.is_some() as usize + self.adjust_pitch.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.speed_change {
                map.serialize_entry("speed_change", x)?;
            }
            if let Some(ref x) = self.adjust_pitch {
                map.serialize_entry("adjust_pitch", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<NightcoreMania> {
        type Value = DeserializedGameMod<'de, NightcoreMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("NightcoreMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["speed_change"];
            let mut unknown_key__ = None;
            let mut speed_change = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "speed_change" => speed_change = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = NightcoreMania {
                speed_change: speed_change.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for NightcoreMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.speed_change.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.speed_change {
                map.serialize_entry("speed_change", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<FadeInMania> {
        type Value = DeserializedGameMod<'de, FadeInMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("FadeInMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = FadeInMania {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for FadeInMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<HiddenMania> {
        type Value = DeserializedGameMod<'de, HiddenMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("HiddenMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = HiddenMania {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for HiddenMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<CoverMania> {
        type Value = DeserializedGameMod<'de, CoverMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("CoverMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["coverage", "direction"];
            let mut unknown_key__ = None;
            let mut coverage = None;
            let mut direction = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "coverage" => coverage = Some(map.next_value()?),
                    "direction" => direction = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = CoverMania {
                coverage: coverage.unwrap_or_default(),
                direction: direction.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for CoverMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.coverage.is_some() as usize + self.direction.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.coverage {
                map.serialize_entry("coverage", x)?;
            }
            if let Some(ref x) = self.direction {
                map.serialize_entry("direction", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<FlashlightMania> {
        type Value = DeserializedGameMod<'de, FlashlightMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("FlashlightMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["size_multiplier", "combo_based_size"];
            let mut unknown_key__ = None;
            let mut size_multiplier = None;
            let mut combo_based_size = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "size_multiplier" => size_multiplier = Some(map.next_value()?),
                    "combo_based_size" => combo_based_size = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = FlashlightMania {
                size_multiplier: size_multiplier.unwrap_or_default(),
                combo_based_size: combo_based_size.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for FlashlightMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count =
                self.size_multiplier.is_some() as usize + self.combo_based_size.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.size_multiplier {
                map.serialize_entry("size_multiplier", x)?;
            }
            if let Some(ref x) = self.combo_based_size {
                map.serialize_entry("combo_based_size", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<AccuracyChallengeMania> {
        type Value = DeserializedGameMod<'de, AccuracyChallengeMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("AccuracyChallengeMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] =
                &["minimum_accuracy", "accuracy_judge_mode", "restart"];
            let mut unknown_key__ = None;
            let mut minimum_accuracy = None;
            let mut accuracy_judge_mode = None;
            let mut restart = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "minimum_accuracy" => minimum_accuracy = Some(map.next_value()?),
                    "accuracy_judge_mode" => accuracy_judge_mode = Some(map.next_value()?),
                    "restart" => restart = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = AccuracyChallengeMania {
                minimum_accuracy: minimum_accuracy.unwrap_or_default(),
                accuracy_judge_mode: accuracy_judge_mode.unwrap_or_default(),
                restart: restart.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for AccuracyChallengeMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.minimum_accuracy.is_some() as usize
                + self.accuracy_judge_mode.is_some() as usize
                + self.restart.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.minimum_accuracy {
                map.serialize_entry("minimum_accuracy", x)?;
            }
            if let Some(ref x) = self.accuracy_judge_mode {
                map.serialize_entry("accuracy_judge_mode", x)?;
            }
            if let Some(ref x) = self.restart {
                map.serialize_entry("restart", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<RandomMania> {
        type Value = DeserializedGameMod<'de, RandomMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("RandomMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["seed"];
            let mut unknown_key__ = None;
            let mut seed = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "seed" => seed = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = RandomMania {
                seed: seed.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for RandomMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.seed.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.seed {
                map.serialize_entry("seed", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<DualStagesMania> {
        type Value = DeserializedGameMod<'de, DualStagesMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("DualStagesMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = DualStagesMania {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for DualStagesMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<MirrorMania> {
        type Value = DeserializedGameMod<'de, MirrorMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("MirrorMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = MirrorMania {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for MirrorMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<DifficultyAdjustMania> {
        type Value = DeserializedGameMod<'de, DifficultyAdjustMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("DifficultyAdjustMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] =
                &["drain_rate", "overall_difficulty", "extended_limits"];
            let mut unknown_key__ = None;
            let mut drain_rate = None;
            let mut overall_difficulty = None;
            let mut extended_limits = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "drain_rate" => drain_rate = Some(map.next_value()?),
                    "overall_difficulty" => overall_difficulty = Some(map.next_value()?),
                    "extended_limits" => extended_limits = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = DifficultyAdjustMania {
                drain_rate: drain_rate.unwrap_or_default(),
                overall_difficulty: overall_difficulty.unwrap_or_default(),
                extended_limits: extended_limits.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for DifficultyAdjustMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.drain_rate.is_some() as usize
                + self.overall_difficulty.is_some() as usize
                + self.extended_limits.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.drain_rate {
                map.serialize_entry("drain_rate", x)?;
            }
            if let Some(ref x) = self.overall_difficulty {
                map.serialize_entry("overall_difficulty", x)?;
            }
            if let Some(ref x) = self.extended_limits {
                map.serialize_entry("extended_limits", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<ClassicMania> {
        type Value = DeserializedGameMod<'de, ClassicMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("ClassicMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = ClassicMania {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for ClassicMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<InvertMania> {
        type Value = DeserializedGameMod<'de, InvertMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("InvertMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = InvertMania {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for InvertMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<ConstantSpeedMania> {
        type Value = DeserializedGameMod<'de, ConstantSpeedMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("ConstantSpeedMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = ConstantSpeedMania {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for ConstantSpeedMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<HoldOffMania> {
        type Value = DeserializedGameMod<'de, HoldOffMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("HoldOffMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = HoldOffMania {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for HoldOffMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<OneKeyMania> {
        type Value = DeserializedGameMod<'de, OneKeyMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("OneKeyMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = OneKeyMania {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for OneKeyMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<TwoKeysMania> {
        type Value = DeserializedGameMod<'de, TwoKeysMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("TwoKeysMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = TwoKeysMania {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for TwoKeysMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<ThreeKeysMania> {
        type Value = DeserializedGameMod<'de, ThreeKeysMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("ThreeKeysMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = ThreeKeysMania {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for ThreeKeysMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<FourKeysMania> {
        type Value = DeserializedGameMod<'de, FourKeysMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("FourKeysMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = FourKeysMania {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for FourKeysMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<FiveKeysMania> {
        type Value = DeserializedGameMod<'de, FiveKeysMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("FiveKeysMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = FiveKeysMania {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for FiveKeysMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<SixKeysMania> {
        type Value = DeserializedGameMod<'de, SixKeysMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("SixKeysMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = SixKeysMania {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for SixKeysMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<SevenKeysMania> {
        type Value = DeserializedGameMod<'de, SevenKeysMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("SevenKeysMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = SevenKeysMania {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for SevenKeysMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<EightKeysMania> {
        type Value = DeserializedGameMod<'de, EightKeysMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("EightKeysMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = EightKeysMania {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for EightKeysMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<NineKeysMania> {
        type Value = DeserializedGameMod<'de, NineKeysMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("NineKeysMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = NineKeysMania {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for NineKeysMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<TenKeysMania> {
        type Value = DeserializedGameMod<'de, TenKeysMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("TenKeysMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = TenKeysMania {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for TenKeysMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<AutoplayMania> {
        type Value = DeserializedGameMod<'de, AutoplayMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("AutoplayMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = AutoplayMania {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for AutoplayMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<CinemaMania> {
        type Value = DeserializedGameMod<'de, CinemaMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("CinemaMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = CinemaMania {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for CinemaMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<WindUpMania> {
        type Value = DeserializedGameMod<'de, WindUpMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("WindUpMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["initial_rate", "final_rate", "adjust_pitch"];
            let mut unknown_key__ = None;
            let mut initial_rate = None;
            let mut final_rate = None;
            let mut adjust_pitch = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "initial_rate" => initial_rate = Some(map.next_value()?),
                    "final_rate" => final_rate = Some(map.next_value()?),
                    "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = WindUpMania {
                initial_rate: initial_rate.unwrap_or_default(),
                final_rate: final_rate.unwrap_or_default(),
                adjust_pitch: adjust_pitch.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for WindUpMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.initial_rate.is_some() as usize
                + self.final_rate.is_some() as usize
                + self.adjust_pitch.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.initial_rate {
                map.serialize_entry("initial_rate", x)?;
            }
            if let Some(ref x) = self.final_rate {
                map.serialize_entry("final_rate", x)?;
            }
            if let Some(ref x) = self.adjust_pitch {
                map.serialize_entry("adjust_pitch", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<WindDownMania> {
        type Value = DeserializedGameMod<'de, WindDownMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("WindDownMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["initial_rate", "final_rate", "adjust_pitch"];
            let mut unknown_key__ = None;
            let mut initial_rate = None;
            let mut final_rate = None;
            let mut adjust_pitch = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "initial_rate" => initial_rate = Some(map.next_value()?),
                    "final_rate" => final_rate = Some(map.next_value()?),
                    "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = WindDownMania {
                initial_rate: initial_rate.unwrap_or_default(),
                final_rate: final_rate.unwrap_or_default(),
                adjust_pitch: adjust_pitch.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for WindDownMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.initial_rate.is_some() as usize
                + self.final_rate.is_some() as usize
                + self.adjust_pitch.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.initial_rate {
                map.serialize_entry("initial_rate", x)?;
            }
            if let Some(ref x) = self.final_rate {
                map.serialize_entry("final_rate", x)?;
            }
            if let Some(ref x) = self.adjust_pitch {
                map.serialize_entry("adjust_pitch", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<MutedMania> {
        type Value = DeserializedGameMod<'de, MutedMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("MutedMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[
                "inverse_muting",
                "enable_metronome",
                "mute_combo_count",
                "affects_hit_sounds",
            ];
            let mut unknown_key__ = None;
            let mut inverse_muting = None;
            let mut enable_metronome = None;
            let mut mute_combo_count = None;
            let mut affects_hit_sounds = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "inverse_muting" => inverse_muting = Some(map.next_value()?),
                    "enable_metronome" => enable_metronome = Some(map.next_value()?),
                    "mute_combo_count" => mute_combo_count = Some(map.next_value()?),
                    "affects_hit_sounds" => affects_hit_sounds = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = MutedMania {
                inverse_muting: inverse_muting.unwrap_or_default(),
                enable_metronome: enable_metronome.unwrap_or_default(),
                mute_combo_count: mute_combo_count.unwrap_or_default(),
                affects_hit_sounds: affects_hit_sounds.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for MutedMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = self.inverse_muting.is_some() as usize
                + self.enable_metronome.is_some() as usize
                + self.mute_combo_count.is_some() as usize
                + self.affects_hit_sounds.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.inverse_muting {
                map.serialize_entry("inverse_muting", x)?;
            }
            if let Some(ref x) = self.enable_metronome {
                map.serialize_entry("enable_metronome", x)?;
            }
            if let Some(ref x) = self.mute_combo_count {
                map.serialize_entry("mute_combo_count", x)?;
            }
            if let Some(ref x) = self.affects_hit_sounds {
                map.serialize_entry("affects_hit_sounds", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<AdaptiveSpeedMania> {
        type Value = DeserializedGameMod<'de, AdaptiveSpeedMania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("AdaptiveSpeedMania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &["initial_rate", "adjust_pitch"];
            let mut unknown_key__ = None;
            let mut initial_rate = None;
            let mut adjust_pitch = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    "initial_rate" => initial_rate = Some(map.next_value()?),
                    "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = AdaptiveSpeedMania {
                initial_rate: initial_rate.unwrap_or_default(),
                adjust_pitch: adjust_pitch.unwrap_or_default(),
            };
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for AdaptiveSpeedMania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count =
                self.initial_rate.is_some() as usize + self.adjust_pitch.is_some() as usize;
            let mut map = s.serialize_map(Some(field_count))?;
            if let Some(ref x) = self.initial_rate {
                map.serialize_entry("initial_rate", x)?;
            }
            if let Some(ref x) = self.adjust_pitch {
                map.serialize_entry("adjust_pitch", x)?;
            }
            map.end()
        }
    }
    impl<'de> Visitor<'de> for GameModVisitor<ScoreV2Mania> {
        type Value = DeserializedGameMod<'de, ScoreV2Mania>;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("ScoreV2Mania")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            const FIELDS: &'static [&'static str] = &[];
            let mut unknown_key__ = None;
            while let Some(key) = map.next_key::<MaybeOwnedStr<'de>>()? {
                match key.as_str() {
                    _ => {
                        unknown_key__ = Some(key);
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }
            let gamemod = ScoreV2Mania {};
            Ok(DeserializedGameMod::new(gamemod, unknown_key__, FIELDS))
        }
    }
    impl Serialize for ScoreV2Mania {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let field_count = 0;
            let map = s.serialize_map(Some(field_count))?;
            map.end()
        }
    }
    impl<'de> Deserialize<'de> for UnknownMod {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            struct UnknownModVisitor;
            impl<'de> Visitor<'de> for UnknownModVisitor {
                type Value = UnknownMod;
                fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                    f.write_str("any unknown mod")
                }
                fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    while map.next_entry::<IgnoredAny, IgnoredAny>()?.is_some() {}
                    Ok(UnknownMod {
                        acronym: UnknownMod::UNKNOWN_ACRONYM,
                    })
                }
            }
            d.deserialize_map(UnknownModVisitor)
        }
    }
    impl Serialize for UnknownMod {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            s.serialize_map(Some(0)).and_then(SerializeMap::end)
        }
    }
    impl<'a, 'de> Visitor<'de> for GameModSettingsSeed<'a> {
        type Value = GameMod;
        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("GameMod settings")
        }
        fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
            let d = MapAccessDeserializer::new(map);
            let res = match (self.acronym, self.mode) {
                ("EZ", GameMode::Osu) => GameMod::EasyOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("NF", GameMode::Osu) => GameMod::NoFailOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("HT", GameMode::Osu) => GameMod::HalfTimeOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("DC", GameMode::Osu) => GameMod::DaycoreOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("HR", GameMode::Osu) => GameMod::HardRockOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("SD", GameMode::Osu) => GameMod::SuddenDeathOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("PF", GameMode::Osu) => GameMod::PerfectOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("DT", GameMode::Osu) => GameMod::DoubleTimeOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("NC", GameMode::Osu) => GameMod::NightcoreOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("HD", GameMode::Osu) => GameMod::HiddenOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("FL", GameMode::Osu) => GameMod::FlashlightOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("BL", GameMode::Osu) => GameMod::BlindsOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("ST", GameMode::Osu) => GameMod::StrictTrackingOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("AC", GameMode::Osu) => GameMod::AccuracyChallengeOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("TP", GameMode::Osu) => GameMod::TargetPracticeOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("DA", GameMode::Osu) => GameMod::DifficultyAdjustOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("CL", GameMode::Osu) => GameMod::ClassicOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("RD", GameMode::Osu) => GameMod::RandomOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("MR", GameMode::Osu) => GameMod::MirrorOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("AL", GameMode::Osu) => GameMod::AlternateOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("SG", GameMode::Osu) => GameMod::SingleTapOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("AT", GameMode::Osu) => GameMod::AutoplayOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("CN", GameMode::Osu) => GameMod::CinemaOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("RX", GameMode::Osu) => GameMod::RelaxOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("AP", GameMode::Osu) => GameMod::AutopilotOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("SO", GameMode::Osu) => GameMod::SpunOutOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("TR", GameMode::Osu) => GameMod::TransformOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("WG", GameMode::Osu) => GameMod::WiggleOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("SI", GameMode::Osu) => GameMod::SpinInOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("GR", GameMode::Osu) => GameMod::GrowOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("DF", GameMode::Osu) => GameMod::DeflateOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("WU", GameMode::Osu) => GameMod::WindUpOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("WD", GameMode::Osu) => GameMod::WindDownOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("TC", GameMode::Osu) => GameMod::TraceableOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("BR", GameMode::Osu) => GameMod::BarrelRollOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("AD", GameMode::Osu) => GameMod::ApproachDifferentOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("MU", GameMode::Osu) => GameMod::MutedOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("NS", GameMode::Osu) => GameMod::NoScopeOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("MG", GameMode::Osu) => GameMod::MagnetisedOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("RP", GameMode::Osu) => GameMod::RepelOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("AS", GameMode::Osu) => GameMod::AdaptiveSpeedOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("FR", GameMode::Osu) => GameMod::FreezeFrameOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("BU", GameMode::Osu) => GameMod::BubblesOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("SY", GameMode::Osu) => GameMod::SynesthesiaOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("DP", GameMode::Osu) => GameMod::DepthOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("BM", GameMode::Osu) => GameMod::BloomOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("TD", GameMode::Osu) => GameMod::TouchDeviceOsu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("SV2", GameMode::Osu) => GameMod::ScoreV2Osu(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("EZ", GameMode::Taiko) => GameMod::EasyTaiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("NF", GameMode::Taiko) => GameMod::NoFailTaiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("HT", GameMode::Taiko) => GameMod::HalfTimeTaiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("DC", GameMode::Taiko) => GameMod::DaycoreTaiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("HR", GameMode::Taiko) => GameMod::HardRockTaiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("SD", GameMode::Taiko) => GameMod::SuddenDeathTaiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("PF", GameMode::Taiko) => GameMod::PerfectTaiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("DT", GameMode::Taiko) => GameMod::DoubleTimeTaiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("NC", GameMode::Taiko) => GameMod::NightcoreTaiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("HD", GameMode::Taiko) => GameMod::HiddenTaiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("FL", GameMode::Taiko) => GameMod::FlashlightTaiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("AC", GameMode::Taiko) => GameMod::AccuracyChallengeTaiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("RD", GameMode::Taiko) => GameMod::RandomTaiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("DA", GameMode::Taiko) => GameMod::DifficultyAdjustTaiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("CL", GameMode::Taiko) => GameMod::ClassicTaiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("SW", GameMode::Taiko) => GameMod::SwapTaiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("SG", GameMode::Taiko) => GameMod::SingleTapTaiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("CS", GameMode::Taiko) => GameMod::ConstantSpeedTaiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("AT", GameMode::Taiko) => GameMod::AutoplayTaiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("CN", GameMode::Taiko) => GameMod::CinemaTaiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("RX", GameMode::Taiko) => GameMod::RelaxTaiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("WU", GameMode::Taiko) => GameMod::WindUpTaiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("WD", GameMode::Taiko) => GameMod::WindDownTaiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("MU", GameMode::Taiko) => GameMod::MutedTaiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("AS", GameMode::Taiko) => GameMod::AdaptiveSpeedTaiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("SV2", GameMode::Taiko) => GameMod::ScoreV2Taiko(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("EZ", GameMode::Catch) => GameMod::EasyCatch(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("NF", GameMode::Catch) => GameMod::NoFailCatch(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("HT", GameMode::Catch) => GameMod::HalfTimeCatch(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("DC", GameMode::Catch) => GameMod::DaycoreCatch(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("HR", GameMode::Catch) => GameMod::HardRockCatch(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("SD", GameMode::Catch) => GameMod::SuddenDeathCatch(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("PF", GameMode::Catch) => GameMod::PerfectCatch(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("DT", GameMode::Catch) => GameMod::DoubleTimeCatch(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("NC", GameMode::Catch) => GameMod::NightcoreCatch(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("HD", GameMode::Catch) => GameMod::HiddenCatch(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("FL", GameMode::Catch) => GameMod::FlashlightCatch(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("AC", GameMode::Catch) => GameMod::AccuracyChallengeCatch(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("DA", GameMode::Catch) => GameMod::DifficultyAdjustCatch(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("CL", GameMode::Catch) => GameMod::ClassicCatch(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("MR", GameMode::Catch) => GameMod::MirrorCatch(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("AT", GameMode::Catch) => GameMod::AutoplayCatch(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("CN", GameMode::Catch) => GameMod::CinemaCatch(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("RX", GameMode::Catch) => GameMod::RelaxCatch(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("WU", GameMode::Catch) => GameMod::WindUpCatch(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("WD", GameMode::Catch) => GameMod::WindDownCatch(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("FF", GameMode::Catch) => GameMod::FloatingFruitsCatch(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("MU", GameMode::Catch) => GameMod::MutedCatch(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("NS", GameMode::Catch) => GameMod::NoScopeCatch(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("SV2", GameMode::Catch) => GameMod::ScoreV2Catch(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("EZ", GameMode::Mania) => GameMod::EasyMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("NF", GameMode::Mania) => GameMod::NoFailMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("HT", GameMode::Mania) => GameMod::HalfTimeMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("DC", GameMode::Mania) => GameMod::DaycoreMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("NR", GameMode::Mania) => GameMod::NoReleaseMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("HR", GameMode::Mania) => GameMod::HardRockMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("SD", GameMode::Mania) => GameMod::SuddenDeathMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("PF", GameMode::Mania) => GameMod::PerfectMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("DT", GameMode::Mania) => GameMod::DoubleTimeMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("NC", GameMode::Mania) => GameMod::NightcoreMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("FI", GameMode::Mania) => GameMod::FadeInMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("HD", GameMode::Mania) => GameMod::HiddenMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("CO", GameMode::Mania) => GameMod::CoverMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("FL", GameMode::Mania) => GameMod::FlashlightMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("AC", GameMode::Mania) => GameMod::AccuracyChallengeMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("RD", GameMode::Mania) => GameMod::RandomMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("DS", GameMode::Mania) => GameMod::DualStagesMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("MR", GameMode::Mania) => GameMod::MirrorMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("DA", GameMode::Mania) => GameMod::DifficultyAdjustMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("CL", GameMode::Mania) => GameMod::ClassicMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("IN", GameMode::Mania) => GameMod::InvertMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("CS", GameMode::Mania) => GameMod::ConstantSpeedMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("HO", GameMode::Mania) => GameMod::HoldOffMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("1K", GameMode::Mania) => GameMod::OneKeyMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("2K", GameMode::Mania) => GameMod::TwoKeysMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("3K", GameMode::Mania) => GameMod::ThreeKeysMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("4K", GameMode::Mania) => GameMod::FourKeysMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("5K", GameMode::Mania) => GameMod::FiveKeysMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("6K", GameMode::Mania) => GameMod::SixKeysMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("7K", GameMode::Mania) => GameMod::SevenKeysMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("8K", GameMode::Mania) => GameMod::EightKeysMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("9K", GameMode::Mania) => GameMod::NineKeysMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("10K", GameMode::Mania) => GameMod::TenKeysMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("AT", GameMode::Mania) => GameMod::AutoplayMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("CN", GameMode::Mania) => GameMod::CinemaMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("WU", GameMode::Mania) => GameMod::WindUpMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("WD", GameMode::Mania) => GameMod::WindDownMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("MU", GameMode::Mania) => GameMod::MutedMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("AS", GameMode::Mania) => GameMod::AdaptiveSpeedMania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                ("SV2", GameMode::Mania) => GameMod::ScoreV2Mania(
                    DeserializedGameMod::try_deserialize_mod(d, self.deny_unknown_fields)?,
                ),
                _ => {
                    let acronym = <Acronym as std::str::FromStr>::from_str(self.acronym)
                        .map_err(DeError::custom)?;
                    // All fields are specified already but we still want to clear
                    // out content from the deserializer.
                    #[allow(clippy::needless_update)]
                    let unknown = UnknownMod {
                        acronym,
                        ..Deserialize::deserialize(d)?
                    };
                    match self.mode {
                        GameMode::Osu => GameMod::UnknownOsu(unknown),
                        GameMode::Taiko => GameMod::UnknownTaiko(unknown),
                        GameMode::Catch => GameMod::UnknownCatch(unknown),
                        GameMode::Mania => GameMod::UnknownMania(unknown),
                    }
                }
            };
            Ok(res)
        }
    }
    impl Serialize for GameMod {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut s = s.serialize_map(None)?;
            s.serialize_entry("acronym", self.acronym().as_str())?;
            match self {
                Self::EasyOsu(m) => {
                    let has_some = m.retries.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::HalfTimeOsu(m) => {
                    let has_some = m.speed_change.is_some() || m.adjust_pitch.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::DaycoreOsu(m) => {
                    let has_some = m.speed_change.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::SuddenDeathOsu(m) => {
                    let has_some = m.fail_on_slider_tail.is_some() || m.restart.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::PerfectOsu(m) => {
                    let has_some = m.restart.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::DoubleTimeOsu(m) => {
                    let has_some = m.speed_change.is_some() || m.adjust_pitch.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::NightcoreOsu(m) => {
                    let has_some = m.speed_change.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::HiddenOsu(m) => {
                    let has_some = m.only_fade_approach_circles.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::FlashlightOsu(m) => {
                    let has_some = m.follow_delay.is_some()
                        || m.size_multiplier.is_some()
                        || m.combo_based_size.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::AccuracyChallengeOsu(m) => {
                    let has_some = m.minimum_accuracy.is_some()
                        || m.accuracy_judge_mode.is_some()
                        || m.restart.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::TargetPracticeOsu(m) => {
                    let has_some = m.seed.is_some() || m.metronome.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::DifficultyAdjustOsu(m) => {
                    let has_some = m.circle_size.is_some()
                        || m.approach_rate.is_some()
                        || m.drain_rate.is_some()
                        || m.overall_difficulty.is_some()
                        || m.extended_limits.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::ClassicOsu(m) => {
                    let has_some = m.no_slider_head_accuracy.is_some()
                        || m.classic_note_lock.is_some()
                        || m.always_play_tail_sample.is_some()
                        || m.fade_hit_circle_early.is_some()
                        || m.classic_health.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::RandomOsu(m) => {
                    let has_some = m.angle_sharpness.is_some() || m.seed.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::MirrorOsu(m) => {
                    let has_some = m.reflection.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::WiggleOsu(m) => {
                    let has_some = m.strength.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::GrowOsu(m) => {
                    let has_some = m.start_scale.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::DeflateOsu(m) => {
                    let has_some = m.start_scale.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::WindUpOsu(m) => {
                    let has_some = m.initial_rate.is_some()
                        || m.final_rate.is_some()
                        || m.adjust_pitch.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::WindDownOsu(m) => {
                    let has_some = m.initial_rate.is_some()
                        || m.final_rate.is_some()
                        || m.adjust_pitch.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::BarrelRollOsu(m) => {
                    let has_some = m.spin_speed.is_some() || m.direction.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::ApproachDifferentOsu(m) => {
                    let has_some = m.scale.is_some() || m.style.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::MutedOsu(m) => {
                    let has_some = m.inverse_muting.is_some()
                        || m.enable_metronome.is_some()
                        || m.mute_combo_count.is_some()
                        || m.affects_hit_sounds.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::NoScopeOsu(m) => {
                    let has_some = m.hidden_combo_count.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::MagnetisedOsu(m) => {
                    let has_some = m.attraction_strength.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::RepelOsu(m) => {
                    let has_some = m.repulsion_strength.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::AdaptiveSpeedOsu(m) => {
                    let has_some = m.initial_rate.is_some() || m.adjust_pitch.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::DepthOsu(m) => {
                    let has_some = m.max_depth.is_some() || m.show_approach_circles.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::BloomOsu(m) => {
                    let has_some = m.max_size_combo_count.is_some() || m.max_cursor_size.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::HalfTimeTaiko(m) => {
                    let has_some = m.speed_change.is_some() || m.adjust_pitch.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::DaycoreTaiko(m) => {
                    let has_some = m.speed_change.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::SuddenDeathTaiko(m) => {
                    let has_some = m.restart.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::PerfectTaiko(m) => {
                    let has_some = m.restart.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::DoubleTimeTaiko(m) => {
                    let has_some = m.speed_change.is_some() || m.adjust_pitch.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::NightcoreTaiko(m) => {
                    let has_some = m.speed_change.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::FlashlightTaiko(m) => {
                    let has_some = m.size_multiplier.is_some() || m.combo_based_size.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::AccuracyChallengeTaiko(m) => {
                    let has_some = m.minimum_accuracy.is_some()
                        || m.accuracy_judge_mode.is_some()
                        || m.restart.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::RandomTaiko(m) => {
                    let has_some = m.seed.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::DifficultyAdjustTaiko(m) => {
                    let has_some = m.scroll_speed.is_some()
                        || m.drain_rate.is_some()
                        || m.overall_difficulty.is_some()
                        || m.extended_limits.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::WindUpTaiko(m) => {
                    let has_some = m.initial_rate.is_some()
                        || m.final_rate.is_some()
                        || m.adjust_pitch.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::WindDownTaiko(m) => {
                    let has_some = m.initial_rate.is_some()
                        || m.final_rate.is_some()
                        || m.adjust_pitch.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::MutedTaiko(m) => {
                    let has_some = m.inverse_muting.is_some()
                        || m.enable_metronome.is_some()
                        || m.mute_combo_count.is_some()
                        || m.affects_hit_sounds.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::AdaptiveSpeedTaiko(m) => {
                    let has_some = m.initial_rate.is_some() || m.adjust_pitch.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::EasyCatch(m) => {
                    let has_some = m.retries.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::HalfTimeCatch(m) => {
                    let has_some = m.speed_change.is_some() || m.adjust_pitch.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::DaycoreCatch(m) => {
                    let has_some = m.speed_change.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::SuddenDeathCatch(m) => {
                    let has_some = m.restart.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::PerfectCatch(m) => {
                    let has_some = m.restart.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::DoubleTimeCatch(m) => {
                    let has_some = m.speed_change.is_some() || m.adjust_pitch.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::NightcoreCatch(m) => {
                    let has_some = m.speed_change.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::FlashlightCatch(m) => {
                    let has_some = m.size_multiplier.is_some() || m.combo_based_size.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::AccuracyChallengeCatch(m) => {
                    let has_some = m.minimum_accuracy.is_some()
                        || m.accuracy_judge_mode.is_some()
                        || m.restart.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::DifficultyAdjustCatch(m) => {
                    let has_some = m.circle_size.is_some()
                        || m.approach_rate.is_some()
                        || m.hard_rock_offsets.is_some()
                        || m.drain_rate.is_some()
                        || m.overall_difficulty.is_some()
                        || m.extended_limits.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::WindUpCatch(m) => {
                    let has_some = m.initial_rate.is_some()
                        || m.final_rate.is_some()
                        || m.adjust_pitch.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::WindDownCatch(m) => {
                    let has_some = m.initial_rate.is_some()
                        || m.final_rate.is_some()
                        || m.adjust_pitch.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::MutedCatch(m) => {
                    let has_some = m.inverse_muting.is_some()
                        || m.enable_metronome.is_some()
                        || m.mute_combo_count.is_some()
                        || m.affects_hit_sounds.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::NoScopeCatch(m) => {
                    let has_some = m.hidden_combo_count.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::EasyMania(m) => {
                    let has_some = m.retries.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::HalfTimeMania(m) => {
                    let has_some = m.speed_change.is_some() || m.adjust_pitch.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::DaycoreMania(m) => {
                    let has_some = m.speed_change.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::SuddenDeathMania(m) => {
                    let has_some = m.restart.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::PerfectMania(m) => {
                    let has_some = m.restart.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::DoubleTimeMania(m) => {
                    let has_some = m.speed_change.is_some() || m.adjust_pitch.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::NightcoreMania(m) => {
                    let has_some = m.speed_change.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::CoverMania(m) => {
                    let has_some = m.coverage.is_some() || m.direction.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::FlashlightMania(m) => {
                    let has_some = m.size_multiplier.is_some() || m.combo_based_size.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::AccuracyChallengeMania(m) => {
                    let has_some = m.minimum_accuracy.is_some()
                        || m.accuracy_judge_mode.is_some()
                        || m.restart.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::RandomMania(m) => {
                    let has_some = m.seed.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::DifficultyAdjustMania(m) => {
                    let has_some = m.drain_rate.is_some()
                        || m.overall_difficulty.is_some()
                        || m.extended_limits.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::WindUpMania(m) => {
                    let has_some = m.initial_rate.is_some()
                        || m.final_rate.is_some()
                        || m.adjust_pitch.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::WindDownMania(m) => {
                    let has_some = m.initial_rate.is_some()
                        || m.final_rate.is_some()
                        || m.adjust_pitch.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::MutedMania(m) => {
                    let has_some = m.inverse_muting.is_some()
                        || m.enable_metronome.is_some()
                        || m.mute_combo_count.is_some()
                        || m.affects_hit_sounds.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                Self::AdaptiveSpeedMania(m) => {
                    let has_some = m.initial_rate.is_some() || m.adjust_pitch.is_some();
                    if has_some {
                        s.serialize_entry("settings", m)?;
                    }
                }
                _ => {}
            }
            s.end()
        }
    }
    impl<'de> Deserialize<'de> for GameModIntermode {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            fn try_acronym_to_gamemod<E: DeError>(
                acronym: &MaybeOwnedStr<'_>,
            ) -> Result<GameModIntermode, E> {
                acronym
                    .as_str()
                    .parse()
                    .map(GameModIntermode::from_acronym)
                    .map_err(DeError::custom)
            }
            let raw_seed = GameModRawSeed {
                deny_unknown_fields: true,
            };
            match raw_seed.deserialize(d)? {
                GameModRaw::Bits(bits) => GameModIntermode::try_from_bits(bits)
                    .ok_or_else(|| DeError::custom("invalid bitflags")),
                GameModRaw::Acronym(acronym) => try_acronym_to_gamemod(&acronym),
                GameModRaw::Full { acronym, .. } => try_acronym_to_gamemod(&acronym),
            }
        }
    }
    impl serde::Serialize for GameModIntermode {
        fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            s.serialize_str(self.acronym().as_str())
        }
    }
    impl GameModSettings<'_> {
        pub(crate) fn try_deserialize(
            &self,
            acronym: &str,
            deny_unknown_fields: bool,
        ) -> Option<GameMod> {
            macro_rules! try_deser {
                ( $osu_mod:ident, $taiko_mod:ident, $catch_mod:ident, $mania_mod:ident, ) => {{
                    try_deser!(@ $osu_mod Osu);
                    try_deser!(@ $taiko_mod Taiko);
                    try_deser!(@ $catch_mod Catch);
                    try_deser!(@ $mania_mod Mania);
                }};
                ( @ Skip_ $mode:ident ) => {};
                ( @ $name:ident $mode:ident ) => {
                    if let Ok(m) = DeserializedGameMod::try_deserialize_mod(self, deny_unknown_fields) {
                        return Some(GameMod::$name(m));
                    }
                };
            }
            match acronym {
                "10K" => try_deser!(Skip_, Skip_, Skip_, TenKeysMania,),
                "1K" => try_deser!(Skip_, Skip_, Skip_, OneKeyMania,),
                "2K" => try_deser!(Skip_, Skip_, Skip_, TwoKeysMania,),
                "3K" => try_deser!(Skip_, Skip_, Skip_, ThreeKeysMania,),
                "4K" => try_deser!(Skip_, Skip_, Skip_, FourKeysMania,),
                "5K" => try_deser!(Skip_, Skip_, Skip_, FiveKeysMania,),
                "6K" => try_deser!(Skip_, Skip_, Skip_, SixKeysMania,),
                "7K" => try_deser!(Skip_, Skip_, Skip_, SevenKeysMania,),
                "8K" => try_deser!(Skip_, Skip_, Skip_, EightKeysMania,),
                "9K" => try_deser!(Skip_, Skip_, Skip_, NineKeysMania,),
                "AC" => try_deser!(
                    AccuracyChallengeOsu,
                    AccuracyChallengeTaiko,
                    AccuracyChallengeCatch,
                    AccuracyChallengeMania,
                ),
                "AD" => try_deser!(ApproachDifferentOsu, Skip_, Skip_, Skip_,),
                "AL" => try_deser!(AlternateOsu, Skip_, Skip_, Skip_,),
                "AP" => try_deser!(AutopilotOsu, Skip_, Skip_, Skip_,),
                "AS" => try_deser!(
                    AdaptiveSpeedOsu,
                    AdaptiveSpeedTaiko,
                    Skip_,
                    AdaptiveSpeedMania,
                ),
                "AT" => try_deser!(AutoplayOsu, AutoplayTaiko, AutoplayCatch, AutoplayMania,),
                "BL" => try_deser!(BlindsOsu, Skip_, Skip_, Skip_,),
                "BM" => try_deser!(BloomOsu, Skip_, Skip_, Skip_,),
                "BR" => try_deser!(BarrelRollOsu, Skip_, Skip_, Skip_,),
                "BU" => try_deser!(BubblesOsu, Skip_, Skip_, Skip_,),
                "CL" => try_deser!(ClassicOsu, ClassicTaiko, ClassicCatch, ClassicMania,),
                "CN" => try_deser!(CinemaOsu, CinemaTaiko, CinemaCatch, CinemaMania,),
                "CO" => try_deser!(Skip_, Skip_, Skip_, CoverMania,),
                "CS" => try_deser!(Skip_, ConstantSpeedTaiko, Skip_, ConstantSpeedMania,),
                "DA" => try_deser!(
                    DifficultyAdjustOsu,
                    DifficultyAdjustTaiko,
                    DifficultyAdjustCatch,
                    DifficultyAdjustMania,
                ),
                "DC" => try_deser!(DaycoreOsu, DaycoreTaiko, DaycoreCatch, DaycoreMania,),
                "DF" => try_deser!(DeflateOsu, Skip_, Skip_, Skip_,),
                "DP" => try_deser!(DepthOsu, Skip_, Skip_, Skip_,),
                "DS" => try_deser!(Skip_, Skip_, Skip_, DualStagesMania,),
                "DT" => try_deser!(
                    DoubleTimeOsu,
                    DoubleTimeTaiko,
                    DoubleTimeCatch,
                    DoubleTimeMania,
                ),
                "EZ" => try_deser!(EasyOsu, EasyTaiko, EasyCatch, EasyMania,),
                "FF" => try_deser!(Skip_, Skip_, FloatingFruitsCatch, Skip_,),
                "FI" => try_deser!(Skip_, Skip_, Skip_, FadeInMania,),
                "FL" => try_deser!(
                    FlashlightOsu,
                    FlashlightTaiko,
                    FlashlightCatch,
                    FlashlightMania,
                ),
                "FR" => try_deser!(FreezeFrameOsu, Skip_, Skip_, Skip_,),
                "GR" => try_deser!(GrowOsu, Skip_, Skip_, Skip_,),
                "HD" => try_deser!(HiddenOsu, HiddenTaiko, HiddenCatch, HiddenMania,),
                "HO" => try_deser!(Skip_, Skip_, Skip_, HoldOffMania,),
                "HR" => try_deser!(HardRockOsu, HardRockTaiko, HardRockCatch, HardRockMania,),
                "HT" => try_deser!(HalfTimeOsu, HalfTimeTaiko, HalfTimeCatch, HalfTimeMania,),
                "IN" => try_deser!(Skip_, Skip_, Skip_, InvertMania,),
                "MG" => try_deser!(MagnetisedOsu, Skip_, Skip_, Skip_,),
                "MR" => try_deser!(MirrorOsu, Skip_, MirrorCatch, MirrorMania,),
                "MU" => try_deser!(MutedOsu, MutedTaiko, MutedCatch, MutedMania,),
                "NC" => try_deser!(NightcoreOsu, NightcoreTaiko, NightcoreCatch, NightcoreMania,),
                "NF" => try_deser!(NoFailOsu, NoFailTaiko, NoFailCatch, NoFailMania,),
                "NR" => try_deser!(Skip_, Skip_, Skip_, NoReleaseMania,),
                "NS" => try_deser!(NoScopeOsu, Skip_, NoScopeCatch, Skip_,),
                "PF" => try_deser!(PerfectOsu, PerfectTaiko, PerfectCatch, PerfectMania,),
                "RD" => try_deser!(RandomOsu, RandomTaiko, Skip_, RandomMania,),
                "RP" => try_deser!(RepelOsu, Skip_, Skip_, Skip_,),
                "RX" => try_deser!(RelaxOsu, RelaxTaiko, RelaxCatch, Skip_,),
                "SD" => try_deser!(
                    SuddenDeathOsu,
                    SuddenDeathTaiko,
                    SuddenDeathCatch,
                    SuddenDeathMania,
                ),
                "SG" => try_deser!(SingleTapOsu, SingleTapTaiko, Skip_, Skip_,),
                "SI" => try_deser!(SpinInOsu, Skip_, Skip_, Skip_,),
                "SO" => try_deser!(SpunOutOsu, Skip_, Skip_, Skip_,),
                "ST" => try_deser!(StrictTrackingOsu, Skip_, Skip_, Skip_,),
                "SV2" => try_deser!(ScoreV2Osu, ScoreV2Taiko, ScoreV2Catch, ScoreV2Mania,),
                "SW" => try_deser!(Skip_, SwapTaiko, Skip_, Skip_,),
                "SY" => try_deser!(SynesthesiaOsu, Skip_, Skip_, Skip_,),
                "TC" => try_deser!(TraceableOsu, Skip_, Skip_, Skip_,),
                "TD" => try_deser!(TouchDeviceOsu, Skip_, Skip_, Skip_,),
                "TP" => try_deser!(TargetPracticeOsu, Skip_, Skip_, Skip_,),
                "TR" => try_deser!(TransformOsu, Skip_, Skip_, Skip_,),
                "WD" => try_deser!(WindDownOsu, WindDownTaiko, WindDownCatch, WindDownMania,),
                "WG" => try_deser!(WiggleOsu, Skip_, Skip_, Skip_,),
                "WU" => try_deser!(WindUpOsu, WindUpTaiko, WindUpCatch, WindUpMania,),
                _ => {}
            }
            None
        }
    }
};
#[macro_export(local_inner_macros)]
#[cfg(feature = "macros")]
#[doc(hidden)]
macro_rules! mods_inner {
    ( @ $mode:ident: $( $acronym:tt )* ) => {{
        // Making sure it's a valid GameMode
        let _ = $crate::GameMode::$mode;

        #[allow(unused_mut)]
        let mut mods = $crate::GameMods::new();
        $( mods.insert(mods_inner!(< ! $mode $acronym)(Default::default())); )*
        mods
    }};
    ( @ $( $acronym:tt )* ) => {{
        #[allow(unused_mut)]
        let mut mods = $crate::GameModsIntermode::new();
        $( mods.insert(mods_inner!(< $acronym)); )*
        mods
    }};

    // Translating acronym to variant name
    ( < $( ! $mode:ident )? 10K ) => { mods_inner!(> $( $mode )? TenKeys ) };
    ( < $( ! $mode:ident )? 1K ) => { mods_inner!(> $( $mode )? OneKey ) };
    ( < $( ! $mode:ident )? 2K ) => { mods_inner!(> $( $mode )? TwoKeys ) };
    ( < $( ! $mode:ident )? 3K ) => { mods_inner!(> $( $mode )? ThreeKeys ) };
    ( < $( ! $mode:ident )? 4K ) => { mods_inner!(> $( $mode )? FourKeys ) };
    ( < $( ! $mode:ident )? 5K ) => { mods_inner!(> $( $mode )? FiveKeys ) };
    ( < $( ! $mode:ident )? 6K ) => { mods_inner!(> $( $mode )? SixKeys ) };
    ( < $( ! $mode:ident )? 7K ) => { mods_inner!(> $( $mode )? SevenKeys ) };
    ( < $( ! $mode:ident )? 8K ) => { mods_inner!(> $( $mode )? EightKeys ) };
    ( < $( ! $mode:ident )? 9K ) => { mods_inner!(> $( $mode )? NineKeys ) };
    ( < $( ! $mode:ident )? AC ) => { mods_inner!(> $( $mode )? AccuracyChallenge ) };
    ( < $( ! $mode:ident )? AD ) => { mods_inner!(> $( $mode )? ApproachDifferent ) };
    ( < $( ! $mode:ident )? AL ) => { mods_inner!(> $( $mode )? Alternate ) };
    ( < $( ! $mode:ident )? AP ) => { mods_inner!(> $( $mode )? Autopilot ) };
    ( < $( ! $mode:ident )? AS ) => { mods_inner!(> $( $mode )? AdaptiveSpeed ) };
    ( < $( ! $mode:ident )? AT ) => { mods_inner!(> $( $mode )? Autoplay ) };
    ( < $( ! $mode:ident )? BL ) => { mods_inner!(> $( $mode )? Blinds ) };
    ( < $( ! $mode:ident )? BM ) => { mods_inner!(> $( $mode )? Bloom ) };
    ( < $( ! $mode:ident )? BR ) => { mods_inner!(> $( $mode )? BarrelRoll ) };
    ( < $( ! $mode:ident )? BU ) => { mods_inner!(> $( $mode )? Bubbles ) };
    ( < $( ! $mode:ident )? CL ) => { mods_inner!(> $( $mode )? Classic ) };
    ( < $( ! $mode:ident )? CN ) => { mods_inner!(> $( $mode )? Cinema ) };
    ( < $( ! $mode:ident )? CO ) => { mods_inner!(> $( $mode )? Cover ) };
    ( < $( ! $mode:ident )? CS ) => { mods_inner!(> $( $mode )? ConstantSpeed ) };
    ( < $( ! $mode:ident )? DA ) => { mods_inner!(> $( $mode )? DifficultyAdjust ) };
    ( < $( ! $mode:ident )? DC ) => { mods_inner!(> $( $mode )? Daycore ) };
    ( < $( ! $mode:ident )? DF ) => { mods_inner!(> $( $mode )? Deflate ) };
    ( < $( ! $mode:ident )? DP ) => { mods_inner!(> $( $mode )? Depth ) };
    ( < $( ! $mode:ident )? DS ) => { mods_inner!(> $( $mode )? DualStages ) };
    ( < $( ! $mode:ident )? DT ) => { mods_inner!(> $( $mode )? DoubleTime ) };
    ( < $( ! $mode:ident )? EZ ) => { mods_inner!(> $( $mode )? Easy ) };
    ( < $( ! $mode:ident )? FF ) => { mods_inner!(> $( $mode )? FloatingFruits ) };
    ( < $( ! $mode:ident )? FI ) => { mods_inner!(> $( $mode )? FadeIn ) };
    ( < $( ! $mode:ident )? FL ) => { mods_inner!(> $( $mode )? Flashlight ) };
    ( < $( ! $mode:ident )? FR ) => { mods_inner!(> $( $mode )? FreezeFrame ) };
    ( < $( ! $mode:ident )? GR ) => { mods_inner!(> $( $mode )? Grow ) };
    ( < $( ! $mode:ident )? HD ) => { mods_inner!(> $( $mode )? Hidden ) };
    ( < $( ! $mode:ident )? HO ) => { mods_inner!(> $( $mode )? HoldOff ) };
    ( < $( ! $mode:ident )? HR ) => { mods_inner!(> $( $mode )? HardRock ) };
    ( < $( ! $mode:ident )? HT ) => { mods_inner!(> $( $mode )? HalfTime ) };
    ( < $( ! $mode:ident )? IN ) => { mods_inner!(> $( $mode )? Invert ) };
    ( < $( ! $mode:ident )? MG ) => { mods_inner!(> $( $mode )? Magnetised ) };
    ( < $( ! $mode:ident )? MR ) => { mods_inner!(> $( $mode )? Mirror ) };
    ( < $( ! $mode:ident )? MU ) => { mods_inner!(> $( $mode )? Muted ) };
    ( < $( ! $mode:ident )? NC ) => { mods_inner!(> $( $mode )? Nightcore ) };
    ( < $( ! $mode:ident )? NF ) => { mods_inner!(> $( $mode )? NoFail ) };
    ( < $( ! $mode:ident )? NR ) => { mods_inner!(> $( $mode )? NoRelease ) };
    ( < $( ! $mode:ident )? NS ) => { mods_inner!(> $( $mode )? NoScope ) };
    ( < $( ! $mode:ident )? PF ) => { mods_inner!(> $( $mode )? Perfect ) };
    ( < $( ! $mode:ident )? RD ) => { mods_inner!(> $( $mode )? Random ) };
    ( < $( ! $mode:ident )? RP ) => { mods_inner!(> $( $mode )? Repel ) };
    ( < $( ! $mode:ident )? RX ) => { mods_inner!(> $( $mode )? Relax ) };
    ( < $( ! $mode:ident )? SD ) => { mods_inner!(> $( $mode )? SuddenDeath ) };
    ( < $( ! $mode:ident )? SG ) => { mods_inner!(> $( $mode )? SingleTap ) };
    ( < $( ! $mode:ident )? SI ) => { mods_inner!(> $( $mode )? SpinIn ) };
    ( < $( ! $mode:ident )? SO ) => { mods_inner!(> $( $mode )? SpunOut ) };
    ( < $( ! $mode:ident )? ST ) => { mods_inner!(> $( $mode )? StrictTracking ) };
    ( < $( ! $mode:ident )? SV2 ) => { mods_inner!(> $( $mode )? ScoreV2 ) };
    ( < $( ! $mode:ident )? SW ) => { mods_inner!(> $( $mode )? Swap ) };
    ( < $( ! $mode:ident )? SY ) => { mods_inner!(> $( $mode )? Synesthesia ) };
    ( < $( ! $mode:ident )? TC ) => { mods_inner!(> $( $mode )? Traceable ) };
    ( < $( ! $mode:ident )? TD ) => { mods_inner!(> $( $mode )? TouchDevice ) };
    ( < $( ! $mode:ident )? TP ) => { mods_inner!(> $( $mode )? TargetPractice ) };
    ( < $( ! $mode:ident )? TR ) => { mods_inner!(> $( $mode )? Transform ) };
    ( < $( ! $mode:ident )? WD ) => { mods_inner!(> $( $mode )? WindDown ) };
    ( < $( ! $mode:ident )? WG ) => { mods_inner!(> $( $mode )? Wiggle ) };
    ( < $( ! $mode:ident )? WU ) => { mods_inner!(> $( $mode )? WindUp ) };

    // Unknown acronym
    ( < ! $mode:ident $other:tt $( $rest:tt )* ) => { mods_inner!(<< $other) };
    ( < $other:tt $( $rest:tt )* ) => { mods_inner!(<< $other) };
    ( << $other:tt ) => {
        std::compile_error!(std::concat!("unknown mod acronym `", std::stringify!($other), "`"))
    };

    // Prefixing variant name with the full type path
    ( > $mode:ident $name:ident ) => {
        $crate::macros::paste! { $crate::generated_mods::GameMod::[<$name $mode>] }
    };
    ( > $name:ident ) => {
        $crate::generated_mods::GameModIntermode::$name
    };
}
