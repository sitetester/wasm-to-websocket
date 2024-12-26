### Setup
`cd client` (make sure to be inside `/client` directory)

**Install dependencies**
- `npm install ws`
- `npm install -D typescript ts-node @types/node @types/ws` (Development dependencies)

**Build WASM package**
- `cargo install wasm-pack` (if not already installed)
- `wasm-pack build --target nodejs` 

### Running test
First run the server (check `server/README.md`)

2 types of tests are provided. One in `/tests/ws_ping_test.ts` & other `/web/static/index.html`
Check relevant `README.md` files in corresponding `/tests` & `/web` directories on how to setup/run those tests. 

---
### Helpful points 
**crate-type = ["cdylib"]**
- C Dynamic Library
- Creates a shared library that follows C-language conventions (C ABI)
- The library can be loaded by other languages
- Perfect for WebAssembly because browsers and Node.js need to load and run your code
- Output Format (Windows: .dll, Linux: .so, WebAssembly: .wasm, macOS: .dylib files)

**Why not dylib ?**
- Designed for Rust-to-Rust usage
- Contains Rust-specific metadata and symbols (`#[cfg(feature = "...")]`, `#[test]`, `#[inline]`, `#[repr(C)]`, ...)
- Includes Rust runtime dependencies


### WebAssembly
- A low-level assembly language for the web
- Binary code format similar to machine code
- Can be compiled from multiple languages
- Used for CPU-intensive computations (gaming, video/audio processing, image manipulation, cryptography, ...)

**Features**
- Speed: Faster than equivalent JavaScript
- Safety: runs in a protected environment
- Compatibility: Works in any modern browser or Node.js
- Interoperability: Rust and JavaScript can work together (bi-directional communication)

---

### `wasm-pack build --target nodejs`
A specialized build process that compiles Rust code into WebAssembly specifically optimized  for Node.js environments.  

This command generates a `/pkg` directory with contents similar to:  
```
pkg/
├── wasm_client.js        # Node.js-specific JavaScript bindings
├── wasm_client_bg.wasm   # Optimized WebAssembly module
├── wasm_client.d.ts      # TypeScript definitions
└── package.json          # Configured for Node.js usage
```
- Optimizes the WebAssembly binary (.wasm)
- Uses Node.js fs module to read .wasm binary
- Creates WbAssembly module instance
- Maps Rust functions to JavaScript
- Creates require/exports style modules

### `cargo build --target wasm32-unknown-unknown` 
  - wasm32: 32-bit WebAssembly architecture
  - unknown: No specific operating system
  - unknown: No specific environment
- Creates raw WebAssembly binary
- No JavaScript bindings generated
- This command alone doesn't create web-ready code
- `wasm-pack build --target nodejs` internally calls `cargo build --target wasm32-unknown-unknown --release` as part of its build process
  

