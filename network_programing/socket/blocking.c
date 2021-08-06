#include <stdlib.h>
#include <stdio.h>
#include <sys/socket.h>
#include <netinet/in.h>

int make_socket(uint16_t port)
{
  int sock;
  struct sockaddr_in name;

  sock = socket(AF_INET, SOCK_STREAM, 0);
  if (sock < 0) 
  {
    perror("socket");
    exit(EXIT_FAILURE);
  }

  name.sin_family = AF_INET;
  name.sin_port = htons(port);
  name.sin_addr.s_addr = htonl(INADDR_ANY);

  if (bind(sock, (struct sockaddr *) &name, sizeof(name)) < 0)
  {
    perror("bind");
    exit(EXIT_FAILURE);
  }
}


int main()
{
  make_socket(13245);
}
