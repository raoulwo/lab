#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "gun.h"

typedef struct {
  char *name;
  struct gun_t *gun;
} player_t;

player_t *player_new() { return (player_t *)malloc(sizeof(player_t)); }

void player_ctor(player_t *player, const char *name) {
  player->name = (char *)malloc((strlen(name) + 1) * sizeof(char));
  strcpy(player->name, name);
  player->gun = 0;
}

void player_dtor(player_t *player) { free(player->name); }

void player_pickup_gun(player_t *player, struct gun_t *gun) {
  // Here, the *aggregation* relation between the player and the
  // gun begins.
  player->gun = gun;
}

void player_drop_gun(player_t *player) {
  // Here, the *aggregation* relation ends. The gun is not freed
  // since the player isn't the owner of the gun.
  player->gun = 0;
}

void player_shoot(player_t *player) {
  if (player->gun) {
    gun_trigger(player->gun);
  } else {
    fprintf(stderr, "Player wants to shoot but they don't have a gun!");
    exit(1);
  }
}
