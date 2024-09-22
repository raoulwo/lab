#include <iostream>

class Rect {
public:
  int area() { return width * length; }
  int width;
  int length;
};

int main(int argc, char **argv) {
  Rect rect;
  rect.width = 10;
  rect.length = 25;

  int area = rect.area();
  std::cout << "area: " << area << '\n';

  return 0;
}
