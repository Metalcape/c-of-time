#include <pmdsky.h>
#include <cot.h>

// Called when using moves. Should return true if a custom effect was applied.
// This function is only called if the move doesn't fail due to a missing target
bool CustomApplyMoveEffect(
  move_effect_input* data, struct entity* user, struct entity* target, struct move* move
) {
  COT_LOGFMT(COT_LOG_CAT_EFFECTS, "Running move effect %d", data->move_id);
  switch (data->move_id) {
//    case MOVE_SCRATCH:
//      // Replace move 260 (Scratch) with custom Body Press effect
//      data->out_dealt_damage = MoveBodyPress(user, target, move);
//      // Return true to signal that we've handled the effect
//      return true;
    default:
      // Return false to use the game's normal effect
      return false;
  }
}
