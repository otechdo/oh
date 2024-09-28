gemini:
	@cargo build --release --features full
install:
	@mkdir -p /usr/share/icons/Gemini /var/log/Gemini /usr/share/applications/Gemini
	@install -m 644 ./icons/Gemini.svg /usr/share/icons/Gemini/
	@install -m 644 ./desktop/Gemini.desktop /usr/share/applications/Gemini/Gemini.desktop
	@install -m 755 ./target/release/oh /usr/bin/oh


