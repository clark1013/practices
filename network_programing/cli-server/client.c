#include <sys/socket.h>
#include <stdio.h>
#include <arpa/inet.h>
#include <strings.h>
#include <string.h>
#include <stdlib.h>

#define MESSAGE_SIZE 10240000

void send_data(int sockfd)
{
  char *query;
  query = malloc(MESSAGE_SIZE + 1);
  for (int i = 0; i < MESSAGE_SIZE; i++)
  {
    query[i] = "A";
  }
  query[MESSAGE_SIZE] = "\0";

  char *cp;
  cp = query;
  size_t remain = strlen(query);
  
  while (remain > 0)
  {
    ssize_t nwrite = send(sockfd, cp, remain, 0);
    fprintf(stdout, "write buffer %ld\n", nwrite);
    if (nwrite < 0) {
      perror("send error");
      return;
    }
    remain -= nwrite;
    cp += nwrite;
  }
  return;
}

int main(int argc, char **argv)
{
  if (argc != 2) {
    perror("Usage: client <ip addr>");
    exit(1);
  }

  int sockfd;
  struct sockaddr_in servaddr;

  sockfd = socket(AF_INET, SOCK_STREAM, 0);

  bzero(&servaddr, sizeof(servaddr));
  servaddr.sin_family = AF_INET;
  servaddr.sin_port = htons(12345);
  inet_pton(AF_INET, argv[1], &servaddr.sin_addr);
  connect(sockfd, (struct sinaddr *) &servaddr, sizeof(servaddr));
  send_data(sockfd);
  exit(0);
}
