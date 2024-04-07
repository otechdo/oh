- [PDF](https://raw.githubusercontent.com/otechdo/arch/main/arch.pdf) 

- [EPUB](https://raw.githubusercontent.com/otechdo/arch/main/arch.epub)

> sha512sum arch.pdf

`269036e9752ef3edb89d8ec0b351bc745d25499b45a0c3f9038ed97b8c6b3caa9f9b7af4d08ef71cab74148b67199e73d221f5dffa6ed6aab50ca0010940f703 arch.pdf`

> sha512sum arch.epub

`8ddf24144aeb1abfe16d3500ccefa72ab46c9a513dbdc76c3b34d387bc76d66640d54f77a5ce26facde6012110ef9fb257bed04ec87edf087416acfceb9bcb46 arch.epub`

# Set keymap

```bash
loadkeys <keymap>
```

## Partitioning

1. 1024MB  EFI partition         # ef00      /boot/efi    
2. 4096MB  Linux partition    # 8300     /boot
3. 100%       Linux partition    # 8300     /

```bash
cgdisk /dev/sda
```

## Formatting

> /boot/efi

```bash
mkfs.vfat -F 32 /dev/sda1
```

> /boot

```bash
mkfs.ext2 /dev/sda2
```

> /

```bash
mkfs.ext4 /dev/sda3
```

## Mounting

### The root partition

```bash
mount /dev/sda3 /mnt
```

### Create the boot mount point

```bash
mkdir /mnt/boot
```

### Mounting the boot partition

```bash
mount /dev/sda2 /mnt/boot
```

### Create the EFI mount point

```bash
mkdir /mnt/boot/efi
```

### Mounting the EFI partition

```bash
mount /dev/sda1 /mnt/boot/efi
```

## Update mirrorlist

```bash
reflector -c <country> --sort delay --save /etc/pacman.d/mirrorlist -p https
```

## Init pacman

```bash
pacman-key --init && pacman-key --populate archlinux
```

# Installation

## The base system

```bash
pacstrap /mnt base base-devel wget git linux linux-firmware vim efibootmgr rustup sudo grub networkmanager w3m archiso reflector <shell> <ucode> <graphics_driver>
```

# Generate fstab

```bash
genfstab -U /mnt >> /mnt/etc/fstab
```

# Enter in the new system

```bash
arch-chroot /mnt && cd ~
```

# Manage accounts

## Create your account

```bash
useradd -m -U -c 'YOUR REAL NAME' -s <shell> <username>
```

## Generate root password

```bash
passwd root
```

### Generate your password

```bash
passwd <username>
```

### Add your account to sudoers file

```bash
echo '<username> ALL=(ALL) ALL' > /etc/sudoers.d/<username>
```

# Sign in

```bash
su - <username>
```

# Configure rust

```bash
rustup default stable
```

# Modify pacman.conf

```bash
sudo vim /etc/pacman.conf
```

# Refresh repositories

```bash
sudo pacman -Sy
```

# Installation of yay

```bash
git clone https://aur.archlinux.org/yay 
cd yay
makepkg -si 
cd ..
rm -rf yay
```

# Install arch

## From GitHub

```bash
git clone https://github.com/otechdo/arch 
cd arch 
make 
sudo make install
```

### From Crates.io

```bash
cargo install arch 
install -m 755 "$HOME/.cargo/bin/arch" /usr/bin/arch
```

### From Aur

```bash
paru -Syu manager
```

## Setup a new arch

```bash
arch --setup
```

## Desktop

- [@deepin](https://wiki.archlinux.org/title/Deepin_Desktop_Environment)
- [@kde](https://wiki.archlinux.org/title/KDE)
- [@gnome](https://wiki.archlinux.org/title/GNOME)
- [@xmonad](https://wiki.archlinux.org/title/xmonad)
- [@i3](https://wiki.archlinux.org/title/i3)

## Install all selected packages on arch

```bash
arch --install
arch -S <pkg> <pkg>
```

# Quit the fresh new system

```bash
exit
```

# Umount all mounted partitions

```bash
umount -R /mnt
```

# Reboot

```bash
reboot
```

# Arch commands

## Setup a new arch

```bash
arch -i
```

```bash
arch --setup
```

## Remove packages

```bash
arch -R <pkg> <pkg>
```

```bash
arch --uninstall
```

## Install new packages

```bash
arch -S <pkg> <pkg>
```

```bash
arch --install
```

## Update mirrorlist

```bash
arch -M
```

```bash
arch --mirrors
```

## Check updates

```bash
arch -C
```

```bash
arch --check
```

## Install packages as dependencies

```bash
arch -d
```

```bash
arch --deps
```

# Update archlinux

```bash
arch
```

```bash
arch -u
```

```bash
arch --update
```

## Search a package

```bash
arch -s <pkg>
```

```bash
arch --search <pkg>
```

## Show arch current version

```bash
arch -v
```

```bash
arch --version
```

## Download updates

```bash
arch -d
```

```bash
arch --download-updates
```

## Show help message

```bash
arch -h
```

```bash
arch --help
```

## Cancel the upgrade reboot

```bash
arch -x
```

```bash
arch --cancel
```

## Upgrade the system and reboot

```bash
arch -U
```

```bash
arch --upgrade
```

## Generate arch packages cache

```bash
arch -c
```

```bash
arch --cache
```

## Navigate on news

```bash
arch -n
```

```bash
arch --news
```

## Navigate on the Aur

```bash
arch -a
```

```bash
arch --aur
```

### Navigate on the forum

```bash
arch -f
```

```bash
arch --forum
```

### Navigate on the man pages

```bash
arch -m
```

```bash
arch --man
```

```bash
arch --woman
```

### Navigate on the wiki

```bash
arch -w
```

```bash
arch --wiki
```

# Toolbox support

[GitHub - toolbx-images/images: Community maintained container images to use with toolbx and distrobox](https://github.com/toolbx-images/images?tab=readme-ov-file)

## List all toolbox

```bash
os --list
```

## Create a new toolbox

```bash
os --add fedora 39 workstation
os --add fedora 39
```

## Create a new toolbox from an image

```bash
os --add-from quay.io/toolbx-images/debian-toolbox:12
```

## Enter in toolbox

```bash
os --use workstation
os --use fedora-toolbox-39
```

## Run a command in toolbox

```bash
os --run workstation ls
os --run fedora-toolbox-39 ls
```

## Stop a toolbox

```bash
os --stop workstation
os --stop fedora-toolbox-39
```

## Remove a toolbox

```bash
os --rm workstation
os --rm fedora-toolbox-39
```

## Key Bindings

This file lists all of the key bindings currently registered by prompts.

## All prompts

These key bindings may be used with all prompts.

| **command**                      | **description**         |
| -------------------------------- | ----------------------- |
| <kbd>enter</kbd>                 | Submit answer.          |
| <kbd>esc</kbd>                   | Cancel the prompt\*.    |
| <kbd>ctrl</kbd>  +  <kbd>c</kbd> | Interrupt the prompt\*. |

\* Canceling and interrupting a prompt have two different meanings. Canceling is defined specially for when the end user is allowed to skip a prompt, the library user can then use `prompt_skippable` which wraps the return type into an `Option` and catches the `CanceledOperation` error transforming it into a `Ok(None)` result. Interrupted operations are closer to "stop-the-world" operations, where the library user should treat them as termination commands.

## Text Input

These key bindings may be used with all prompts that ask the user for text input: [`Text`], [`Select`], [`MultiSelect`], [`Confirm`], [`CustomType`] and [`Password`]. The [`Editor`] prompt is not included because it opens a separate text editor for text input.

| **command**                         | **description**                                 |
| ----------------------------------- | ----------------------------------------------- |
| <kbd>character</kbd>                | Insert the character into the input.            |
| <kbd>left</kbd>                     | Move the cursor back one character.             |
| <kbd>right</kbd>                    | Move the cursor forward one character.          |
| <kbd>ctrl</kbd> + <kbd>left</kbd>   | Move one word to the left of the cursor.        |
| <kbd>ctrl</kbd> + <kbd>right</kbd>  | Move one word to the right of the cursor.       |
| <kbd>home</kbd>                     | Move cursor to the start of the line*.          |
| <kbd>end</kbd>                      | Move cursor to the end of the line*.            |
| <kbd>backspace</kbd>                | Delete one character to the left of the cursor. |
| <kbd>delete</kbd>                   | Delete the character at the cursor.             |
| <kbd>ctrl</kbd> + <kbd>delete</kbd> | Delete one word to the right of the cursor.     |

\* Key bindings not supported on [`Select`] and [`MultiSelect`] prompts.

## Text Prompts

These key bindings may be used in [`Text`] prompts.

| **command**          | **description**                                               |
| -------------------- | ------------------------------------------------------------- |
| <kbd>enter</kbd>     | Submit the current current text input.                        |
| <kbd>up</kbd>        | When suggestions are displayed, move cursor one row up.       |
| <kbd>down</kbd>      | When suggestions are displayed, move cursor one row down.     |
| <kbd>page up</kbd>   | When suggestions are displayed, move cursor one page up.      |
| <kbd>page down</kbd> | When suggestions are displayed, move cursor one page down.    |
| <kbd>tab</kbd>       | Replace current input with the resulting suggestion if any.   |
| others               | See [Text Input](#text-input) and [All Prompts](#all-prompts) |

## Select Prompts

These key bindings may be used in [`Select`] prompts.

| **command**          | **description**                                               |
| -------------------- | ------------------------------------------------------------- |
| <kbd>enter</kbd>     | Submit the current highlighted option.                        |
| <kbd>up</kbd>        | Move cursor one row up.                                       |
| <kbd>down</kbd>      | Move cursor one row down.                                     |
| <kbd>k</kbd>         | Move cursor one row up when vim mode is enabled.              |
| <kbd>j</kbd>         | Move cursor one row down when vim mode is enabled.            |
| <kbd>page up</kbd>   | Move cursor one page up.                                      |
| <kbd>page down</kbd> | Move cursor one page down.                                    |
| <kbd>home</kbd>      | Move cursor to the first option.                              |
| <kbd>end</kbd>       | Move cursor to the last option.                               |
| others               | See [Text Input](#text-input) and [All Prompts](#all-prompts) |

## MultiSelect Prompts

These key bindings may be used in [`MultiSelect`] prompts.

| **command**          | **description**                                               |
| -------------------- | ------------------------------------------------------------- |
| <kbd>enter</kbd>     | Submit the options currently selected.                        |
| <kbd>space</kbd>     | Toggle the selection of the current highlighted option.       |
| <kbd>up</kbd>        | Move cursor one row up.                                       |
| <kbd>down</kbd>      | Move cursor one row down.                                     |
| <kbd>k</kbd>         | Move cursor one row up when vim mode is enabled.              |
| <kbd>j</kbd>         | Move cursor one row down when vim mode is enabled.            |
| <kbd>page up</kbd>   | Move cursor one page up.                                      |
| <kbd>page down</kbd> | Move cursor one page down.                                    |
| <kbd>home</kbd>      | Move cursor to the first option.                              |
| <kbd>end</kbd>       | Move cursor to the last option.                               |
| <kbd>left</kbd>      | Unselect all options.                                         |
| <kbd>right</kbd>     | Select all options.                                           |
| others               | See [Text Input](#text-input) and [All Prompts](#all-prompts) |

## DateSelect Prompts

These key bindings may be used in the interactive calendar of the [`DateSelect`] prompt.

| **command**                              | **description**                                               |
| ---------------------------------------- | ------------------------------------------------------------- |
| <kbd>space bar</kbd> or <kbd>enter</kbd> | Submit the current highlighted date.                          |
| <kbd>up</kbd>                            | Move cursor one row up.                                       |
| <kbd>down</kbd>                          | Move cursor one row down.                                     |
| <kbd>left</kbd>                          | Move cursor one column to the left.                           |
| <kbd>right</kbd>                         | Move cursor one column to the right.                          |
| <kbd>k</kbd>                             | Move cursor one row up when vim mode is enabled.              |
| <kbd>j</kbd>                             | Move cursor one row down when vim mode is enabled.            |
| <kbd>h</kbd>                             | Move cursor one column to the left when vim mode is enabled.  |
| <kbd>l</kbd>                             | Move cursor one column to the right when vim mode is enabled. |
| <kbd>ctrl</kbd> + <kbd>up</kbd>          | Move calendar back by one year.                               |
| <kbd>ctrl</kbd> + <kbd>down</kbd>        | Move calendar forward by one year.                            |
| <kbd>ctrl</kbd> + <kbd>left</kbd>        | Move calendar back by one month.                              |
| <kbd>ctrl</kbd> + <kbd>right</kbd>       | Move calendar forward by one month.                           |

## Editor Prompts

These key bindings may be used in [`Editor`] prompts.

| **command**      | **description**                                                |
| ---------------- | -------------------------------------------------------------- |
| <kbd>e</kbd>     | Open the editor.                                               |
| <kbd>enter</kbd> | Submit the current content of the temporary file being edited. |
