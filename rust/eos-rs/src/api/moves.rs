//! Functions related to getting information about monster moves.

use crate::api::_common::get_faint_reason;
use crate::api::dungeon_mode::DungeonEntity;
use crate::api::enums::{MoveCategory, Weather};
use crate::api::items::ItemId;
use crate::api::types::MonsterTypeId;
use crate::ffi;

/// A monster move.
pub type Move = ffi::move_;

/// A move ID with associated methods to get metadata.
///
/// Use the associated constants or the [`Self::new`] method to get instances of this.
pub type MoveId = ffi::move_id;
impl Copy for MoveId {}

#[repr(u32)]
#[derive(PartialEq, Eq, Clone, Copy)]
/// Move target (i.e., who does a move affect when used?).
pub enum MoveTarget {
    Enemies = ffi::move_target::TARGET_ENEMIES,
    Party = ffi::move_target::TARGET_PARTY,
    All = ffi::move_target::TARGET_ALL,
    User = ffi::move_target::TARGET_USER,
    EnemiesAfterCharging = ffi::move_target::TARGET_ENEMIES_AFTER_CHARGING,
    AllExceptUser = ffi::move_target::TARGET_ALL_EXCEPT_USER,
    Teammates = ffi::move_target::TARGET_TEAMMATES,
    Special = ffi::move_target::TARGET_SPECIAL,
}

impl TryInto<MoveTarget> for ffi::move_target::Type {
    type Error = ();

    fn try_into(self) -> Result<MoveTarget, Self::Error> {
        match self {
            ffi::move_target::TARGET_ENEMIES => Ok(MoveTarget::Enemies),
            ffi::move_target::TARGET_PARTY => Ok(MoveTarget::Party),
            ffi::move_target::TARGET_ALL => Ok(MoveTarget::All),
            ffi::move_target::TARGET_USER => Ok(MoveTarget::User),
            ffi::move_target::TARGET_ENEMIES_AFTER_CHARGING => Ok(MoveTarget::EnemiesAfterCharging),
            ffi::move_target::TARGET_ALL_EXCEPT_USER => Ok(MoveTarget::AllExceptUser),
            ffi::move_target::TARGET_TEAMMATES => Ok(MoveTarget::Teammates),
            ffi::move_target::TARGET_SPECIAL => Ok(MoveTarget::Special),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Eq, Clone, Copy)]
/// Move range.
pub enum MoveRange {
    Front = ffi::move_range::RANGE_FRONT,
    FrontAndSides = ffi::move_range::RANGE_FRONT_AND_SIDES,
    Nearby = ffi::move_range::RANGE_NEARBY,
    Room = ffi::move_range::RANGE_ROOM,
    Front2 = ffi::move_range::RANGE_FRONT_2,
    Front10 = ffi::move_range::RANGE_FRONT_10,
    Floor = ffi::move_range::RANGE_FLOOR,
    User = ffi::move_range::RANGE_USER,
    FrontWithCornerCutting = ffi::move_range::RANGE_FRONT_WITH_CORNER_CUTTING,
    Front2WithCornerCutting = ffi::move_range::RANGE_FRONT_2_WITH_CORNER_CUTTING,
    Special = ffi::move_range::RANGE_SPECIAL,
}

impl TryInto<MoveRange> for ffi::move_range::Type {
    type Error = ();

    fn try_into(self) -> Result<MoveRange, Self::Error> {
        match self {
            ffi::move_range::RANGE_FRONT => Ok(MoveRange::Front),
            ffi::move_range::RANGE_FRONT_AND_SIDES => Ok(MoveRange::FrontAndSides),
            ffi::move_range::RANGE_NEARBY => Ok(MoveRange::Nearby),
            ffi::move_range::RANGE_ROOM => Ok(MoveRange::Room),
            ffi::move_range::RANGE_FRONT_2 => Ok(MoveRange::Front2),
            ffi::move_range::RANGE_FRONT_10 => Ok(MoveRange::Front10),
            ffi::move_range::RANGE_FLOOR => Ok(MoveRange::Floor),
            ffi::move_range::RANGE_USER => Ok(MoveRange::User),
            ffi::move_range::RANGE_FRONT_WITH_CORNER_CUTTING => {
                Ok(MoveRange::FrontWithCornerCutting)
            }
            ffi::move_range::RANGE_FRONT_2_WITH_CORNER_CUTTING => {
                Ok(MoveRange::Front2WithCornerCutting)
            }
            ffi::move_range::RANGE_SPECIAL => Ok(MoveRange::Special),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Eq, Clone, Copy)]
/// Conditions checked by the AI to determine when a move should be used.
/// It does not affect how the move works.
pub enum MoveAiCondition {
    None = ffi::move_ai_condition::AI_CONDITION_NONE,
    /// The AI will consider a target elegible wirh a chance equal to the
    /// move's "ai_condition_random_chance" value.
    Random = ffi::move_ai_condition::AI_CONDITION_RANDOM,
    /// Target has HP <= 25%
    Hp25 = ffi::move_ai_condition::AI_CONDITION_HP_25,
    /// Target has a negative status condition
    Status = ffi::move_ai_condition::AI_CONDITION_STATUS,
    /// Target is asleep, napping or in a nightmare
    Asleep = ffi::move_ai_condition::AI_CONDITION_ASLEEP,
    /// Target is ghost-type and not exposed
    Ghost = ffi::move_ai_condition::AI_CONDITION_GHOST,
    /// Target has HP <= 25% or a negative status condition
    Hp25OrStatus = ffi::move_ai_condition::AI_CONDITION_HP_25_OR_STATUS,
}

impl TryInto<MoveAiCondition> for ffi::move_ai_condition::Type {
    type Error = ();

    fn try_into(self) -> Result<MoveAiCondition, Self::Error> {
        match self {
            ffi::move_ai_condition::AI_CONDITION_NONE => Ok(MoveAiCondition::None),
            ffi::move_ai_condition::AI_CONDITION_RANDOM => Ok(MoveAiCondition::Random),
            ffi::move_ai_condition::AI_CONDITION_HP_25 => Ok(MoveAiCondition::Hp25),
            ffi::move_ai_condition::AI_CONDITION_STATUS => Ok(MoveAiCondition::Status),
            ffi::move_ai_condition::AI_CONDITION_ASLEEP => Ok(MoveAiCondition::Asleep),
            ffi::move_ai_condition::AI_CONDITION_GHOST => Ok(MoveAiCondition::Ghost),
            ffi::move_ai_condition::AI_CONDITION_HP_25_OR_STATUS => {
                Ok(MoveAiCondition::Hp25OrStatus)
            }
            _ => Err(()),
        }
    }
}

/// Range, target and AI data for a move.
/// Values are None, if they are invalid / non-standard.
pub struct MoveTargetAndRange {
    pub target: Option<MoveTarget>,
    pub range: Option<MoveRange>,
    pub ai_condition: Option<MoveAiCondition>,
    pub unused: u16,
}

impl From<ffi::move_target_and_range> for MoveTargetAndRange {
    fn from(tr: ffi::move_target_and_range) -> Self {
        MoveTargetAndRange {
            target: tr.target().try_into().ok(),
            range: tr.range().try_into().ok(),
            ai_condition: tr.ai_condition().try_into().ok(),
            unused: tr.unused(),
        }
    }
}

/// Will fail, if any values are None in MoveTargetAndRange.
impl TryFrom<MoveTargetAndRange> for ffi::move_target_and_range {
    type Error = ();

    fn try_from(value: MoveTargetAndRange) -> Result<Self, Self::Error> {
        if value.target.is_none() || value.range.is_none() || value.ai_condition.is_none() {
            return Err(());
        }
        Ok(ffi::move_target_and_range {
            _bitfield_align_1: [],
            _bitfield_1: ffi::move_target_and_range::new_bitfield_1(
                value.target.unwrap() as ffi::move_target::Type,
                value.range.unwrap() as ffi::move_range::Type,
                value.ai_condition.unwrap() as ffi::move_ai_condition::Type,
                value.unused,
            ),
        })
    }
}

/// See [`MoveId`] for additional metadata methods.
impl Move {
    /// Initializes a move info struct.
    ///
    /// This sets f_exists and f_enabled_for_ai on the flags, the ID to the given ID,
    /// the PP to the max PP for the move ID, and the ginseng boost to 0.
    ///
    /// # Safety
    /// The pointer must point to a valid Move or uninitialized Move.
    pub unsafe fn init(move_pnt: *mut Self, move_id: MoveId) {
        ffi::InitMove(move_pnt, move_id)
    }
    /// Returns the move ID
    pub fn id(&self) -> MoveId {
        self.id.val()
    }

    /// Gets the move target-and-range field. See struct move_target_and_range in the C headers.
    pub fn get_target_and_range(&self, is_ai: bool) -> MoveTargetAndRange {
        unsafe { ffi::GetMoveTargetAndRange(force_mut_ptr!(self), is_ai as ffi::bool_) }.into()
    }

    /// Gets the base power of the move.
    pub fn get_base_power(&self) -> i32 {
        unsafe { ffi::GetMoveBasePower(force_mut_ptr!(self)) }
    }

    /// Gets the maximum PP for the move.
    ///
    /// Returns max PP for the given move, capped at 99.
    pub fn get_max_pp(&self) -> i32 {
        unsafe { ffi::GetMaxPp(force_mut_ptr!(self)) }
    }

    /// Gets the critical hit chance of the move.
    pub fn get_crit_chance(&self) -> i32 {
        unsafe { ffi::GetMoveCritChance(force_mut_ptr!(self)) }
    }

    /// Returns whether a move's range string is 19 ("User").
    pub fn is_move_range_string_19(&self) -> bool {
        unsafe { ffi::IsMoveRangeString19(force_mut_ptr!(self)) > 0 }
    }

    /// Gets the type of the move.
    pub fn get_type(&self) -> MonsterTypeId {
        unsafe { ffi::GetMoveType(force_mut_ptr!(self)) }
    }

    /// Gets the AI weight of a move.
    pub fn get_ai_weight(&self) -> u8 {
        unsafe { ffi::GetMoveAiWeight(force_mut_ptr!(self)) }
    }

    /// Gets the accuracy1 value for the move.
    pub fn get_accuracy1(&self) -> u8 {
        unsafe { ffi::GetMoveAccuracyOrAiChance(force_mut_ptr!(self), 0) }
    }

    /// Gets the accuracy2 value for the move.
    pub fn get_accuracy2(&self) -> u8 {
        unsafe { ffi::GetMoveAccuracyOrAiChance(force_mut_ptr!(self), 1) }
    }

    /// Gets the `ai_condition_random_chance` value for the move.
    pub fn get_ai_condition_random_chance(&self) -> u8 {
        unsafe { ffi::GetMoveAccuracyOrAiChance(force_mut_ptr!(self), 2) }
    }

    /// Checks whether a moved used by a monster should play its alternative animation.
    /// Includes checks for Curse, Snore, Sleep Talk, Solar Beam and 2-turn moves.
    pub fn should_play_alternative_animation(&self, user: &DungeonEntity) -> bool {
        unsafe {
            ffi::ShouldMovePlayAlternativeAnimation(force_mut_ptr!(self), force_mut_ptr!(user)) > 0
        }
    }

    /// Returns the move animation ID that should be played for a move.
    /// It contains a check for weather ball. After that, if the parameter
    /// `should_play_alternative_animation` is false, the move ID is returned.
    ///
    /// `should_play_alternative_animation` can be retrieved with
    /// [`Self::should_play_alternative_animation`].
    ///
    /// If it's true, there's a bunch of manual ID checks that result on a certain hardcoded return
    /// value.
    pub fn get_animation_id(
        &self,
        apparent_weather: Weather,
        should_play_alternative_animation: bool,
    ) -> u16 {
        unsafe {
            ffi::GetMoveAnimationId(
                force_mut_ptr!(self),
                apparent_weather as ffi::weather_id::Type,
                should_play_alternative_animation as ffi::bool_,
            )
        }
    }
}

/// This impl provides general metadata about moves in the game.
///
/// See [`Move`] for additional metadata methods.
impl MoveId {
    /// Returns the ID struct for the move with the given ID.
    ///
    /// # Safety
    /// The caller must make sure the ID is valid (refers to an existing move),
    /// otherwise this is UB.
    pub const unsafe fn new(id: u32) -> Self {
        Self(id)
    }

    /// Returns the ID of this move.
    pub const fn id(&self) -> u32 {
        self.0
    }

    /// Checks if the move is a recoil move (affected by Reckless).
    pub fn is_recoil_move(&self) -> bool {
        unsafe { ffi::IsRecoilMove(*self) > 0 }
    }

    /// Checks if the move is a punch move (affected by Iron Fist).
    pub fn is_punch_move(&self) -> bool {
        unsafe { ffi::IsPunchMove(*self) > 0 }
    }

    /// Gets a move's category (physical, special, status). Returns None if the category is invalid.
    pub fn get_category(&self) -> Option<MoveCategory> {
        unsafe { ffi::GetMoveCategory(*self) }.try_into().ok()
    }

    /// Gets the faint reason code (see HandleFaint) for a given move-item combination.
    ///         
    /// If there's no item, the reason code is the move ID. If the item is an orb, return
    /// FAINT_REASON_ORB_ITEM. Otherwise, return FAINT_REASON_NON_ORB_ITEM.
    pub fn get_faint_reason(&self, item_id: ItemId) -> ffi::faint_reason {
        get_faint_reason(*self, item_id)
    }
}

impl From<MoveId> for u32 {
    fn from(v: MoveId) -> Self {
        v.0
    }
}
