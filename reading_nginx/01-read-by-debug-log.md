## 目标配置
```
location /test {
    set $foo world;
    echo "hello $foo";
}
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
- 追踪 `ngx_http_init_connection` 的调用链可知其在 `ngx_http_add_listening` 中被赋值给了 `ls->handler`，其最上层的函数为 `ngx_http_block`，即在 `http block` 被初始化时会注册其 `handler`。
- `ngx_http_init_connection` 在执行的时，会将下一个事件的 `handler` 设置为 `ngx_http_wait_request_handler`，即为下一个事件的入口
- `ngx_http_wait_request_handler`，下一个 handler 为 `ngx_http_process_request_line`
- `ngx_http_process_request_line` 会读取 http 的请求行，类似于 `"GET / HTTP/1.1"`。并会调用 `ngx_http_process_request_uri` 解析 uri 中的参数。下一个 handler 为 `ngx_http_process_request_headers`。
- `ngx_http_process_request_headers` 会读取所有 http 头部。并会调用 `ngx_http_process_request -> ngx_http_handler`，下一个 handler 为 `ngx_http_core_run_phases`。
- `ngx_http_core_run_phases` 会调用每个 `phase_handler` 的 `checker` 方法。`checker` 在 `ngx_http_init_phase_handlers` 中注册。
- `checker` 的调用实际上就是在执行 nginx 的 13 个阶段，所有模块都会将自己的 `handler` 注册到这 13 个阶段从而完成模块的功能。


## 一个简单的 Hello World 日志文件如下
```
2021/08/11 23:34:24 [debug] 3584510#0: epoll: fd:8 ev:0001 d:00007FE60C52F010
2021/08/11 23:34:24 [debug] 3584510#0: accept on 0.0.0.0:80, ready: 0
2021/08/11 23:34:24 [debug] 3584510#0: posix_memalign: 00000000009CC840:512 @16
2021/08/11 23:34:24 [debug] 3584510#0: *1 accept: 111.203.244.2:60561 fd:11
2021/08/11 23:34:24 [debug] 3584510#0: *1 event timer add: 11: 60000:11344881917
2021/08/11 23:34:24 [debug] 3584510#0: *1 reusable connection: 1
2021/08/11 23:34:24 [debug] 3584510#0: *1 epoll add event: fd:11 op:1 ev:80002001
2021/08/11 23:34:24 [debug] 3584510#0: timer delta: 29398
2021/08/11 23:34:24 [debug] 3584510#0: worker cycle
2021/08/11 23:34:24 [debug] 3584510#0: epoll timer: 60000
2021/08/11 23:34:24 [debug] 3584510#0: epoll: fd:11 ev:0001 d:00007FE60C52F1F0
2021/08/11 23:34:24 [debug] 3584510#0: *1 http wait request handler
2021/08/11 23:34:24 [debug] 3584510#0: *1 malloc: 00000000009B1420:1024
2021/08/11 23:34:24 [debug] 3584510#0: *1 recv: eof:0, avail:-1
2021/08/11 23:34:24 [debug] 3584510#0: *1 recv: fd:11 79 of 1024
2021/08/11 23:34:24 [debug] 3584510#0: *1 reusable connection: 0
2021/08/11 23:34:24 [debug] 3584510#0: *1 posix_memalign: 00000000009DD420:4096 @16
2021/08/11 23:34:24 [debug] 3584510#0: *1 http process request line
2021/08/11 23:34:24 [debug] 3584510#0: *1 http request line: "GET /test HTTP/1.1"
2021/08/11 23:34:24 [debug] 3584510#0: *1 http uri: "/test"
2021/08/11 23:34:24 [debug] 3584510#0: *1 http args: ""
2021/08/11 23:34:24 [debug] 3584510#0: *1 http exten: ""
2021/08/11 23:34:24 [debug] 3584510#0: *1 posix_memalign: 00000000009D2BE0:4096 @16
2021/08/11 23:34:24 [debug] 3584510#0: *1 http process request header line
2021/08/11 23:34:24 [debug] 3584510#0: *1 http header: "Host: kube-master"
2021/08/11 23:34:24 [debug] 3584510#0: *1 http header: "User-Agent: curl/7.78.0"
2021/08/11 23:34:24 [debug] 3584510#0: *1 http header: "Accept: */*"
2021/08/11 23:34:24 [debug] 3584510#0: *1 http header done
2021/08/11 23:34:24 [debug] 3584510#0: *1 event timer del: 11: 11344881917
2021/08/11 23:34:24 [debug] 3584510#0: *1 generic phase: 0
2021/08/11 23:34:24 [debug] 3584510#0: *1 rewrite phase: 1
2021/08/11 23:34:24 [debug] 3584510#0: *1 test location: "/"
2021/08/11 23:34:24 [debug] 3584510#0: *1 test location: "test"
2021/08/11 23:34:24 [debug] 3584510#0: *1 using configuration "/test"
2021/08/11 23:34:24 [debug] 3584510#0: *1 http cl:-1 max:1048576
2021/08/11 23:34:24 [debug] 3584510#0: *1 rewrite phase: 3
2021/08/11 23:34:24 [debug] 3584510#0: *1 http script value: "world"
2021/08/11 23:34:24 [debug] 3584510#0: *1 http script set $foo
2021/08/11 23:34:24 [debug] 3584510#0: *1 post rewrite phase: 4
2021/08/11 23:34:24 [debug] 3584510#0: *1 generic phase: 5
2021/08/11 23:34:24 [debug] 3584510#0: *1 generic phase: 6
2021/08/11 23:34:24 [debug] 3584510#0: *1 generic phase: 7
2021/08/11 23:34:24 [debug] 3584510#0: *1 generic phase: 8
2021/08/11 23:34:24 [debug] 3584510#0: *1 access phase: 9
2021/08/11 23:34:24 [debug] 3584510#0: *1 access phase: 10
2021/08/11 23:34:24 [debug] 3584510#0: *1 access phase: 11
2021/08/11 23:34:24 [debug] 3584510#0: *1 post access phase: 12
2021/08/11 23:34:24 [debug] 3584510#0: *1 generic phase: 13
2021/08/11 23:34:24 [debug] 3584510#0: *1 generic phase: 14
2021/08/11 23:34:24 [debug] 3584510#0: *1 http script copy: "hello "
2021/08/11 23:34:24 [debug] 3584510#0: *1 http script var: "world"
2021/08/11 23:34:24 [debug] 3584510#0: *1 HTTP/1.1 200 OK
Server: nginx/1.20.0
Date: Wed, 11 Aug 2021 15:34:24 GMT
Content-Type: application/octet-stream
Transfer-Encoding: chunked
Connection: keep-alive

2021/08/11 23:34:24 [debug] 3584510#0: *1 write new buf t:1 f:0 00000000009D3090, pos 00000000009D3090, size: 170 file: 0, size: 0
2021/08/11 23:34:24 [debug] 3584510#0: *1 http write filter: l:0 f:0 s:170
2021/08/11 23:34:24 [debug] 3584510#0: *1 http output filter "/test?"
2021/08/11 23:34:24 [debug] 3584510#0: *1 http copy filter: "/test?"
2021/08/11 23:34:24 [debug] 3584510#0: *1 http postpone filter "/test?" 00000000009D31D8
2021/08/11 23:34:24 [debug] 3584510#0: *1 http chunk: 11
2021/08/11 23:34:24 [debug] 3584510#0: *1 http chunk: 1
2021/08/11 23:34:24 [debug] 3584510#0: *1 write old buf t:1 f:0 00000000009D3090, pos 00000000009D3090, size: 170 file: 0, size: 0
2021/08/11 23:34:24 [debug] 3584510#0: *1 write new buf t:1 f:0 00000000009D3278, pos 00000000009D3278, size: 3 file: 0, size: 0
2021/08/11 23:34:24 [debug] 3584510#0: *1 write new buf t:0 f:0 00000000009DE3C0, pos 00000000009DE3C0, size: 11 file: 0, size: 0
2021/08/11 23:34:24 [debug] 3584510#0: *1 write new buf t:0 f:0 000000000074CF08, pos 000000000074CF08, size: 1 file: 0, size: 0
2021/08/11 23:34:24 [debug] 3584510#0: *1 write new buf t:0 f:0 0000000000000000, pos 00000000004FDBF1, size: 2 file: 0, size: 0
2021/08/11 23:34:24 [debug] 3584510#0: *1 http write filter: l:0 f:0 s:187
2021/08/11 23:34:24 [debug] 3584510#0: *1 http copy filter: 0 "/test?"
2021/08/11 23:34:24 [debug] 3584510#0: *1 http output filter "/test?"
2021/08/11 23:34:24 [debug] 3584510#0: *1 http copy filter: "/test?"
2021/08/11 23:34:24 [debug] 3584510#0: *1 http postpone filter "/test?" 00000000009D3380
2021/08/11 23:34:24 [debug] 3584510#0: *1 http chunk: 0
2021/08/11 23:34:24 [debug] 3584510#0: *1 write old buf t:1 f:0 00000000009D3090, pos 00000000009D3090, size: 170 file: 0, size: 0
2021/08/11 23:34:24 [debug] 3584510#0: *1 write old buf t:1 f:0 00000000009D3278, pos 00000000009D3278, size: 3 file: 0, size: 0
2021/08/11 23:34:24 [debug] 3584510#0: *1 write old buf t:0 f:0 00000000009DE3C0, pos 00000000009DE3C0, size: 11 file: 0, size: 0
2021/08/11 23:34:24 [debug] 3584510#0: *1 write old buf t:0 f:0 000000000074CF08, pos 000000000074CF08, size: 1 file: 0, size: 0
2021/08/11 23:34:24 [debug] 3584510#0: *1 write old buf t:0 f:0 0000000000000000, pos 00000000004FDBF1, size: 2 file: 0, size: 0
2021/08/11 23:34:24 [debug] 3584510#0: *1 write new buf t:0 f:0 0000000000000000, pos 00000000004FDBEE, size: 5 file: 0, size: 0
2021/08/11 23:34:24 [debug] 3584510#0: *1 http write filter: l:1 f:0 s:192
2021/08/11 23:34:24 [debug] 3584510#0: *1 http write filter limit 0
2021/08/11 23:34:24 [debug] 3584510#0: *1 writev: 192 of 192
2021/08/11 23:34:24 [debug] 3584510#0: *1 http write filter 0000000000000000
2021/08/11 23:34:24 [debug] 3584510#0: *1 http copy filter: 0 "/test?"
2021/08/11 23:34:24 [debug] 3584510#0: *1 http set discard body
2021/08/11 23:34:24 [debug] 3584510#0: *1 http finalize request: 0, "/test?" a:1, c:1
2021/08/11 23:34:24 [debug] 3584510#0: *1 set http keepalive handler
2021/08/11 23:34:24 [debug] 3584510#0: *1 http close request
2021/08/11 23:34:24 [debug] 3584510#0: *1 http log handler
2021/08/11 23:34:24 [debug] 3584510#0: *1 free: 00000000009DD420, unused: 0
2021/08/11 23:34:24 [debug] 3584510#0: *1 free: 00000000009D2BE0, unused: 1888
2021/08/11 23:34:24 [debug] 3584510#0: *1 free: 00000000009B1420
2021/08/11 23:34:24 [debug] 3584510#0: *1 hc free: 0000000000000000
2021/08/11 23:34:24 [debug] 3584510#0: *1 hc busy: 0000000000000000 0
2021/08/11 23:34:24 [debug] 3584510#0: *1 tcp_nodelay
2021/08/11 23:34:24 [debug] 3584510#0: *1 reusable connection: 1
2021/08/11 23:34:24 [debug] 3584510#0: *1 event timer add: 11: 65000:11344886919
2021/08/11 23:34:24 [debug] 3584510#0: timer delta: 2
2021/08/11 23:34:24 [debug] 3584510#0: worker cycle
2021/08/11 23:34:24 [debug] 3584510#0: epoll timer: 65000
2021/08/11 23:34:24 [debug] 3584510#0: epoll: fd:11 ev:2001 d:00007FE60C52F1F0
2021/08/11 23:34:24 [debug] 3584510#0: *1 http keepalive handler
2021/08/11 23:34:24 [debug] 3584510#0: *1 malloc: 00000000009B1420:1024
2021/08/11 23:34:24 [debug] 3584510#0: *1 recv: eof:1, avail:-1
2021/08/11 23:34:24 [debug] 3584510#0: *1 recv: fd:11 0 of 1024
2021/08/11 23:34:24 [info] 3584510#0: *1 client 111.203.244.2 closed keepalive connection
2021/08/11 23:34:24 [debug] 3584510#0: *1 close http connection: 11
2021/08/11 23:34:24 [debug] 3584510#0: *1 event timer del: 11: 11344886919
2021/08/11 23:34:24 [debug] 3584510#0: *1 reusable connection: 0
2021/08/11 23:34:24 [debug] 3584510#0: *1 free: 00000000009B1420
2021/08/11 23:34:24 [debug] 3584510#0: *1 free: 00000000009CC840, unused: 136
2021/08/11 23:34:24 [debug] 3584510#0: timer delta: 31
2021/08/11 23:34:24 [debug] 3584510#0: worker cycle
2021/08/11 23:34:24 [debug] 3584510#0: epoll timer: -1
```
