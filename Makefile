SHELLS := $(shell cat /etc/shells)

	.PHONY: oh
oh:
	@clear
	@cargo build --release
	@echo -e "\033[1;32m    Finished\033[1;39m the oh executable has been built successfully\033[39m"
	@sudo mkdir -p /usr/share/applications/oh/services/{root,user}
	@echo -e "\033[1;32m    Finished\033[1;39m the oh applications directory is ready to use\033[39m"
install: completions
	@rm -rf /usr/share/oh
	@echo -e "\033[1;32m    Finished\033[1;39m the oh directory is removed successfully\033[39m"
	@mkdir /usr/share/oh
	@echo -e "\033[1;32m    Finished\033[1;39m the oh directory has been created successfully\033[39m"
	@mkdir /usr/share/oh/cache
	@echo -e "\033[1;32m    Finished\033[1;39m the oh cache directory has been created successfully\033[39m"
	@cp -r oh/profiles /usr/share/oh
	@echo -e "\033[1;32m    Finished\033[1;39m The oh profiles has been installed successfully\033[39m"
	@cp -r oh/conf /usr/share/oh
	@echo -e "\033[1;32m    Finished\033[1;39m configuration files has been installed successfully\033[39m"
	@install -m 644  oh/desktop/oh.desktop /usr/share/applications/oh.desktop
	@echo -e "\033[1;32m    Finished\033[1;39m the oh.desktop is installed successfully\033[39m"
	@install -m 644  oh/desktop/up.desktop /usr/share/applications/up.desktop
	@echo -e "\033[1;32m    Finished\033[1;39m the up.desktop is installed successfully\033[39m"
	@install -m 755 target/release/oh /usr/bin/oh
	@echo -e "\033[1;32m    Finished\033[1;39m the oh executable is ready to use\033[39m"
	@install -m 755 target/release/os /usr/bin/os
	@echo -e "\033[1;32m    Finished\033[1;39m the os executable is ready to use\033[39m"
	@install -m 644  oh/systemd/oh.service /usr/lib/systemd/system/oh.service
	@echo -e "\033[1;32m    Finished\033[1;39m the oh service is installed successfully\033[39m"
	@install -m 644  oh/systemd/oh.timer /usr/lib/systemd/system/oh.timer
	@echo -e "\033[1;32m    Finished\033[1;39m the oh timer is installed successfully\033[39m"
	@install -Dm 644  oh/icons/Up.svg  /usr/share/icons/oh/Up.svg
	@echo -e "\033[1;32m    Finished\033[1;39m the oh upgrade icon is now installed\033[39m"
	@install -Dm 644  oh/icons/Arch.svg  /usr/share/icons/oh/Arch.svg
	@echo -e "\033[1;32m    Finished\033[1;39m the oh icon is now installed\033[39m"
	@install -m 755  oh/extras/up  /usr/bin/up
	@echo -e "\033[1;32m    Finished\033[1;39m the oh upgrade icon is now installed\033[39m"
	@install -m 644  oh/man/oh.1 /usr/share/man/man1
	@echo -e "\033[1;32m    Finished\033[1;39m the oh man icon has been installed successfully\033[39m"
	@install -m 644  oh/man/os.1 /usr/share/man/man1
	@echo -e "\033[1;32m    Finished\033[1;39m the os man icon has been installed successfully\033[39m"
	@install -Dm644 LICENSE "/usr/share/licenses/oh/LICENSE"
	@echo -e "\033[1;32m    Finished\033[1;39m the LICENSE is ready to use\033[39m"
setup:
	@clear
	@target/release/oh --setup
	@echo -e "\033[1;32m    Finished\033[1;39m congratulations\033[39m"
update:
	@echo -e "\033[1;32m    Starting\033[1;39m updating the repository\033[30m"
	@git pull --quiet origin main || exit 1
	@echo -e "\033[1;32m    Finished\033[1;39m repository updated successfully\033[30m"
completions:
ifeq ($(findstring fish,$(SHELLS)),fish)
	@echo -e "\033[1;32m    Finished\033[1;39m completions installation for fish has been started\033[30m"
	@install -m 644 oh/completions/oh.fish /etc/fish/completions/oh.fish
	@echo -e "\033[1;32m    Finished\033[1;39m completions installation for oh executable\033[30m"
	@install -m 644 os/completions/os.fish /etc/fish/completions/os.fish
	@echo -e "\033[1;32m    Finished\033[1;39m completions installation for os executable\033[30m"
	@echo -e "\033[1;32m    Finished\033[1;39m completions installation finnish for fish\033[30m"
endif
ifeq ($(findstring zsh,$(SHELLS)),zsh)
	@echo -e "\033[1;32m    Finished\033[1;39m completions installation for zsh has been started\033[30m"
	@echo -e "\033[1;32m    Finished\033[1;39m completions installation for oh executable\033[30m"
	@install -m 644 oh/completions/oh.zsh /usr/share/zsh/site-functions/_oh
	@echo -e "\033[1;32m    Finished\033[1;39m completions installation finnish for zsh\033[30m"
endif