#include <pmdsky.h>
#include <cot.h>

// Remove the comment in patches/patch.cotpatch to enable this example patch
int CustomGetMovePower(struct entity* entity, struct move* move) {
  // Randomize move power
  int rolledPower = RandRange(1, 100);

  // Print the rolled value to the message log
  char messageBuffer[32];
  Snprintf(messageBuffer, 32, "Rolled move power %d!", rolledPower);
  
  LogMessage(entity, messageBuffer, true);

  return rolledPower;
}

// Trigger speed boost and reset counter
// Also reset counter if last action was not a move

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
