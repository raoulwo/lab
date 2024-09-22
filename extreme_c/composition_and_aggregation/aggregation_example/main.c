#include <stdlib.h>

#include "gun.h"
#include "player.h"

int main(int argc, char **argv) {
  struct gun_t *gun = gun_new();
  gun_ctor(gun, 3);

  struct player_t *player = player_new();
  player_ctor(player, "Billy");

  player_pickup_gun(player, gun);

  while (gun_has_bullets(gun)) {
    player_shoot(player);
  }
  gun_reload(gun);
  while (gun_has_bullets(gun)) {
    player_shoot(player);
  }

  player_drop_gun(player);

  player_dtor(player);
  free(player);

  gun_dtor(gun);
  free(gun);

  return 0;
}
