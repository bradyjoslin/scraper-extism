build:
	cargo build --release --target wasm32-unknown-unknown

test:
	extism call ./target/wasm32-unknown-unknown/release/scraper_extism.wasm scraper --input='{"html": "<ul> <li>foo</li> <li>bar</li> </ul><p class='moon'>test text</p>", "selector": ".moon"}' --log-level=info