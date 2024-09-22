#include <stdio.h>

typedef struct {
  int width;
  int length;
} rect_t;

int rect_area(rect_t *rect) { return rect->width * rect->length; }

int main(int argc, char **argv) {
  rect_t rect;
  rect.width = 10;
  rect.length = 25;

  int area = rect_area(&rect);
  printf("area: %d\n", area);

  return 0;
}
