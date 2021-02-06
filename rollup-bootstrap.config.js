import rust from "@wasm-tool/rollup-plugin-rust";

export default {
    input: {
        dummy: "./Cargo.toml",
    },
    plugins: [
        rust(),
    ],
};
