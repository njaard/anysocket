This library exposes new types that wrap _either_
`TcpSocket` or `UnixSocket` types.

This library compiles on Windows but doesn't support UnixSocket types.

Before:
```
if addr.starts_with("unix:") {
	let socket = UnixListener::bind(&addr["unix:".len() ..]).expect("binding");
	while let Ok(socket) = socket.accept() {
		// ...
	}
}
else {
	let socket = TcpListener::bind(addr).expect("binding");;
	while let Ok(socket) = socket.accept() {
		// ...
	}
}
```

After:

```
let socket = addr.bind_any().expect("binding");
while let Ok(socket) = socket.accept() {
	// ...
}

```
