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
  file.type = 'file';
  file.accept = 'image/*';
  document.documentElement.appendChild(file);
  const img = document.createElement('img');
  img.id = 'preview';
  img.style.display = 'none';
  document.documentElement.appendChild(img);
  module.init_file(file.id);
});
