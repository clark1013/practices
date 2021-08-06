#include <sys/socket.h>
#include <stdlib.h>
#include <stdio.h>
#include <netinet/in.h>
#include <strings.h>
#include <unistd.h>
#include <errno.h>

void read_data(int sockfd);

int main(int argc, char **argv)
{
  int listenfd, connfd;
  socklen_t clilen;
  struct sockaddr_in cliaddr, servaddr;

  listenfd = socket(AF_INET, SOCK_STREAM, 0);
  
  bzero(&servaddr, sizeof(servaddr));
  servaddr.sin_family = AF_INET;
  servaddr.sin_port = htons(12345);
  servaddr.sin_addr.s_addr = htonl(INADDR_ANY);

  bind(listenfd, (struct sockaddr *) &servaddr, sizeof(servaddr));
  listen(listenfd, 1024);

  for ( ; ; )
  {
    clilen = sizeof(cliaddr);
    connfd = accept(listenfd, (struct sockaddr *) &servaddr, &clilen);
    read_data(connfd);
    close(connfd);
  }
}

ssize_t readn(int sockfd, void *vptr, size_t size)
{
  size_t nleft;
  ssize_t nread;
  char *ptr;

  ptr = vptr;
  nleft = size;

  while (nleft > 0)
  {
    nread = read(sockfd, ptr, size);
    if (nread < 0) {
      if (errno == EINTR)
        nread = 0;
      else
        return -1;
    } else if (nread == 0)
      break;

    nleft -= nread;
    ptr += nread;
  }

  return (size - nleft);
}

void read_data(int sockfd)
{
  ssize_t n;
  char buff[1024];

  int timer = 0;
  for ( ; ; )
  {
    fprintf(stdout, "block in read\n");
    n = readn(sockfd, buff, 1024);
    if (n == 0)
    {
      return;
    }

    timer++;
    fprintf(stdout, "read 1k %d\n", timer);
    usleep(1000);
  }
}
