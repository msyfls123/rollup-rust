import wasm from '../Cargo.toml';

wasm().then((module) => {
  console.log(module);
  module.greet('kimi');
  module.keys({
    a: 1,
    b: 2,
    kimi: 'me',
  });
});
