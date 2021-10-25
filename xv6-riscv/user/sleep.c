#include "kernel/types.h"
#include "user/user.h"

void main(uint argc, char *argv[]) {
  if (!(argc == 2)) {
    printf("Usage: sleep [time]\n");
    exit(1);
  }
  int t = atoi(argv[1]);
  if (t <= 0) {
    printf("Invalid time\n");
    exit(1);
  }
  sleep(t);
  exit(0);
}
