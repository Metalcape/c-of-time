#include <pmdsky.h>
#include <cot.h>

// Trigger speed boost and reset counter
// Also reset counter if last action was not a move
void SpeedBoostTrigger(struct entity* entity) {
  struct monster* mon = entity->info;

  int speed_stage = mon->statuses.speed_stage;
  uint8_t* counter = &(mon->field_0x1f);
  uint8_t threshold = 1;
  uint16_t last_action = mon->action_id;

  switch(speed_stage) {
    case 2:
      threshold = 2;
      break;
    case 3:
      threshold = 3;
      break;
    case 4:
      threshold = 0xFF; // Can't go over 4x speed
  }

  // Increase speed and reset counter
  if(*counter >= threshold) {
    // 5 turns at x2
    // 10 turns at x3
    // 15 turns at x4
    BoostSpeedOneStage(entity, entity, 5 * threshold, false);
    *counter = 0;
  }

  // If the last action was not a move, reset the counter regardless
  // 0x14 is for the player, 0x15 for AI
  if(last_action != 0x14 && last_action != 0x15)
    *counter = 0;
}

// Increase the speed boost counter if the entity used a move
void IncreaseSpeedBoostCounter(struct entity* entity) {
  if (AbilityIsActive(entity, ABILITY_SPEED_BOOST)) {
    struct monster* mon = entity->info;
    uint8_t* counter = &(mon->field_0x1f);
    uint16_t last_action = mon->action_id;
    if(last_action == 0x14 || last_action == 0x15)
      *counter = *counter + 1;
  }
}

// If either Course Checker or Gap Prober are active, we must check for obstacles between user and target
bool IqCheckForObstacles(struct entity* user) {
  bool cc = IqSkillIsEnabled(user, IQ_COURSE_CHECKER);
  bool gp = IqSkillIsEnabled(user, IQ_GAP_PROBER);

  if(cc || gp) return true;
  else return false;
}

// If Gap Prober is active we skip over checking for other monsters in the way
bool GapProberIsActive(struct entity* entity) {
  return IqSkillIsEnabled(entity, IQ_GAP_PROBER);
}
