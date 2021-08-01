## 一个简单的 Hello World 日志文件如下
```
2021/08/01 23:32:53 [debug] 2544368#0: epoll: fd:9 ev:0001 d:00007FFFF7FAD010
2021/08/01 23:32:53 [debug] 2544368#0: accept on 0.0.0.0:80, ready: 0
2021/08/01 23:32:53 [debug] 2544368#0: posix_memalign: 0000000000791840:512 @16
2021/08/01 23:32:53 [debug] 2544368#0: *1 accept: 127.0.0.1:33108 fd:11
2021/08/01 23:32:53 [debug] 2544368#0: *1 event timer add: 11: 60000:10480783378
2021/08/01 23:32:53 [debug] 2544368#0: *1 reusable connection: 1
2021/08/01 23:32:53 [debug] 2544368#0: *1 epoll add event: fd:11 op:1 ev:80002001
2021/08/01 23:32:53 [debug] 2544368#0: timer delta: 11714
2021/08/01 23:32:53 [debug] 2544368#0: worker cycle
2021/08/01 23:32:53 [debug] 2544368#0: epoll timer: 60000
2021/08/01 23:34:56 [debug] 2544368#0: epoll: fd:11 ev:2001 d:00007FFFF7FAD100
2021/08/01 23:34:56 [debug] 2544368#0: *1 http wait request handler
2021/08/01 23:34:56 [debug] 2544368#0: *1 malloc: 0000000000776420:1024
2021/08/01 23:34:56 [debug] 2544368#0: *1 recv: eof:1, avail:-1
2021/08/01 23:34:56 [debug] 2544368#0: *1 recv: fd:11 73 of 1024
2021/08/01 23:34:56 [debug] 2544368#0: *1 reusable connection: 0
2021/08/01 23:34:56 [debug] 2544368#0: *1 posix_memalign: 00000000007AE0C0:4096 @16
2021/08/01 23:34:56 [debug] 2544368#0: *1 http process request line
2021/08/01 23:34:56 [debug] 2544368#0: *1 http request line: "GET / HTTP/1.1"
2021/08/01 23:34:56 [debug] 2544368#0: *1 http uri: "/"
2021/08/01 23:34:56 [debug] 2544368#0: *1 http args: ""
2021/08/01 23:34:56 [debug] 2544368#0: *1 http exten: ""
2021/08/01 23:34:56 [debug] 2544368#0: *1 posix_memalign: 00000000007A2420:4096 @16
2021/08/01 23:34:56 [debug] 2544368#0: *1 http process request header line
2021/08/01 23:34:56 [debug] 2544368#0: *1 http header: "Host: localhost"
2021/08/01 23:34:56 [debug] 2544368#0: *1 http header: "User-Agent: curl/7.61.1"
2021/08/01 23:34:56 [debug] 2544368#0: *1 http header: "Accept: */*"
2021/08/01 23:34:56 [debug] 2544368#0: *1 http header done
2021/08/01 23:34:56 [debug] 2544368#0: *1 event timer del: 11: 10480783378
2021/08/01 23:34:56 [debug] 2544368#0: *1 generic phase: 0
2021/08/01 23:34:56 [debug] 2544368#0: *1 rewrite phase: 1
2021/08/01 23:34:56 [debug] 2544368#0: *1 test location: "/"
2021/08/01 23:34:56 [debug] 2544368#0: *1 using configuration "/"
2021/08/01 23:34:56 [debug] 2544368#0: *1 http cl:-1 max:1048576
2021/08/01 23:34:56 [debug] 2544368#0: *1 rewrite phase: 3
2021/08/01 23:34:56 [debug] 2544368#0: *1 post rewrite phase: 4
2021/08/01 23:34:56 [debug] 2544368#0: *1 generic phase: 5
2021/08/01 23:34:56 [debug] 2544368#0: *1 generic phase: 6
2021/08/01 23:34:56 [debug] 2544368#0: *1 generic phase: 7
2021/08/01 23:34:56 [debug] 2544368#0: *1 generic phase: 8
2021/08/01 23:34:56 [debug] 2544368#0: *1 access phase: 9
2021/08/01 23:34:56 [debug] 2544368#0: *1 access phase: 10
2021/08/01 23:34:56 [debug] 2544368#0: *1 access phase: 11
2021/08/01 23:34:56 [debug] 2544368#0: *1 post access phase: 12
2021/08/01 23:34:56 [debug] 2544368#0: *1 generic phase: 13
2021/08/01 23:34:56 [debug] 2544368#0: *1 generic phase: 14
2021/08/01 23:34:56 [debug] 2544368#0: *1 content phase: 15
2021/08/01 23:34:56 [debug] 2544368#0: *1 content phase: 16
2021/08/01 23:34:56 [debug] 2544368#0: *1 open index "/usr/local/nginx/html/index.html"
2021/08/01 23:34:56 [debug] 2544368#0: *1 internal redirect: "/index.html?"
2021/08/01 23:34:56 [debug] 2544368#0: *1 rewrite phase: 1
2021/08/01 23:34:56 [debug] 2544368#0: *1 test location: "/"
2021/08/01 23:34:56 [debug] 2544368#0: *1 test location: "test"
2021/08/01 23:34:56 [debug] 2544368#0: *1 test location: "50x.html"
2021/08/01 23:34:56 [debug] 2544368#0: *1 using configuration "/"
2021/08/01 23:34:56 [debug] 2544368#0: *1 http cl:-1 max:1048576
2021/08/01 23:34:56 [debug] 2544368#0: *1 rewrite phase: 3
2021/08/01 23:34:56 [debug] 2544368#0: *1 post rewrite phase: 4
2021/08/01 23:34:56 [debug] 2544368#0: *1 generic phase: 5
2021/08/01 23:34:56 [debug] 2544368#0: *1 generic phase: 6
2021/08/01 23:34:56 [debug] 2544368#0: *1 generic phase: 7
2021/08/01 23:34:56 [debug] 2544368#0: *1 generic phase: 8
2021/08/01 23:34:56 [debug] 2544368#0: *1 access phase: 9
2021/08/01 23:34:56 [debug] 2544368#0: *1 access phase: 10
2021/08/01 23:34:56 [debug] 2544368#0: *1 access phase: 11
2021/08/01 23:34:56 [debug] 2544368#0: *1 post access phase: 12
2021/08/01 23:34:56 [debug] 2544368#0: *1 generic phase: 13
2021/08/01 23:34:56 [debug] 2544368#0: *1 generic phase: 14
2021/08/01 23:34:56 [debug] 2544368#0: *1 content phase: 15
2021/08/01 23:34:56 [debug] 2544368#0: *1 content phase: 16
2021/08/01 23:34:56 [debug] 2544368#0: *1 content phase: 17
2021/08/01 23:34:56 [debug] 2544368#0: *1 content phase: 18
2021/08/01 23:34:56 [debug] 2544368#0: *1 content phase: 19
2021/08/01 23:34:56 [debug] 2544368#0: *1 content phase: 20
2021/08/01 23:34:56 [debug] 2544368#0: *1 http filename: "/usr/local/nginx/html/index.html"
2021/08/01 23:34:56 [debug] 2544368#0: *1 add cleanup: 00000000007AEE90
2021/08/01 23:34:56 [debug] 2544368#0: *1 http static fd: 12
2021/08/01 23:34:56 [debug] 2544368#0: *1 http set discard body
2021/08/01 23:34:56 [debug] 2544368#0: *1 HTTP/1.1 200 OK
Server: nginx/1.20.0
Date: Sun, 01 Aug 2021 15:34:56 GMT
Content-Type: text/html
Content-Length: 612
Last-Modified: Fri, 14 May 2021 08:44:49 GMT
Connection: keep-alive
ETag: "609e3881-264"
Accept-Ranges: bytes

2021/08/01 23:34:56 [debug] 2544368#0: *1 write new buf t:1 f:0 00000000007A2800, pos 00000000007A2800, size: 238 file: 0, size: 0
2021/08/01 23:34:56 [debug] 2544368#0: *1 http write filter: l:0 f:0 s:238
2021/08/01 23:34:56 [debug] 2544368#0: *1 http output filter "/index.html?"
2021/08/01 23:34:56 [debug] 2544368#0: *1 http copy filter: "/index.html?"
2021/08/01 23:34:56 [debug] 2544368#0: *1 http postpone filter "/index.html?" 00007FFFFFFFDBA0
2021/08/01 23:34:56 [debug] 2544368#0: *1 write old buf t:1 f:0 00000000007A2800, pos 00000000007A2800, size: 238 file: 0, size: 0
2021/08/01 23:34:56 [debug] 2544368#0: *1 write new buf t:0 f:1 0000000000000000, pos 0000000000000000, size: 0 file: 0, size: 612
2021/08/01 23:34:56 [debug] 2544368#0: *1 http write filter: l:1 f:0 s:850
2021/08/01 23:34:56 [debug] 2544368#0: *1 http write filter limit 0
2021/08/01 23:34:56 [debug] 2544368#0: *1 writev: 238 of 238
2021/08/01 23:34:56 [debug] 2544368#0: *1 sendfile: @0 612
2021/08/01 23:34:56 [info] 2544368#0: *1 sendfile() failed (32: Broken pipe) while sending response to client, client: 127.0.0.1, server: localhost, request: "GET / HTTP/1.1", host: "localhost"
2021/08/01 23:34:56 [debug] 2544368#0: *1 http write filter FFFFFFFFFFFFFFFF
2021/08/01 23:34:56 [debug] 2544368#0: *1 http copy filter: -1 "/index.html?"
2021/08/01 23:34:56 [debug] 2544368#0: *1 http finalize request: -1, "/index.html?" a:1, c:2
2021/08/01 23:34:56 [debug] 2544368#0: *1 http terminate request count:2
2021/08/01 23:34:56 [debug] 2544368#0: *1 http terminate cleanup count:2 blk:0
2021/08/01 23:34:56 [debug] 2544368#0: *1 http finalize request: -4, "/index.html?" a:1, c:2
2021/08/01 23:34:56 [debug] 2544368#0: *1 http request count:2 blk:0
2021/08/01 23:34:56 [debug] 2544368#0: *1 http posted request: "/index.html?"
2021/08/01 23:34:56 [debug] 2544368#0: *1 http terminate handler count:1
2021/08/01 23:34:56 [debug] 2544368#0: *1 http request count:1 blk:0
2021/08/01 23:34:56 [debug] 2544368#0: *1 http close request
2021/08/01 23:34:56 [debug] 2544368#0: *1 http log handler
2021/08/01 23:34:56 [debug] 2544368#0: *1 run cleanup: 00000000007AEE90
2021/08/01 23:34:56 [debug] 2544368#0: *1 file cleanup: fd:12
2021/08/01 23:34:56 [debug] 2544368#0: *1 free: 00000000007AE0C0, unused: 40
2021/08/01 23:34:56 [debug] 2544368#0: *1 free: 00000000007A2420, unused: 2592
2021/08/01 23:34:56 [debug] 2544368#0: *1 close http connection: 11
2021/08/01 23:34:56 [debug] 2544368#0: *1 reusable connection: 0
2021/08/01 23:34:56 [debug] 2544368#0: *1 free: 0000000000776420
2021/08/01 23:34:56 [debug] 2544368#0: *1 free: 0000000000791840, unused: 136
2021/08/01 23:34:56 [debug] 2544368#0: timer delta: 122964
2021/08/01 23:34:56 [debug] 2544368#0: worker cycle
2021/08/01 23:34:56 [debug] 2544368#0: epoll timer: -1
```

## 逐步分析
- `ngx_process_events_and_timers -> ngx_process_events` 会开始处理事件，这里 `ngx_process_event` 跟使用的具体事件模块有关系，以 epoll 为例，实际调用的方法为 `ngx_epoll_process_events`，在没有事件的时候会阻塞在 `epoll_wait` 系统调用。
- 请求到达后，最先进行处理的是 `ngx_event_accept` 方法，负责进行 `accept` 系统调用，从 socket 中读取>数据。其初始化在 `ngx_event_process_init` 中，这个函数会把读事件的 handler 设置为 `ngx_event_accept`>。
- 在 `ngx_epoll_add_event` 打断点，得到如下堆栈，可以知道下一个事件由 `ngx_http_init_connection` 注册。
```
#0  ngx_epoll_add_event (ev=0x7c1bb0, event=8193, flags=2147483648)
    at src/event/modules/ngx_epoll_module.c:580
#1  0x0000000000435f55 in ngx_handle_read_event (rev=rev@entry=0x7c1bb0, flags=flags@entry=0)
    at src/event/ngx_event.c:274
#2  0x0000000000458fa5 in ngx_http_init_connection (c=0x7ffff7fad100)
    at src/http/ngx_http_request.c:370
#3  0x0000000000436cc8 in ngx_event_accept (ev=0x7c1b50) at src/event/ngx_event_accept.c:310
#4  0x0000000000440d25 in ngx_epoll_process_events (cycle=0x793c20, timer=<optimized out>,
    flags=<optimized out>) at src/event/modules/ngx_epoll_module.c:901
#5  0x0000000000435e5b in ngx_process_events_and_timers (cycle=cycle@entry=0x793c20)
    at src/event/ngx_event.c:247
#6  0x000000000043fdb3 in ngx_single_process_cycle (cycle=cycle@entry=0x793c20)
    at src/os/unix/ngx_process_cycle.c:300
#7  0x0000000000415589 in main (argc=1, argv=<optimized out>) at src/core/nginx.c:380
```
- 
