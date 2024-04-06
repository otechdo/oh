arch: clean
	@cargo build --release
install: 
	@install -m 755 target/release/arch /usr/bin/arch
clean:
	@cargo clean
setup: arch
	@echo "Enter your password in order to refreshing the packages cache"
	@sudo -k target/release/arch --refresh-cache
	@target/release/arch setup 

