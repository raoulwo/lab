#include <stdlib.h>

typedef int bool_t;

typedef struct {
  int bullets;
} gun_t;

gun_t *gun_new() { return (gun_t *)malloc(sizeof(gun_t)); }

void gun_ctor(gun_t *gun, int bullets) {
  gun->bullets = 0;
  if (bullets > 0) {
    gun->bullets = bullets;
  }
}

void gun_dtor(gun_t *gun) {}

bool_t gun_has_bullets(gun_t *gun) { return gun->bullets > 0; }

void gun_trigger(gun_t *gun) { gun->bullets--; }

void gun_reload(gun_t *gun) { gun->bullets = 7; }
