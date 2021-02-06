export * from "./target/wasm-pack/rollup-rust/index";

type Exports = typeof import("./target/wasm-pack/rollup-rust/index");
declare const init: () => Promise<Exports>;
export default init;
