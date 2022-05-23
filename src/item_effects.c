#include <pmdsky.h>
#include <cot.h>

// Called when using items. Should return true if a custom effect was applied.
bool CustomApplyItemEffect(
  struct entity* user, struct entity* target, struct item* item, bool is_thrown
) {
  COT_LOGFMT(COT_LOG_CAT_EFFECTS, "Running item effect %d", item->id.val);
  switch (item->id.val) {
  //  case ITEM_MAX_ELIXIR:
  //    // Replace item 99 (Max Elixir) with custom Elixir effect
  //    ItemElixir(target);
  //    // Return true to signal that we've handled the effect
  //    return true;
    default:
      // Return false to use the game's normal effect
      return false;
  }
}
