# Installation

## With AI

```bash
cargo install oh --features ai
```

## With prompts without AI

```bash
cargo install oh --features ask
```

## Without AI and prompt

```bash
cargo install oh --features bar
```

## All features

```bash
cargo install oh --features full
```

## Run

```bash
oh --help
```

```bash
API_KEY="gemini api key here" oh
```

* [Get api key](https://aistudio.google.com/app/apikey)

### Command options

**Base Options**

* `--distro="archlinux"`:
    * Specifies the Linux distribution to install (Arch Linux).

* `--install-requirements=true`:
    * Automatically installs packages needed for the installation process itself.

* `--bootloader="systemd-boot"`:
    * Sets `systemd-boot` as the bootloader.
    * **Alternatives:** `grub` (more feature-rich but potentially more complex)

* `--boot=/dev/sda1`:
    * Specifies the partition where the bootloader will be installed.
    * **Consideration:** Ensure this is a separate partition, typically formatted as FAT32 for UEFI systems.

* `--efi=/dev/sda2`:
    * Specifies the EFI system partition (required for UEFI systems).
    * **Consideration:** This should also be a separate FAT32 partition.

* `--root=/dev/sda3`:
    * Sets the root partition for the system.
    * **Consideration:** This is where the core operating system files will reside.

* `--home=/dev/sda4`:
    * Sets the `/home` partition for user data.
    * **Consideration:** Separating `/home` allows for easier system re-installations without losing user data.

* `--swap=/dev/sda5`:
    * Specifies the partition to be used for swap space.
    * **Consideration:** Swap size recommendations vary, but generally 1-2 times the amount of RAM is a good starting point. With modern systems and ample RAM, swap might not be necessary.

* `--boot-directory=/boot`:
    * Sets the boot directory.
    * **Consideration:** Usually located within the root partition (`/`) but can be a separate partition if desired.

* `--progress=bar`:
    * Displays a progress bar during the installation.
    * **Alternative**: `--progress=dialog` for a dialog-based progress indicator.

* `--lang="fr_FR.UTF-8"`:
    * Sets the system language to French with UTF-8 encoding.

* `--filesystem="xfs"`:
    * Uses the XFS filesystem for the root partition.
    * **Alternatives**: Consider `ext4` for a widely supported and stable option, or `btrfs` for advanced features like snapshots and subvolumes.

* `--keymap="fr"`:
    * Configures the keyboard layout to French.
    * **Consideration:** Adjust to your actual keyboard layout.

* `--timezone="Europe/Paris"`:
    * Sets the timezone to Paris.
    * **Consideration:** Adjust to your actual timezone.

* `--hostname="mon-arch"`: Sets the hostname of the machine.

* `--user="monutilisateur"`:
    * Creates a standard user named "monutilisateur".
    * **Consideration**: Choose a secure username.

* `--group="wheel audio video"`:
    * Adds the created user to the `wheel`, `audio`, and `video` groups.
    * **Consideration**: `wheel` allows the user to use `sudo` for administrative tasks.

* `--cachedir=/chemin/vers/cache`:
    * Specifies the directory for package caching.
    * **Consideration**: Ensure this directory has enough free space.

* `--edit`:
    * Allows editing configuration files during installation.

* `--editor=vim`:
    * Sets Vim as the default text editor for configuration file editing.
    * **Alternative**: `--editor=nano` if you prefer a simpler editor.

* `--shell=fish`:
    * Sets Fish as the default shell for the new user.
    * **Alternatives**: `--shell=zsh` or `--shell=bash` if you prefer those shells.

* `--dry-run`:
    * Performs a test run of the installation without making any actual changes.
    * **Consideration**: Useful for verifying your configuration before committing to the installation.

* `--color=[auto|always|never]`:
    * Controls the use of colors in the console output.

* `--verbose`:
    * Enables verbose output during the installation process.
    * **Consideration**: Can be helpful for troubleshooting but may produce a lot of output.

* `--path="$HOME/bin:$HOME/.local/bin:$PATH"`:
    * Appends additional directories to the `PATH` environment variable.
    * **Consideration**: Useful if you plan to install tools in custom locations.

* `--browser="browser"`:
    * Install the specified browser to navigate on the internet.
    * **Consideration:**  Replace "browser" with the actual browser you want to install (e.g., `firefox`, `chromium`).

* `--ia="model"`:
    * Specify the AI helper to use during the installation.
    * **Consideration:**  This might be specific to the `oh` tool and its documentation should be consulted for available options.

* `--ia-api-key=""`:
    * Specify the API key for the AI helper if required.
    * **Consideration:**  Ensure you have a valid API key if the AI helper needs one.

**Network**

* `--mirrorlist 'URL'`:
    * Specifies the URL of a mirror list to use for package downloads.
    * **Consideration**: You can also use `--reflector=true` to generate a mirrorlist automatically.

* `--reflector=true`:
    * Automatically generates a mirror list based on your location and network speed.
    * **Consideration**:  This is a good option if you don't have a preferred mirrorlist.

* `--wifi-password="password"` & `--wifi-ssid="network name"`:
    * Used to connect to a Wi-Fi network during the installation.

* `--gateway`:
    * Sets the default gateway for network access.
    * **Consideration**:  Required if you're using a static IP address.

* `--dhcp`:
    * Enables automatic network configuration using DHCP.
    * **Consideration**: The default if no static IP is provided.

* `--ip=192.168.1.23`:
    * Sets a static IP address for the system.
    * **Consideration**:  Use this if you need a fixed IP address on your network.

**Packages and Services**

* `--aur-helper="yay"`:
    * Specifies `yay` as the AUR (Arch User Repository) helper for installing packages from the AUR.
    * **Alternatives**:  Other popular AUR helpers include `paru` and `trizen`.

* `--aur="packages"`:
    * Specifies a list of AUR packages to install.
    * **Consideration**:  List the package names separated by spaces.

* `--groups="base-devel"`:
    * Installs a group of packages related to base development tools.
    * **Consideration**:  Other useful groups might include `networkmanager`, `xorg`, etc. Check `pacman -Sg` for a full list.

* `--packages="gnome gnome-extra gdm"`:
    * Installs the GNOME desktop environment, additional GNOME packages, and the GDM display manager.
    * **Alternatives**: You can replace `gnome` with other desktop environments like `kde-plasma`, `xfce4`, etc.

* `--enable-services="NetworkManager sshd gdm"`:
    * Enables the NetworkManager, sshd (SSH server), and gdm (GNOME Display Manager) services at startup.
    * **Consideration**: Adjust the list of services based on your needs.

**Architecture and Boot**

* `--arch=x86_64`:
    * Specifies the target architecture for the installation (64-bit).
    * **Consideration**: Most modern systems use this architecture.

* `--target=efi`:
    * Indicates that you want to install the system on a disk using the UEFI boot mode.
    * **Consideration**:  Ensure your system supports UEFI boot.

**Security and Configuration**

* `--disable-root`:
    * Disables the root account for security reasons.
    * **Consideration**: You'll need to use `sudo` to perform administrative tasks.

* `--ask-password`:
    * Prompts the user to enter passwords during the installation.

* **`--firmware`:**
    * Installs the necessary firmware for certain hardware devices.
    * **Consideration:**  Useful if you have hardware that requires non-free firmware to function properly.

**Mirrors**

* `--mirrorlist="..."`:
    * Uses a specific mirror list (here, those in France using HTTPS).
    * **Consideration**: You can also use `--reflector=true` to generate a mirrorlist automatically.

* `--country="fr"`:
    * Filters the mirror list to keep only those located in France.
    * **Consideration:** Adjust to your country code for faster downloads.

* `--sort-mirror="rate"`:
    * Sorts mirrors based on their download speed.

* `--accept-mirrors="https"`:
    * Limits the selection to mirrors using the HTTPS protocol.
    * **Consideration:** Enhances security during package downloads.

* `--save-mirror-at="/etc/pacman.d/mirrorlist"`:
    * Saves the list of selected mirrors in the `/etc/pacman.d/mirrorlist` file.
    * **Consideration:**  Useful for future updates and installations.

**Scripts and AUR**

* `--script-requirements="wget rsync git"`:
    * Specifies that the `wget`, `rsync`, and `git` commands are required to run the custom installation script.
    * **Consideration**:  Make sure these packages are available during the installation process.

* `--script="/chemin/vers/mon_script.sh"`:
    * Executes the `mon_script.sh` script after the base installation.
    * **Consideration**:  This script can be used for further customization and automation.

* `--aur-helper="yay"`:
    * Uses `yay` as an AUR helper.
    * **Alternatives**: Other popular AUR helpers include `paru` and `trizen`.

**Others**

* `--wish="gnome gnome-extra gdm"`:
    * Expresses the preference to install GNOME and its components, with actions in case of success or failure.

* `--on-wish-failure="i3wm dmenu vim"`:
    * If the GNOME installation fails, installs these packages instead.
    * **Consideration:**  Provides a fallback window manager and basic tools in case the primary desktop environment fails to install

* `--on-wish-success="gnome.sh"`:
    * Executes this script if the GNOME installation is successful.
    * **Consideration:** Useful for post-installation configuration specific to GNOME.

* `--transaction=true`:
    * Enables the transaction system (may require additional configuration).
    * **Consideration:** This can help ensure a clean rollback in case of installation errors, but might require additional setup depending on the tool used

* `--transaction-btrfs=false`:
    * Disables the use of Btrfs snapshots for transactions.

* `--noconfirm`:
    * Does not display confirmation prompts.
    * **Consideration:** Use with caution as it can lead to unintended actions.

* `--needed`:
    * Installs only packages that are not already present.

* `--encrypt=false`:
    * Disables disk encryption.

* `--profile=<profil>`:
    * Selects a specific configuration profile (not specified in this example).

* `--developer-mode=false`:
    * Disables developer mode.

* `--log-level=warn`:
    * Sets the logging level to "warn".

* `--ignore="paquet_a_ignorer"`:
    * Ignores the installation of the specified package.
    * **Consideration:**  Useful if you know a specific package is causing issues or is not needed.

* `--depots="core extra multilib"`:
    * Repositories to be used to download packages.
    * **Consideration:** These are the standard repositories for Arch Linux.

* `--logfile=/chemin/vers/journal.log`:
    * Saves installation logs to a file.
    * **Consideration:** Helpful for troubleshooting any potential issues during or after installation

* `--reboot=true`:
    * Automatically reboots the system after installation.
      Hardware-Specific Options

**Hardware-Specific Options**

* **Graphics**
    * `--gfx-driver="nvidia"`: Installs the proprietary NVIDIA graphics driver.
    * `--gfx-driver="amdgpu"`: Installs the open-source AMDGPU driver for AMD graphics cards.
    * `--gfx-driver="intel"`: Installs the open-source Intel graphics driver.

* **Wi-Fi**
    * `--wifi-driver="broadcom-wl"`: Installs the Broadcom wireless driver (wl).
    * `--wifi-driver="rtl8188eu"`: Installs the Realtek 8188EU wireless driver.

* **Other**
    * `--audio-driver="alsa"`: Installs the ALSA sound driver (usually the default).
    * `--bluetooth=true`: Enables Bluetooth support during installation.
    * `--printer=true`: Installs CUPS (Common Unix Printing System) for printing support.
    * `--virtualbox-guest-modules`: Installs necessary modules for running Arch Linux as a VirtualBox guest.
    * `--vmware-guest-modules`: Installs necessary modules for running Arch Linux as a VMware guest.
      Great, let's brainstorm some more options that could be added to the Arch Linux installation command, considering potential needs beyond the ones already addressed:

**Additional Options**

* **Locale & Input:**
    * `--locale="fr_FR.UTF-8 UTF-8"`: Sets the default locale for the system, influencing date/time, number formatting, etc.
    * `--font="font_name"`: Specifies a default font to be installed for the system.

* **Package Management:**
    * `--no-upgrade`: Skips upgrading packages during the installation process.
    * `--force`: Forces the installation of packages, even if they are already present or there are file conflicts (use with caution).
    * `--asdeps`: Installs packages as dependencies, so they will be removed if no other package depends on them.

* **Bootloader Configuration:**
    * `--bootloader-entries="Arch Linux Windows"`:  Configures the bootloader to display specific entries (replace with your desired OS names).
    * `--bootloader-timeout=5`: Sets the timeout (in seconds) for the bootloader menu.

* **Networking:**
    * `--enable-dhcpcd`: Enables the dhcpcd service for automatic network configuration (alternative to NetworkManager).
    * `--hostname`: Sets the hostname of the machine.

* **Security:**
    * `--encrypt`: Enables full-disk encryption (requires additional configuration and passphrase during boot).
    * `-- luks-name="my_encrypted_volume"`: Sets the name of the encrypted volume if using `--encrypt`.

* **User Management:**
    * `--fullname="John Doe"`: Sets the full name for the created user.

* **Post-installation**
    * `--skip-chroot`: Skips entering the chroot environment after the base installation, useful if you want to customize things further manually.
    * `--no-reboot`: Prevents the system from rebooting after installation.

* **Other**
    * `--quiet`: Suppresses most output during the installation process.
    * `--debug`: Enables debug mode for more verbose output, helpful for troubleshooting.

**Remember these points**

* **Tool Compatibility**: The availability of these options might vary depending on the specific installation tool you're using. Always refer to its documentation for the most accurate information.
* **Careful Consideration**: Some options, like `--encrypt` or `--noconfirm`, require careful consideration and additional configuration. Use them cautiously and ensure you understand their implications.
