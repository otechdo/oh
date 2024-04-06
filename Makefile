arch: clean
	cargo build --release
install: 
	install -m 755 target/release/arch /usr/bin/arch
clean:
	cargo clean
