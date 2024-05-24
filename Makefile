TIME:=2
oh:
	@clear
	@echo -e "\033[1;32m    Starting\033[1;39m checking source code\033[39m"
	@sleep $(TIME)
	@zuu
	@echo -e "\033[1;32m    Finished\033[1;39m checking source code\033[39m"
	@sleep $(TIME)
	@echo -e "\033[1;32m    Starting\033[1;39m building executable from source code\033[39m"
	@cargo build --release
	@echo -e "\033[1;32m    Finished\033[1;39m the oh executable has been built successfully\033[39m"
	@sleep $(TIME)
	@echo -e "\033[1;32m    Starting\033[1;39m checking desktop file\033[39m"
	@sleep $(TIME)
	@desktop-file-validate ./desktop/oh.desktop
	@echo -e "\033[1;32m    Finished\033[1;39m the oh desktop file is valid\033[39m"
	@desktop-file-validate ./desktop/up.desktop
	@echo -e "\033[1;32m    Finished\033[1;39m the up desktop file is valid\033[39m"
	@echo -e "\033[1;32m    Finished\033[1;39m All desktop file are valid\033[39m"
	@sleep $(TIME)
install:
	@rsync -a ./icons/ /usr/share/icons/
	@echo -e "\033[1;32m    Finished\033[1;39m the icons files has been rsync successfully\033[39m"
	@rsync -a ./desktop/ /usr/share/applications/
	@echo -e "\033[1;32m    Finished\033[1;39m the desktop files has been rsync successfully\033[39m"
	@rsync -a ./fish/ /etc/fish/completions/
	@echo -e "\033[1;32m    Finished\033[1;39m the completions files has been rsync successfully\033[39m"
	@install -Dm644 LICENSE "/usr/share/licenses/oh/LICENSE"
	@echo -e "\033[1;32m    Finished\033[1;39m the LICENSE is ready to use\033[39m"
	@install -m 755 target/release/oh "/usr/bin/oh"
	@echo -e "\033[1;32m    Finished\033[1;39m the program is ready to use\033[39m"
	@update-desktop-database  /usr/share/applications/
setup: install
	@clear
	@target/release/oh --setup
	@echo -e "\033[1;32m    Finished\033[1;39m congratulations\033[39m"
update:
	@echo -e "\033[1;32m    Starting\033[1;39m updating the repository\033[30m"
	@git pull --quiet origin main || exit 1
	@echo -e "\033[1;32m    Finished\033[1;39m repository updated successfully\033[30m"
