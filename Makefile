SHELLS := $(shell cat /etc/shells)

arch: update
	@cargo build --release
	@echo -e "\033[1;32m    Finished\033[1;39m the arch executable has been builded successfully\033[39m"
install: completions
	@install -m 644  arch/systemd/arch.service /usr/lib/systemd/system/arch.service
	@echo -e "\033[1;32m    Finished\033[1;39m the arch service is installed successfully\033[39m"
	@install -m 644  arch/systemd/arch.timer /usr/lib/systemd/system/arch.timer
	@echo -e "\033[1;32m    Finished\033[1;39m the arch timer is installed successfully\033[39m"
	@target/release/arch --cache > /dev/null || @echo -e "\033[1;32m    Finished\033[1;39m the arch cache is not configured successfully\033[39m"
	@echo -e "\033[1;32m    Finished\033[1;39m the arch packages cache has been successfully created\033[39m"
	@install -m 755 target/release/arch /usr/bin/arch
	@echo -e "\033[1;32m    Finished\033[1;39m the arch executable is ready to use\033[39m"
	@install -m 755 target/release/os /usr/bin/os
	@echo -e "\033[1;32m    Finished\033[1;39m the os executable is ready to use\033[39m"
setup: arch
	@echo -e "\033[1;32m    Finished\033[1;39m enter your password in order to refreshing the packages cache\033[39m"
	@sudo -k target/release/arch --cache
	@echo -e "\033[1;32m    Finished\033[1;39m package cache generated successfully\033[39m"
	@echo -e "\033[1;32m    Finished\033[1;39m setup is starting\033[39m"
	@target/release/arch --setup
	@clear
	@echo -e "\033[1;32m    Finished\033[1;39m congratulations\033[39m"
update:
	@echo -e "\033[1;32m    Starting\033[1;39m updating the repository\033[30m"
	@git pull --quiet origin main || exit 1
	@echo -e "\033[1;32m    Finished\033[1;39m repository updated successfully\033[30m"
completions:
ifeq ($(findstring fish,$(SHELLS)),fish)
	@echo -e "\033[1;32m    Finished\033[1;39m completions installation for fish has been started\033[30m"
	@install -m 644 arch/completions/arch.fish /etc/fish/completions/arch.fish
	@echo -e "\033[1;32m    Finished\033[1;39m completions installation for arch executable\033[30m"
	@install -m 644 os/completions/os.fish /etc/fish/completions/os.fish
	@echo -e "\033[1;32m    Finished\033[1;39m completions installation for os executable\033[30m"
	@echo -e "\033[1;32m    Finished\033[1;39m completions installation finnish for fish\033[30m"
endif
ifeq ($(findstring zsh,$(SHELLS)),zsh)
	@echo -e "\033[1;32m    Finished\033[1;39m completions installation for zsh has been started\033[30m"
	@echo -e "\033[1;32m    Finished\033[1;39m completions installation for arch executable\033[30m"
	@install -m 644 arch/completions/arch.zsh /usr/share/zsh/site-functions/_arch
	@echo -e "\033[1;32m    Finished\033[1;39m completions installation finnish for zsh\033[30m"
endif

