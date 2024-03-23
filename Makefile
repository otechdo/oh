pkgs:
	@pacman -Sl core  | cut -d " "   -f 2 > pkgs
	@pacman -Sl extra | cut -d " "   -f 2 >> pkgs
	@paru --list aur  | cut -d " "   -f 2 >> pkgs