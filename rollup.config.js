import typescript from '@rollup/plugin-typescript'
import html from '@rollup/plugin-html'
import serve from 'rollup-plugin-serve'
import livereload from 'rollup-plugin-livereload'
import rust from '@wasm-tool/rollup-plugin-rust'
import { nodeResolve } from '@rollup/plugin-node-resolve'
import commonjs from '@rollup/plugin-commonjs'

const hot = process.env.NODE_ENV === 'development'

const plugins = [
  typescript(),
  nodeResolve({
    preferBuiltins: true,
    extensions: ['.js', '.json']
  }),
  commonjs({
    include: /node_modules/,
  }),
  rust(),
  html(),
]

if (hot) {
  plugins.push(
    serve({
      contentBase: './dist',
      open: true
    }),
    livereload({ watch: "./dist" })
  );
}

export default {
  input: 'src/index.ts',
  output: {
    dir: "dist",
    format: "iife",
    sourcemap: hot,
  },
  treeshake: !hot,
  plugins,
}
