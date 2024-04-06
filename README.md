<img src="https://raw.githubusercontent.com/otechdo/arch/main/arch/archlinux.svg" alt="archlinux" align="right" width="250">

A archlinux installer, manager for advanced arch users.

- [@documentation](https://github.com/otechdo/arch/blob/main/arch/docs/)
    - [@en](https://raw.githubusercontent.com/otechdo/arch/main/README.md)
    - [@fr](https://github.com/otechdo/arch/blob/main/arch/docs/fr/README.md)
    - [@es](https://github.com/otechdo/arch/blob/main/arch/docs/es/README.md)
    - [@it](https://github.com/otechdo/arch/blob/main/arch/docs/it/README.md)
- [@archlinux](https://archlinux.org)
    - [@guide](https://wiki.archlinux.org/title/Installation_guide)
    - [@wiki](https://wiki.archlinux.org/)
    - [@download](https://archlinux.org/download/)
- [@arch](https://github.com/otechdo/arch/)
    - [@issues](https://github.com/otechdo/arch/issues)
    - [@discussions](https://discord.gg/jWHjkpRJPw)

##  Set desired keymap

```bash
loadkeys <keymap>
```

##  Create three partitions:

* 1 1024MB  EFI partition   # ef00
* 2 4096MB  Linux partition # 8300
* 3 100%    Linux partition # 8300

```bash
cgdisk /dev/sda
```

## Formatting

### /boot/efi

```bash
mkfs.vfat -F 32 /dev/sda1
```

### /boot

```bash
mkfs.ext2 /dev/sda2
```

### /

```bash
mkfs.ext4 /dev/sda3
```

## List device block for mounting

```bash
lsblk --fs
```

## Mount partitions

```bash
mount /dev/sda3 /mnt
mkdir /mnt/boot
mount /dev/sda2 /mnt/boot
mkdir /mnt/boot/efi
mount /dev/sda1 /mnt/boot/efi
```

## Check device block afer mounting

```bash
lsblk --fs --paths
```

## Change pacman mirror priority

```bash
reflector -c <country> --sort delay --save /etc/pacman.d/mirrorlist -p https
```

## Update package signing keys

```bash
pacman-key --init && pacman-key --populate archlinux
```

## Refresh package signing keys

```bash
pacman-key --refresh-keys
```

## Install the base system

```bash
pacstrap /mnt base base-devel wget git linux linux-firmware vim git efibootmgr rustup sudo grub networkmanager w3m archiso reflector <shell> <ucode> <graphics_driver>
```

##  Generate fstab

```bash
genfstab -U /mnt >> /mnt/etc/fstab
```

##  Enter inside the new system

```bash
arch-chroot /mnt && cd ~
```

##  Sync clock

```bash
timedatectl set-ntp true
```

## Create your account

### Create user

```bash
useradd -m -U -c 'REAL NAME' -s <shell> <username>
```

### Create root password

```bash
passwd root
```

### Create user password

```bash
passwd <username>
```

### Add your account to sudoers 

```bash
echo '<username> ALL=(ALL) ALL' > /etc/sudoers.d/<username>
```

## Checkout on your account

```bash
su - <username>
```

## Configure the toolchain

```bash
rustup default stable
```

## Enabled multilib repository

```bash
vim /etc/pacman.conf
```

## Refresh pacman database 

```bash
sudo pacman -Sy
```

## Refresh pacman database 

```bash
sudo pacman -Sy
```

## Install yay

```bash
git clone https://aur.archlinux.org/yay && cd yay && makepkg -si && cd .. && rm -rf yay
```

## Install arch
 
### From GitHub

```bash
git clone https://github.com/otechdo/arch && cd arch && cargo build --release && install -m 755 target/release/arch /usr/bin/arch
```

### From Crates.io

```bash
cargo install arch && install -m 755 "$HOME/.cargo/bin/arch" /usr/bin/arch 
```

### From Aur

```bash
paru -Syu manager
```

## Setup the new arch

```bash
arch setup
```

## Desktop 

- [@deepin](https://wiki.archlinux.org/title/Deepin_Desktop_Environment)
- [@kde](https://wiki.archlinux.org/title/KDE)
- [@gnome](https://wiki.archlinux.org/title/GNOME)

## Install all selected packages on arch

```bash
arch --install-packages
```

## Install all selected packages as deps on arch

```bash
arch --install-dependencies
```

## Remove all selected packages on arch

```bash
arch --remove-packages
```

## Update all mirrors to the enter country

```bash
arch --update-mirrors
```

## Check updates

```bash
arch --check-updates
```

## Update arch

```bash
arch --update
```

## Update and reboot arch

```bash
arch --update-and-reboot
arch --update -r
arch -r --update
```

## Cancel the reboot task

```bash
arch --cancel-reboot
```

## Refrech cache

```bash
sudo arch --refresh-cache
```



## Download updates

```bash
arch --download-updates
```

## Exit chroot

```bash
exit
```

## Umount partitions

```bash
umount -R /mnt
```

## Restart on the new system

```bash
reboot
```

## Key Bindings

This file lists all of the key bindings currently registered by prompts.

## All prompts

These key bindings may be used with all prompts.

| **command**                      | **description**         |
|----------------------------------|-------------------------|
| <kbd>enter</kbd>                 | Submit answer.          |
| <kbd>esc</kbd>                   | Cancel the prompt\*.    |
| <kbd>ctrl</kbd>  +  <kbd>c</kbd> | Interrupt the prompt\*. |

\* Canceling and interrupting a prompt have two different meanings. Canceling is defined specially for when the end user is allowed to skip a prompt, the library user can then use `prompt_skippable` which wraps the return type into an `Option` and catches the `CanceledOperation` error transforming it into a `Ok(None)` result. Interrupted operations are closer to "stop-the-world" operations, where the library user should treat them as termination commands.

## Text Input

These key bindings may be used with all prompts that ask the user for text input: [`Text`], [`Select`], [`MultiSelect`], [`Confirm`], [`CustomType`] and [`Password`]. The [`Editor`] prompt is not included because it opens a separate text editor for text input.


| **command**                          | **description**                                 |
|--------------------------------------|-------------------------------------------------|
| <kbd>character</kbd>                 | Insert the character into the input.            |
| <kbd>left</kbd>                      | Move the cursor back one character.             |
| <kbd>right</kbd>                     | Move the cursor forward one character.          |
| <kbd>ctrl</kbd> + <kbd>left</kbd>    | Move one word to the left of the cursor.        |
| <kbd>ctrl</kbd> + <kbd>right</kbd>   | Move one word to the right of the cursor.       |
| <kbd>home</kbd>                      | Move cursor to the start of the line*.          |
| <kbd>end</kbd>                       | Move cursor to the end of the line*.            |
| <kbd>backspace</kbd>                 | Delete one character to the left of the cursor. |
| <kbd>delete</kbd>                    | Delete the character at the cursor.             |
| <kbd>ctrl</kbd> + <kbd>delete</kbd>  | Delete one word to the right of the cursor.     |

\* Key bindings not supported on [`Select`] and [`MultiSelect`] prompts.

## Text Prompts

These key bindings may be used in [`Text`] prompts.

| **command**          | **description**                                               |
|----------------------|---------------------------------------------------------------|
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
|----------------------|---------------------------------------------------------------|
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
|----------------------|---------------------------------------------------------------|
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
|------------------------------------------|---------------------------------------------------------------|
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
|------------------|----------------------------------------------------------------|
| <kbd>e</kbd>     | Open the editor.                                               |
| <kbd>enter</kbd> | Submit the current content of the temporary file being edited. |


[`Text`]: https://docs.rs/inquire/*/inquire/prompts/text/struct.Text.html
[`DateSelect`]: https://docs.rs/inquire/*/inquire/prompts/dateselect/struct.DateSelect.html
[`Select`]: https://docs.rs/inquire/*/inquire/prompts/select/struct.Select.html
[`MultiSelect`]: https://docs.rs/inquire/*/inquire/prompts/multiselect/struct.MultiSelect.html
[`Confirm`]: https://docs.rs/inquire/*/inquire/prompts/confirm/struct.Confirm.html
[`Editor`]: https://docs.rs/inquire/*/inquire/prompts/editor/struct.Editor.html
[`customtype`]: https://docs.rs/inquire/*/inquire/struct.CustomType.html
[`Password`]: https://docs.rs/inquire/*/inquire/prompts/password/struct.Password.html
