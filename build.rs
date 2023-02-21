fn main() {
	let features = std::env::var("CARGO_CFG_TARGET_FEATURE").unwrap();
	if !features.contains("crt-static") {
		panic!("\ntarget-feature={features}\n", features=features);
	}
}

