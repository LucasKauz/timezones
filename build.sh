wasm-pack build --target web --out-name wasm --out-dir ./static
# this will fail the second time, but we don't need it to succeed
# I need to find a better way to do it
miniserve ./static --index index.html