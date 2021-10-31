#include "kernel/types.h"
#include "user/user.h"

void main(int argc, char *argv[]) {
  int p[2];

  pipe(p);
  if (fork() == 0) {
    close(0);
    dup(p[0]);
    char child[4];
    read(p[0], child, 4);
    printf("%d: received %s\n", getpid(), child);
    write(p[1], "pong", 4);
    close(p[1]);
    close(p[0]);
  } else {
    close(0);
    dup(p[0]);
    write(p[1], "ping", 4);
    char child[4];
    read(p[0], child, 4);
    printf("%d: received %s\n", getpid(), child);
    close(p[1]);
    close(p[0]);
  }
  wait(0);
  exit(0);
}
