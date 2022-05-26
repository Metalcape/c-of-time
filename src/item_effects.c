#include <pmdsky.h>
#include <cot.h>

// Called when using items. Should return true if a custom effect was applied.
bool CustomApplyItemEffect(
  struct entity* user, struct entity* target, struct item* item, bool is_thrown
) {
  switch (item->id.val) {
  //  case ITEM_MAX_ELIXIR:
  //    // Replace item 99 (Max Elixir) with custom Elixir effect
  //    ItemElixir(target);
  //    // Return true to signal that we've handled the effect
  //    return true;
    default:
      return false;
  }
}
