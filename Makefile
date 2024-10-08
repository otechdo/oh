archlinux:
	@cargo build --release --features archlinux
gentoo:
	@cargo build --release --features gentoo
install:
	@mkdir -p /usr/share/icons/Gemini /var/log/Gemini /usr/share/applications/Gemini
	@install -m 644 ./icons/Gemini.svg /usr/share/icons/Gemini/
	@install -m 644 ./desktop/Gemini.desktop /usr/share/applications/Gemini/Gemini.desktop
	@install -m 755 ./target/release/oh /usr/bin/oh
	@install -m 755 services.sh /usr/local/bin/update-services-cache
	@update-services-cache
