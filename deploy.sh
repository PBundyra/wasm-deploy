cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/mario_mim.wasm
git add .
git commit -m "New deploy"
git push

