#include <stdio.h>
#include <string.h>

// During macro expansion, the special macro operators
// `#` and `##` will do the following:
//
// * `#` encloses a param with double quotes, turning
//   it into a string.
// * `##` concatenates params with other elements, in
//   this case it's used to create variable names.

// NOTE: Use `\` to break macros into multiple lines.
// The backslashes aren't substituted with newlines however.
// All lines expanded from the same macro end up as a single
// line.

#define CMD(name)                                                              \
  char name##_cmd[256] = "";                                                   \
  strcpy(name##_cmd, #name);

int main(int argc, char **argv) {
  CMD(copy)
  CMD(paste)
  CMD(put)

  char cmd[256];
  scanf("%s", cmd);

  if (strcmp(cmd, copy_cmd) == 0) {
    // ...
  }

  if (strcmp(cmd, paste_cmd) == 0) {
    // ...
  }

  if (strcmp(cmd, put_cmd) == 0) {
    // ...
  }

  return 0;
}
