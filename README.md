This is a sample project using [Quinn] to communicate over QUIC locally (with a
self-signed certificate).

In particular, it highlights that the server does not know that a new stream is
created by the client until the client sends data:

```console
$ target/debug/server
[server] [2117ms] connection accepted: addr=127.0.0.1:49668
[server] [2117ms] waiting stream opening...
[server] [7119ms] stream opened
```

```console
$ target/debug/client
[client] [15ms] connected: addr=127.0.0.1:1234
[client] [15ms] stream opened
[client] [5017ms] data sent
```

[Quinn]: https://github.com/quinn-rs/quinn
