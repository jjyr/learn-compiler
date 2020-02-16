#include <stdio.h>

int read_int() {
  int c;
  int n = 0;
  while (1) {
    c = getchar();
    if (c < '0' || c > '9')
      break;
    n = n * 10 + (c - '0');
  }
  return n;
}

int print_int(int i) { printf("%d", i); }
