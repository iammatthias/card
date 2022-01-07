# what

Rustlang CLI targeting wasm. runs on node and is deployed with npx/npm.

All content is dynamically pulled from a json api

> This is provided `as is`. This is my first Rust project, and I am sure there are better ways of doing things.

## Building

```
cargo build --target=wasm32-unknown-unknown --release
```

This moves files into:

```
./target/wasm32-unknown-unknown/release/
```

Copy generated `card.js` and `card.wasm` into the root so the [bin](bin.js) can read it:

```
mv ./target/wasm32-unknown-unknown/release/card.* .
```

Done!

Once published the binary can be executed with npx:

```
npx iammatthias
```

Scripts are also discoverable in [package.json](package.json#L9)

## License

MIT

### thanks

Originally based on [https://github.com/rickycodes/card](https://github.com/rickycodes/card)
