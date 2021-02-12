import wasm from '../Cargo.toml';

wasm().then((module) => {
  console.log(module);
  module.greet('kimi');
  module.keys({
    a: 1,
    b: 2,
    kimi: 'me',
  });
  const p = document.createElement('p');
  p.id = 'current-time';
  p.style.color = '#36aefd';
  document.documentElement.appendChild(p);
  module.clock(2000);
});
