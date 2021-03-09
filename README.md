rollup-rust
===

[Live Demo](https://msyfls123.github.io/rollup-rust/)

## Install

__If you can't run this project following normal `npm` steps, try codes below.__

```shell
npm i
node
```
```nodejs
var installResult = require('wasm-pack').install().then(console.log);
.exit
```
```shell
cd ~/Library/Preferences/wasm-pack-nodejs/bin
rm wasm-pack
ln -s $HOME/.cargo/bin/wasm-pack wasm-pack
cd -
sed '/--log-level/d' node_modules/@wasm-tool/rollup-plugin-rust/index.js
```

Then have fun!

## Run & Build
- Run: `npm start`
- Build: `npm run build`
