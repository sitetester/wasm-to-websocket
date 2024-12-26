// The ws package provides WebSocket functionality for Node.js
import WebSocket from 'ws';
// Allow either `ws` or browser WebSocket (WASM code expects a browser-like WebSocket)
// We put it in the global scope to make the WASM code work as if it were in a browser
(global as any).WebSocket = WebSocket;

import {ws_ping} from '../pkg';

const message = "Hello from TypeScript";
console.log("Sending:", message);

ws_ping("ws://127.0.0.1:8081", message)
    .then((result: string) => {
        console.log("Response:", result);
    })
    .catch((error: Error) => {
        console.error("Error:", error);
        process.exit(1);
    });

