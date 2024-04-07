arch: update
	@cargo build --release
install: completions
	@install -m 755 target/release/arch /usr/bin/arch
	@install -m 755 target/release/os /usr/bin/os
setup: arch
	@echo "Enter your password in order to refreshing the packages cache"
	@sudo -k target/release/arch --refresh-cache
	@target/release/arch setup
update:
	@git pull origin main
completions: shells
	@install -m 644 arch/completions/arch.fish /etc/fish/completions/arch.fish
	@install -m 644 os/completions/os.fish /etc/fish/completions/os.fish
	@install -m 644 arch/completions/arch.zsh /usr/share/zsh/site-functions/_arch
shells:
	@sudo pacman -S --needed --noconfirm fish zsh zsh-completions
