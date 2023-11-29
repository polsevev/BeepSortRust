FROM nginx:alpine3.18
COPY ./index.html /usr/share/nginx/html/
COPY ./target/wasm32-unknown-unknown/release/BeepSortMacroQuad.wasm /usr/share/nginx/html/
