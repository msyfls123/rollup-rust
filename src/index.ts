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
  module.clock(1000);
  const file = document.createElement('input');
  file.id = 'file-input';
  file.type = 'file'
  document.documentElement.appendChild(file);
  module.init_file(file.id);
});
