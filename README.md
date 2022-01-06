# what

rustlang cli targetting wasm. runs on node and is deployed with npx/npm.

all content is dynamically pulled from a json api

## Building

```
cargo build --target=wasm32-unknown-emscripten --release
```

This moves files into:

```
./target/wasm32-unknown-emscripten/release/
```

Copy generated `card.js` and `card.wasm` into the root so the [bin](bin.js) can read it:

```
mv ./target/wasm32-unknown-emscripten/release/card.* .
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
