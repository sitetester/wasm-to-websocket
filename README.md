
Check relevant `README.md` files in /client & /server

It's a Rust library that compiles to Wasm and exports a function 
`wsPing(endpoint: string, message: string): Promise<string>` (Typescript syntax)

This function establishes a web socket connection to some "endpoint" and send the text message, receive a message, and return its content.

Check [Running test](https://github.com/sitetester/wasm-to-websocket/tree/main/client#running-test) on how to run relevant tests.  Screenshots: [Web](https://github.com/sitetester/wasm-to-websocket/tree/main/client/web/README.md) & [Node](https://github.com/sitetester/wasm-to-websocket/tree/main/client/tests/README.md)
### Why such workspace setup ? 
- It has conflict with tokio `rt-multi-thread` feature, which is being used in `/server` for running Websocket server
- For websocket server, currently there is only a single file, it's better to put it in same repository. 
